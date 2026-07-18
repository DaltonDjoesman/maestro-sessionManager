//! Short-lived Wayland foreign-toplevel snapshot for window-anchored discovery.
//! Soft-fails (empty list) when the compositor denies the protocol or times out.
//! Cosmic workspace enrichment uses `zcosmic_toplevel_info_v1` + `ext_workspace_manager_v1`
//! when advertised; soft-fails leave `desktop` unset (never fake index `0`).

use std::collections::HashMap;
use std::time::{Duration, Instant};

use cosmic_protocols::toplevel_info::v1::client::{
    zcosmic_toplevel_handle_v1, zcosmic_toplevel_info_v1,
};
use wayland_client::{
    protocol::wl_registry, Connection, Dispatch, Proxy, QueueHandle,
};
use wayland_protocols::ext::foreign_toplevel_list::v1::client::{
    ext_foreign_toplevel_handle_v1, ext_foreign_toplevel_list_v1,
};
use wayland_protocols::ext::workspace::v1::client::{
    ext_workspace_group_handle_v1, ext_workspace_handle_v1, ext_workspace_manager_v1,
};

use super::workspace::WindowRecord;

const SNAPSHOT_TIMEOUT: Duration = Duration::from_millis(500);
/// Max wait for Cosmic `ext_workspace_enter` after foreign-toplevel listing.
/// Kept short: stragglers that never enter (Cosmic quirk) must not block Capture.
const WORKSPACE_ENRICH_TIMEOUT: Duration = Duration::from_millis(700);
/// After Cosmic signals done, wait this long for late enters before giving up.
const WORKSPACE_ENTER_GRACE: Duration = Duration::from_millis(80);

/// Snapshot mapped top-level windows via `ext-foreign-toplevel-list-v1`.
/// Returns an empty vec on any failure (no Wayland display, bind denied, timeout).
pub fn load_wayland_foreign_toplevel() -> Vec<WindowRecord> {
    match snapshot_foreign_toplevels() {
        Ok(records) => records,
        Err(err) => {
            eprintln!("[maestro] wayland foreign-toplevel soft-fail: {err}");
            Vec::new()
        }
    }
}

/// Soft-fail Cosmic workspace attachment for already-collected records.
///
/// Build a stable 0-based index map from workspace handle protocol IDs in announcement order.
/// First-seen handle → `0`, next new handle → `1`, …. Duplicate IDs keep their first index.
pub fn workspace_handle_index_map(handle_ids_in_order: &[u32]) -> HashMap<u32, u32> {
    let mut map = HashMap::new();
    for &id in handle_ids_in_order {
        let next = map.len() as u32;
        map.entry(id).or_insert(next);
    }
    map
}

/// Resolve a 0-based workspace index for a toplevel that entered one or more workspaces.
/// Prefers the lowest index when the toplevel is on multiple workspaces (sticky).
pub fn resolve_workspace_index(map: &HashMap<u32, u32>, entered_handle_ids: &[u32]) -> Option<u32> {
    entered_handle_ids
        .iter()
        .filter_map(|id| map.get(id).copied())
        .min()
}

