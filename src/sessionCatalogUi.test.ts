import { afterEach, describe, expect, it } from "vitest";
import {
  loadPinnedPaths,
  profilesRootStorageKey,
  savePinnedPaths,
} from "./sessionCatalogUi";

describe("sessionCatalogUi", () => {
  const root = "/tmp/maestro-test-profiles";

  afterEach(() => {
    localStorage.clear();
  });

  it("builds a stable hex storage key from profiles root", () => {
    expect(profilesRootStorageKey(root)).toMatch(/^m_[0-9a-f]+$/);
    expect(profilesRootStorageKey(root)).toBe(profilesRootStorageKey(` ${root} `));
  });

  it("round-trips pinned paths in localStorage", () => {
    expect(loadPinnedPaths(root)).toEqual([]);
    savePinnedPaths(root, ["/a.json", "/b.json"]);
    expect(loadPinnedPaths(root)).toEqual(["/a.json", "/b.json"]);
  });
});
