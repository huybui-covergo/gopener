import { describe, it, expect, beforeEach, vi } from "vitest";
import { setActivePinia, createPinia } from "pinia";
import { useOAuthConfigStore } from "../oauth-config";
import { invoke } from "@tauri-apps/api/core";

vi.mock("@tauri-apps/api/core");

const mockedInvoke = vi.mocked(invoke);

describe("useOAuthConfigStore", () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    vi.clearAllMocks();
  });

  it("has correct initial state", () => {
    const store = useOAuthConfigStore();

    expect(store.useCustom).toBe(false);
    expect(store.clientId).toBeNull();
    expect(store.hasClientSecret).toBe(false);
    expect(store.isLoading).toBe(false);
    expect(store.error).toBeNull();
  });

  describe("loadConfig", () => {
    it("loads config from backend", async () => {
      const store = useOAuthConfigStore();
      mockedInvoke.mockResolvedValueOnce({
        use_custom: true,
        client_id: "custom-id-123",
        has_client_secret: true,
      });

      await store.loadConfig();

      expect(mockedInvoke).toHaveBeenCalledWith("get_oauth_config");
      expect(store.useCustom).toBe(true);
      expect(store.clientId).toBe("custom-id-123");
      expect(store.hasClientSecret).toBe(true);
      expect(store.isLoading).toBe(false);
    });

    it("sets error on failure", async () => {
      const store = useOAuthConfigStore();
      mockedInvoke.mockRejectedValueOnce(new Error("Load failed"));

      await store.loadConfig();

      expect(store.error).toBe("Error: Load failed");
      expect(store.isLoading).toBe(false);
    });
  });

  describe("saveConfig", () => {
    it("saves custom credentials", async () => {
      const store = useOAuthConfigStore();
      mockedInvoke.mockResolvedValueOnce(undefined);

      await store.saveConfig("my-client-id", "my-secret");

      expect(mockedInvoke).toHaveBeenCalledWith("save_oauth_config", {
        clientId: "my-client-id",
        clientSecret: "my-secret",
      });
      expect(store.useCustom).toBe(true);
      expect(store.clientId).toBe("my-client-id");
      expect(store.hasClientSecret).toBe(true);
    });

    it("sets hasClientSecret to false when secret is empty", async () => {
      const store = useOAuthConfigStore();
      mockedInvoke.mockResolvedValueOnce(undefined);

      await store.saveConfig("my-client-id", "");

      expect(store.hasClientSecret).toBe(false);
    });

    it("throws and sets error on failure", async () => {
      const store = useOAuthConfigStore();
      mockedInvoke.mockRejectedValueOnce(new Error("Save failed"));

      await expect(store.saveConfig("id", "secret")).rejects.toThrow(
        "Save failed"
      );
      expect(store.error).toBe("Error: Save failed");
    });
  });

  describe("clearConfig", () => {
    it("clears custom config", async () => {
      const store = useOAuthConfigStore();
      store.useCustom = true;
      store.clientId = "some-id";
      store.hasClientSecret = true;
      mockedInvoke.mockResolvedValueOnce(undefined);

      await store.clearConfig();

      expect(mockedInvoke).toHaveBeenCalledWith("clear_oauth_config");
      expect(store.useCustom).toBe(false);
      expect(store.clientId).toBeNull();
      expect(store.hasClientSecret).toBe(false);
    });

    it("sets error on failure", async () => {
      const store = useOAuthConfigStore();
      mockedInvoke.mockRejectedValueOnce(new Error("Clear failed"));

      await store.clearConfig();

      expect(store.error).toBe("Error: Clear failed");
    });
  });
});