fn snapshot_foreign_toplevels() -> Result<Vec<WindowRecord>, String> {
    if std::env::var_os("WAYLAND_DISPLAY").is_none() {
        return Err("WAYLAND_DISPLAY unset".into());
    }

    let conn = Connection::connect_to_env().map_err(|e| format!("connect: {e}"))?;
    let display = conn.display();
    let mut event_queue = conn.new_event_queue();
    let qh = event_queue.handle();
    let _registry = display.get_registry(&qh, ());

    let mut state = SnapshotState::default();
    let deadline = Instant::now() + SNAPSHOT_TIMEOUT;

    event_queue
        .roundtrip(&mut state)
        .map_err(|e| format!("registry roundtrip: {e}"))?;

    if state.list.is_none() {
        return Err("ext_foreign_toplevel_list_v1 global not bound".into());
    }

    while Instant::now() < deadline {
        event_queue
            .roundtrip(&mut state)
            .map_err(|e| format!("list roundtrip: {e}"))?;
        if state.handles.values().any(|h| h.done) && !state.handles.is_empty() {
            // One more pass so late title/app_id events land.
            let _ = event_queue.roundtrip(&mut state);
            break;
        }
    }

    let enrich_deadline = Instant::now() + WORKSPACE_ENRICH_TIMEOUT;
    if let Err(err) = enrich_cosmic_workspaces(&mut event_queue, &mut state, enrich_deadline) {
        eprintln!("[maestro] cosmic workspace soft-fail: {err}");
    }

    let records: Vec<WindowRecord> = state
        .handles
        .values()
        .filter_map(handle_info_to_record)
        .collect();
    Ok(records)
}

/// Bind Cosmic toplevel-info + ext-workspace, associate via `get_cosmic_toplevel`, set
/// `HandleInfo.desktop`. Soft-fails with `Err` when globals are missing or time out.
fn enrich_cosmic_workspaces(
    event_queue: &mut wayland_client::EventQueue<SnapshotState>,
    state: &mut SnapshotState,
    deadline: Instant,
) -> Result<(), String> {
    let (info, version) = state
        .cosmic_info
        .clone()
        .ok_or_else(|| "zcosmic_toplevel_info_v1 not advertised".to_string())?;

    if state.ext_workspace_mgr.is_none() {
        return Err("ext_workspace_manager_v1 not advertised".into());
    }

    // Collect workspace announcements until Done (or deadline). Breaking as soon
    // as the first handle appears can omit later workspaces (e.g. workspace 4),
    // so ExtWorkspaceEnter IDs fail to resolve and those windows stay desktop=None.
    while Instant::now() < deadline && !state.workspace_done {
        event_queue
            .roundtrip(state)
            .map_err(|e| format!("workspace roundtrip: {e}"))?;
    }
    if !state.workspace_order.is_empty() {
        let _ = event_queue.roundtrip(state);
    }

    state.workspace_index = workspace_handle_index_map(&state.workspace_order);
    if state.workspace_index.is_empty() {
        return Err("no ext workspaces announced".into());
    }

    if version < 2 {
        return Err("zcosmic_toplevel_info_v1 version < 2 (need get_cosmic_toplevel)".into());
    }

    let qh = event_queue.handle();
    let foreign_proxies: Vec<_> = state._proxies.clone();
    for foreign in &foreign_proxies {
        let foreign_id = foreign.id().protocol_id();
        let cosmic = info.get_cosmic_toplevel(foreign, &qh, foreign_id);
        state._cosmic_proxies.push(cosmic);
        state.cosmic_workspaces.entry(foreign_id).or_default();
    }

    // Wait for workspace enters, but stop early once Cosmic is done and no new
    // enters arrive for WORKSPACE_ENTER_GRACE. Waiting for 100% assignment made
    // Capture hang for the full enrich timeout whenever one window never enters.
    let expected = foreign_proxies.len().max(1);
    let mut last_assigned = 0usize;
    let mut last_progress = Instant::now();
    while Instant::now() < deadline {
        event_queue
            .roundtrip(state)
            .map_err(|e| format!("cosmic toplevel roundtrip: {e}"))?;
        let assigned = state
            .cosmic_workspaces
            .values()
            .filter(|ids| !ids.is_empty())
            .count();
        if assigned > last_assigned {
            last_assigned = assigned;
            last_progress = Instant::now();
        }
        if assigned >= expected {
            let _ = event_queue.roundtrip(state);
            break;
        }
        if state.cosmic_done && last_progress.elapsed() >= WORKSPACE_ENTER_GRACE {
            let _ = event_queue.roundtrip(state);
            break;
        }
    }

    for (foreign_id, ws_ids) in &state.cosmic_workspaces {
        if let Some(info) = state.handles.get_mut(foreign_id) {
            info.desktop = resolve_workspace_index(&state.workspace_index, ws_ids);
        }
    }

    if std::env::var_os("MAESTRO_DEBUG_WORKSPACES").is_some() {
        let remaining_ms = deadline
            .saturating_duration_since(Instant::now())
            .as_millis();
        eprintln!(
            "[maestro:ws] workspace_done={} cosmic_done={} announced={} map_len={} remaining_ms={}",
            state.workspace_done,
            state.cosmic_done,
            state.workspace_order.len(),
            state.workspace_index.len(),
            remaining_ms
        );
        eprintln!(
            "[maestro:ws] workspace_order={:?} index={:?}",
            state.workspace_order, state.workspace_index
        );
        for id in &state.workspace_order {
            let meta = state.workspace_meta.get(id);
            eprintln!(
                "[maestro:ws] ws_handle={} idx={:?} name={:?} coords={:?} active={}",
                id,
                state.workspace_index.get(id).copied(),
                meta.and_then(|m| m.name.clone()),
                meta.map(|m| m.coordinates.clone()).unwrap_or_default(),
                meta.map(|m| m.active).unwrap_or(false)
            );
        }
        let mut rows: Vec<_> = state.handles.iter().collect();
        rows.sort_by_key(|(id, _)| *id);
        for (foreign_id, info) in rows {
            let ws_ids = state
                .cosmic_workspaces
                .get(foreign_id)
                .cloned()
                .unwrap_or_default();
            let known: Vec<_> = ws_ids
                .iter()
                .map(|id| {
                    format!(
                        "{}→{:?}",
                        id,
                        state.workspace_index.get(id).copied()
                    )
                })
                .collect();
            eprintln!(
                "[maestro:ws] foreign={} desktop={:?} app_id={:?} title={:?} enters={:?} state={:?} resolve={}",
                foreign_id,
                info.desktop,
                info.app_id,
                info.title,
                known,
                state.cosmic_states.get(foreign_id).cloned().unwrap_or_default(),
                if ws_ids.is_empty() {
                    "NO_ENTER"
                } else if info.desktop.is_some() {
                    "OK"
                } else {
                    "UNRESOLVED_ID"
                }
            );
        }
    }

    Ok(())
}

