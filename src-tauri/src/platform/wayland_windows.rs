//! Short-lived Wayland foreign-toplevel snapshot for window-anchored discovery.
//! Soft-fails (empty list) when the compositor denies the protocol or times out.

use std::collections::HashMap;
use std::time::{Duration, Instant};

use wayland_client::{
    protocol::wl_registry, Connection, Dispatch, Proxy, QueueHandle,
};
use wayland_protocols::ext::foreign_toplevel_list::v1::client::{
    ext_foreign_toplevel_handle_v1, ext_foreign_toplevel_list_v1,
};

use super::workspace::WindowRecord;

const SNAPSHOT_TIMEOUT: Duration = Duration::from_millis(750);

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

/// Cosmic `zcosmic_toplevel_info_v1` can expose workspace handles, but mapping those
/// handles to a stable 0-based index needs the Cosmic workspace protocol as well.
/// Soft-fail for now: leave `desktop` unchanged (typically 0 from foreign-toplevel).
pub fn try_attach_cosmic_workspace_metadata(records: &mut [WindowRecord]) {
    let _ = records;
    // Spike confirmed Cosmic advertises `zcosmic_toplevel_info_v1` to unprivileged
    // clients; workspace index attachment is deferred to avoid pulling Cosmic
    // workspace protocol wiring into this change. Capture remains usable flat.
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

    // Optional Cosmic enrichment soft-fails today (see try_attach_cosmic_workspace_metadata).
    let mut records: Vec<WindowRecord> = state
        .handles
        .values()
        .filter_map(handle_info_to_record)
        .collect();
    try_attach_cosmic_workspace_metadata(&mut records);
    Ok(records)
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
        desktop: info.desktop.unwrap_or(0),
        title,
        app_id: info.app_id.clone(),
    })
}

#[derive(Default)]
struct SnapshotState {
    list: Option<ext_foreign_toplevel_list_v1::ExtForeignToplevelListV1>,
    handles: HashMap<u32, HandleInfo>,
    /// Keep proxies alive for the snapshot duration.
    _proxies: Vec<ext_foreign_toplevel_handle_v1::ExtForeignToplevelHandleV1>,
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
            if interface == "ext_foreign_toplevel_list_v1" {
                let list = registry
                    .bind::<ext_foreign_toplevel_list_v1::ExtForeignToplevelListV1, _, _>(
                        name,
                        version.min(1),
                        qh,
                        (),
                    );
                state.list = Some(list);
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
        assert_eq!(record.desktop, 2);
    }

    #[test]
    fn handle_info_falls_back_to_app_id() {
        let info = HandleInfo {
            title: None,
            app_id: Some("cursor".into()),
            desktop: None,
            done: true,
        };
        let record = handle_info_to_record(&info).expect("record");
        assert_eq!(record.title, "cursor");
        assert_eq!(record.desktop, 0);
    }

    #[test]
    fn handle_info_skips_empty() {
        let info = HandleInfo::default();
        assert!(handle_info_to_record(&info).is_none());
    }

    #[test]
    fn cosmic_workspace_attach_is_soft_noop() {
        let mut records = vec![WindowRecord {
            pid: 0,
            desktop: 0,
            title: "Firefox".into(),
            app_id: Some("firefox".into()),
        }];
        try_attach_cosmic_workspace_metadata(&mut records);
        assert_eq!(records[0].desktop, 0);
    }

    #[test]
    fn wayland_fixture_records_merge_into_window_first_path() {
        use super::super::workspace::{
            merge_window_sources, uses_window_first_discovery, WindowSource,
        };
        let wayland = vec![
            WindowRecord {
                pid: 0,
                desktop: 0,
                title: "Today - TickTick".into(),
                app_id: Some("ticktick".into()),
            },
            WindowRecord {
                pid: 0,
                desktop: 1,
                title: "Slack".into(),
                app_id: Some("Slack".into()),
            },
        ];
        let merged = merge_window_sources([WindowSource::WaylandForeignToplevel(wayland)]);
        assert!(uses_window_first_discovery(&merged));
        assert_eq!(merged.windows().len(), 2);
    }
}