fn handle_info_to_record(info: &HandleInfo) -> Option<WindowRecord> {
    let title = info
        .title
        .as_deref()
        .filter(|t| !t.trim().is_empty())
        .or(info.app_id.as_deref())
        .filter(|t| !t.trim().is_empty())?
        .to_string();
    Some(WindowRecord {
        // Foreign-toplevel does not expose PID; title/app_id matching covers workers.
        pid: 0,
        desktop: info.desktop,
        title,
        app_id: info.app_id.clone(),
    })
}

#[derive(Default)]
struct SnapshotState {
    list: Option<ext_foreign_toplevel_list_v1::ExtForeignToplevelListV1>,
    cosmic_info: Option<(zcosmic_toplevel_info_v1::ZcosmicToplevelInfoV1, u32)>,
    ext_workspace_mgr: Option<ext_workspace_manager_v1::ExtWorkspaceManagerV1>,
    handles: HashMap<u32, HandleInfo>,
    /// Keep foreign-toplevel proxies alive for the snapshot duration.
    _proxies: Vec<ext_foreign_toplevel_handle_v1::ExtForeignToplevelHandleV1>,
    _cosmic_proxies: Vec<zcosmic_toplevel_handle_v1::ZcosmicToplevelHandleV1>,
    _ext_ws_groups: Vec<ext_workspace_group_handle_v1::ExtWorkspaceGroupHandleV1>,
    _ext_ws_handles: Vec<ext_workspace_handle_v1::ExtWorkspaceHandleV1>,
    /// foreign protocol_id → Cosmic workspace handle protocol IDs entered.
    cosmic_workspaces: HashMap<u32, Vec<u32>>,
    workspace_order: Vec<u32>,
    workspace_index: HashMap<u32, u32>,
    /// ext workspace protocol_id → human name / coordinates / active bit (debug + future ordering).
    workspace_meta: HashMap<u32, WorkspaceMeta>,
    workspace_done: bool,
    cosmic_done: bool,
    /// foreign protocol_id → last cosmic toplevel state words (debug).
    cosmic_states: HashMap<u32, Vec<String>>,
}

#[derive(Default, Clone, Debug)]
struct WorkspaceMeta {
    name: Option<String>,
    coordinates: Vec<u32>,
    active: bool,
}

#[derive(Default, Clone)]
struct HandleInfo {
    title: Option<String>,
    app_id: Option<String>,
    desktop: Option<u32>,
    done: bool,
}

impl Dispatch<wl_registry::WlRegistry, ()> for SnapshotState {
    fn event(
        state: &mut Self,
        registry: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _: &(),
        _: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        if let wl_registry::Event::Global {
            name,
            interface,
            version,
        } = event
        {
            match interface.as_str() {
                "ext_foreign_toplevel_list_v1" => {
                    let list = registry
                        .bind::<ext_foreign_toplevel_list_v1::ExtForeignToplevelListV1, _, _>(
                            name,
                            version.min(1),
                            qh,
                            (),
                        );
                    state.list = Some(list);
                }
                "zcosmic_toplevel_info_v1" => {
                    let ver = version.min(3);
                    let info = registry
                        .bind::<zcosmic_toplevel_info_v1::ZcosmicToplevelInfoV1, _, _>(
                            name, ver, qh, (),
                        );
                    state.cosmic_info = Some((info, ver));
                }
                "ext_workspace_manager_v1" => {
                    let mgr = registry
                        .bind::<ext_workspace_manager_v1::ExtWorkspaceManagerV1, _, _>(
                            name,
                            version.min(1),
                            qh,
                            (),
                        );
                    state.ext_workspace_mgr = Some(mgr);
                }
                _ => {}
            }
        }
    }
}

impl Dispatch<ext_foreign_toplevel_list_v1::ExtForeignToplevelListV1, ()> for SnapshotState {
    fn event(
        state: &mut Self,
        _: &ext_foreign_toplevel_list_v1::ExtForeignToplevelListV1,
        event: ext_foreign_toplevel_list_v1::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        match event {
            ext_foreign_toplevel_list_v1::Event::Toplevel { toplevel } => {
                let id = toplevel.id().protocol_id();
                state.handles.insert(id, HandleInfo::default());
                state._proxies.push(toplevel);
            }
            ext_foreign_toplevel_list_v1::Event::Finished => {}
            _ => {}
        }
    }

    wayland_client::event_created_child!(SnapshotState, ext_foreign_toplevel_list_v1::ExtForeignToplevelListV1, [
        ext_foreign_toplevel_list_v1::EVT_TOPLEVEL_OPCODE => (ext_foreign_toplevel_handle_v1::ExtForeignToplevelHandleV1, ())
    ]);
}

impl Dispatch<ext_foreign_toplevel_handle_v1::ExtForeignToplevelHandleV1, ()> for SnapshotState {
    fn event(
        state: &mut Self,
        handle: &ext_foreign_toplevel_handle_v1::ExtForeignToplevelHandleV1,
        event: ext_foreign_toplevel_handle_v1::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        let id = handle.id().protocol_id();
        let info = state.handles.entry(id).or_default();
        match event {
            ext_foreign_toplevel_handle_v1::Event::Title { title } => {
                info.title = Some(title);
            }
            ext_foreign_toplevel_handle_v1::Event::AppId { app_id } => {
                info.app_id = Some(app_id);
            }
            ext_foreign_toplevel_handle_v1::Event::Done => {
                info.done = true;
            }
            ext_foreign_toplevel_handle_v1::Event::Closed => {
                state.handles.remove(&id);
            }
            ext_foreign_toplevel_handle_v1::Event::Identifier { .. } => {}
            _ => {}
        }
    }
}

impl Dispatch<zcosmic_toplevel_info_v1::ZcosmicToplevelInfoV1, ()> for SnapshotState {
    fn event(
        state: &mut Self,
        _: &zcosmic_toplevel_info_v1::ZcosmicToplevelInfoV1,
        event: zcosmic_toplevel_info_v1::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        match event {
            zcosmic_toplevel_info_v1::Event::Done => {
                state.cosmic_done = true;
            }
            zcosmic_toplevel_info_v1::Event::Toplevel { toplevel } => {
                // Version 1 path: keep proxy alive; matching falls back to title/app_id
                // via events on the handle (no foreign association). Soft-skip for v1.
                state._cosmic_proxies.push(toplevel);
            }
            zcosmic_toplevel_info_v1::Event::Finished => {}
            _ => {}
        }
    }

    wayland_client::event_created_child!(SnapshotState, zcosmic_toplevel_info_v1::ZcosmicToplevelInfoV1, [
        zcosmic_toplevel_info_v1::EVT_TOPLEVEL_OPCODE => (zcosmic_toplevel_handle_v1::ZcosmicToplevelHandleV1, 0u32)
    ]);
}

impl Dispatch<zcosmic_toplevel_handle_v1::ZcosmicToplevelHandleV1, u32> for SnapshotState {
    fn event(
        state: &mut Self,
        _: &zcosmic_toplevel_handle_v1::ZcosmicToplevelHandleV1,
        event: zcosmic_toplevel_handle_v1::Event,
        foreign_id: &u32,
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        match event {
            zcosmic_toplevel_handle_v1::Event::ExtWorkspaceEnter { workspace } => {
                let ws_id = workspace.id().protocol_id();
                if std::env::var_os("MAESTRO_DEBUG_WORKSPACES").is_some() {
                    eprintln!(
                        "[maestro:ws] ExtWorkspaceEnter foreign={} ws_id={}",
                        foreign_id, ws_id
                    );
                }
                state
                    .cosmic_workspaces
                    .entry(*foreign_id)
                    .or_default()
                    .push(ws_id);
            }
            zcosmic_toplevel_handle_v1::Event::ExtWorkspaceLeave { workspace } => {
                let ws_id = workspace.id().protocol_id();
                if let Some(ids) = state.cosmic_workspaces.get_mut(foreign_id) {
                    ids.retain(|id| *id != ws_id);
                }
            }
            zcosmic_toplevel_handle_v1::Event::WorkspaceEnter { workspace } => {
                // Deprecated cosmic workspace handle; map by protocol id if present in index.
                let ws_id = workspace.id().protocol_id();
                state
                    .cosmic_workspaces
                    .entry(*foreign_id)
                    .or_default()
                    .push(ws_id);
            }
            zcosmic_toplevel_handle_v1::Event::WorkspaceLeave { workspace } => {
                let ws_id = workspace.id().protocol_id();
                if let Some(ids) = state.cosmic_workspaces.get_mut(foreign_id) {
                    ids.retain(|id| *id != ws_id);
                }
            }
            zcosmic_toplevel_handle_v1::Event::State { state: words } => {
                let labels: Vec<String> = words
                    .chunks_exact(4)
                    .filter_map(|c| u32::from_ne_bytes(c.try_into().ok()?).into())
                    .map(|v| format!("state:{v}"))
                    .collect();
                // wayland array is typically sequence of u32 enums; keep raw bytes len too
                let raw = format!("bytes={}", words.len());
                let mut labels = labels;
                if labels.is_empty() {
                    labels.push(raw);
                }
                state.cosmic_states.insert(*foreign_id, labels);
            }
            zcosmic_toplevel_handle_v1::Event::OutputEnter { output } => {
                if std::env::var_os("MAESTRO_DEBUG_WORKSPACES").is_some() {
                    eprintln!(
                        "[maestro:ws] OutputEnter foreign={} output={}",
                        foreign_id,
                        output.id().protocol_id()
                    );
                }
            }
            _ => {}
        }
    }
}

impl Dispatch<ext_workspace_manager_v1::ExtWorkspaceManagerV1, ()> for SnapshotState {
    fn event(
        state: &mut Self,
        _: &ext_workspace_manager_v1::ExtWorkspaceManagerV1,
        event: ext_workspace_manager_v1::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        match event {
            ext_workspace_manager_v1::Event::WorkspaceGroup { workspace_group } => {
                state._ext_ws_groups.push(workspace_group);
            }
            ext_workspace_manager_v1::Event::Workspace { workspace } => {
                let id = workspace.id().protocol_id();
                state.workspace_order.push(id);
                state._ext_ws_handles.push(workspace);
            }
            ext_workspace_manager_v1::Event::Done => {
                state.workspace_done = true;
            }
            ext_workspace_manager_v1::Event::Finished => {}
            _ => {}
        }
    }

    wayland_client::event_created_child!(SnapshotState, ext_workspace_manager_v1::ExtWorkspaceManagerV1, [
        ext_workspace_manager_v1::EVT_WORKSPACE_GROUP_OPCODE => (ext_workspace_group_handle_v1::ExtWorkspaceGroupHandleV1, ()),
        ext_workspace_manager_v1::EVT_WORKSPACE_OPCODE => (ext_workspace_handle_v1::ExtWorkspaceHandleV1, ())
    ]);
}

impl Dispatch<ext_workspace_group_handle_v1::ExtWorkspaceGroupHandleV1, ()> for SnapshotState {
    fn event(
        _: &mut Self,
        _: &ext_workspace_group_handle_v1::ExtWorkspaceGroupHandleV1,
        _: ext_workspace_group_handle_v1::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<ext_workspace_handle_v1::ExtWorkspaceHandleV1, ()> for SnapshotState {
    fn event(
        state: &mut Self,
        handle: &ext_workspace_handle_v1::ExtWorkspaceHandleV1,
        event: ext_workspace_handle_v1::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        let id = handle.id().protocol_id();
        let meta = state.workspace_meta.entry(id).or_default();
        match event {
            ext_workspace_handle_v1::Event::Name { name } => {
                meta.name = Some(name);
            }
            ext_workspace_handle_v1::Event::Coordinates { coordinates } => {
                meta.coordinates = coordinates
                    .chunks_exact(4)
                    .filter_map(|c| u32::from_ne_bytes(c.try_into().ok()?).into())
                    .collect();
                // coordinates is an array of u32; wayland-rs may already decode — keep best effort
                if meta.coordinates.is_empty() && !coordinates.is_empty() {
                    meta.coordinates = coordinates.iter().map(|b| u32::from(*b)).collect();
                }
            }
            ext_workspace_handle_v1::Event::State { state: bits } => {
                // bitfield: active = 1 (see ext-workspace-v1.xml)
                meta.active = match bits {
                    wayland_client::WEnum::Value(v) => {
                        v.contains(ext_workspace_handle_v1::State::Active)
                    }
                    _ => false,
                };
            }
            ext_workspace_handle_v1::Event::Id { id: ws_id } => {
                if std::env::var_os("MAESTRO_DEBUG_WORKSPACES").is_some() {
                    eprintln!("[maestro:ws] workspace Id handle={id} id={ws_id}");
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_info_prefers_title_over_app_id() {
        let info = HandleInfo {
            title: Some("Today - TickTick".into()),
            app_id: Some("ticktick".into()),
            desktop: Some(2),
            done: true,
        };
        let record = handle_info_to_record(&info).expect("record");
        assert_eq!(record.title, "Today - TickTick");
        assert_eq!(record.pid, 0);
        assert_eq!(record.desktop, Some(2));
    }

    #[test]
    fn handle_info_falls_back_to_app_id_without_faking_workspace_zero() {
        let info = HandleInfo {
            title: None,
            app_id: Some("cursor".into()),
            desktop: None,
            done: true,
        };
        let record = handle_info_to_record(&info).expect("record");
        assert_eq!(record.title, "cursor");
        assert_eq!(record.desktop, None);
    }

    #[test]
    fn handle_info_skips_empty() {
        let info = HandleInfo::default();
        assert!(handle_info_to_record(&info).is_none());
    }

    #[test]
    fn workspace_handle_index_map_assigns_stable_zero_based_order() {
        let map = workspace_handle_index_map(&[10, 20, 10, 30]);
        assert_eq!(map.get(&10), Some(&0));
        assert_eq!(map.get(&20), Some(&1));
        assert_eq!(map.get(&30), Some(&2));
        assert_eq!(map.len(), 3);
    }

    #[test]
    fn resolve_workspace_index_prefers_lowest_when_multi() {
        let map = workspace_handle_index_map(&[10, 20, 30]);
        assert_eq!(resolve_workspace_index(&map, &[30, 10]), Some(0));
        assert_eq!(resolve_workspace_index(&map, &[20]), Some(1));
        assert_eq!(resolve_workspace_index(&map, &[99]), None);
        assert_eq!(resolve_workspace_index(&map, &[]), None);
    }

    #[test]
    fn wayland_fixture_records_merge_into_window_first_path() {
        use super::super::workspace::{
            merge_window_sources, uses_window_first_discovery, WindowSource,
        };
        let wayland = vec![
            WindowRecord {
                pid: 0,
                desktop: None,
                title: "Today - TickTick".into(),
                app_id: Some("ticktick".into()),
            },
            WindowRecord {
                pid: 0,
                desktop: Some(1),
                title: "Slack".into(),
                app_id: Some("Slack".into()),
            },
        ];
        let merged = merge_window_sources([WindowSource::WaylandForeignToplevel(wayland)]);
        assert!(uses_window_first_discovery(&merged));
        assert_eq!(merged.windows().len(), 2);
    }

    


    #[test]
    #[ignore = "requires live Cosmic Wayland session"]
    fn live_cosmic_workspaces_attach_distinct_indices() {
        let records = load_wayland_foreign_toplevel();
        assert!(!records.is_empty(), "expected foreign-toplevel windows");
        let with_ws: Vec<_> = records.iter().filter(|r| r.desktop.is_some()).collect();
        println!(
            "[live] wayland windows={} with_workspace={}",
            records.len(),
            with_ws.len()
        );
        for r in records.iter().take(12) {
            println!(
                "  desktop={:?} app_id={:?} title={}",
                r.desktop, r.app_id, r.title
            );
        }
        let indices: std::collections::BTreeSet<_> =
            with_ws.iter().filter_map(|r| r.desktop).collect();
        assert!(
            !indices.is_empty(),
            "expected at least one Cosmic workspace index attached"
        );
    }

    #[test]
    #[ignore = "requires live Cosmic Wayland session + MAESTRO_DEBUG_WORKSPACES=1"]
    fn live_debug_workspace_attach_for_missing_desktop() {
        std::env::set_var("MAESTRO_DEBUG_WORKSPACES", "1");
        let records = load_wayland_foreign_toplevel();
        let missing: Vec<_> = records.iter().filter(|r| r.desktop.is_none()).collect();
        println!(
            "[live-debug] total={} missing_desktop={}",
            records.len(),
            missing.len()
        );
        for r in &missing {
            println!(
                "  MISSING app_id={:?} title={}",
                r.app_id, r.title
            );
        }
        // Diagnostic only — always pass when the session responds.
        assert!(!records.is_empty());
    }
}
