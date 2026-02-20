import { describe, it, expect, beforeEach, vi } from "vitest";
import { setActivePinia, createPinia } from "pinia";
import { useAuthStore } from "../auth";
import { invoke } from "@tauri-apps/api/core";

vi.mock("@tauri-apps/api/core");
vi.mock("@tauri-apps/plugin-shell");

const mockedInvoke = vi.mocked(invoke);

describe("useAuthStore", () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    vi.clearAllMocks();
  });

  it("has correct initial state", () => {
    const store = useAuthStore();

    expect(store.isAuthenticated).toBe(false);
    expect(store.accessToken).toBeNull();
    expect(store.expiresAt).toBeNull();
    expect(store.isLoading).toBe(false);
    expect(store.error).toBeNull();
  });

  it("isExpired returns true when expiresAt is null", () => {
    const store = useAuthStore();
    expect(store.isExpired).toBe(true);
  });

  it("isExpired returns true when token expires within 5 minutes", () => {
    const store = useAuthStore();
    const now = Math.floor(Date.now() / 1000);
    store.expiresAt = now + 200; // less than 300s buffer
    expect(store.isExpired).toBe(true);
  });

  it("isExpired returns false when token has more than 5 minutes left", () => {
    const store = useAuthStore();
    const now = Math.floor(Date.now() / 1000);
    store.expiresAt = now + 600;
    expect(store.isExpired).toBe(false);
  });

  describe("checkAuth", () => {
    it("updates state on successful check", async () => {
      const store = useAuthStore();
      const mockState = {
        is_authenticated: true,
        access_token: "test-token",
        expires_at: 9999999999,
      };
      mockedInvoke.mockResolvedValueOnce(mockState);

      await store.checkAuth();

      expect(mockedInvoke).toHaveBeenCalledWith("check_auth");
      expect(store.isAuthenticated).toBe(true);
      expect(store.accessToken).toBe("test-token");
      expect(store.expiresAt).toBe(9999999999);
      expect(store.isLoading).toBe(false);
    });

    it("resets auth state on error", async () => {
      const store = useAuthStore();
      mockedInvoke.mockRejectedValueOnce(new Error("Network error"));

      await store.checkAuth();

      expect(store.isAuthenticated).toBe(false);
      expect(store.error).toBe("Error: Network error");
      expect(store.isLoading).toBe(false);
    });
  });

  describe("signIn", () => {
    it("returns auth URL on success", async () => {
      const store = useAuthStore();
      mockedInvoke.mockResolvedValueOnce("https://accounts.google.com/o/oauth2/...");

      const url = await store.signIn();

      expect(mockedInvoke).toHaveBeenCalledWith("get_auth_url");
      expect(url).toBe("https://accounts.google.com/o/oauth2/...");
      expect(store.isLoading).toBe(false);
    });

    it("sets error on failure", async () => {
      const store = useAuthStore();
      mockedInvoke.mockRejectedValueOnce(new Error("OAuth failed"));

      await expect(store.signIn()).rejects.toThrow("OAuth failed");
      expect(store.error).toBe("Error: OAuth failed");
    });
  });

  describe("handleCallback", () => {
    it("exchanges code and updates auth state", async () => {
      const store = useAuthStore();
      const mockState = {
        is_authenticated: true,
        access_token: "new-token",
        expires_at: 9999999999,
      };
      mockedInvoke.mockResolvedValueOnce(mockState);

      await store.handleCallback("auth-code-123");

      expect(mockedInvoke).toHaveBeenCalledWith("exchange_code", {
        code: "auth-code-123",
      });
      expect(store.isAuthenticated).toBe(true);
      expect(store.accessToken).toBe("new-token");
    });

    it("throws and sets error on failure", async () => {
      const store = useAuthStore();
      mockedInvoke.mockRejectedValueOnce(new Error("Invalid code"));

      await expect(store.handleCallback("bad-code")).rejects.toThrow(
        "Invalid code"
      );
      expect(store.error).toBe("Error: Invalid code");
    });
  });

  describe("refreshToken", () => {
    it("updates state on successful refresh", async () => {
      const store = useAuthStore();
      const mockState = {
        is_authenticated: true,
        access_token: "refreshed-token",
        expires_at: 9999999999,
      };
      mockedInvoke.mockResolvedValueOnce(mockState);

      await store.refreshToken();

      expect(mockedInvoke).toHaveBeenCalledWith("refresh_token");
      expect(store.accessToken).toBe("refreshed-token");
    });

    it("resets auth on refresh failure", async () => {
      const store = useAuthStore();
      store.isAuthenticated = true;
      mockedInvoke.mockRejectedValueOnce(new Error("Refresh failed"));

      await store.refreshToken();

      expect(store.isAuthenticated).toBe(false);
      expect(store.error).toBe("Error: Refresh failed");
    });
  });

  describe("signOut", () => {
    it("clears all auth state", async () => {
      const store = useAuthStore();
      store.isAuthenticated = true;
      store.accessToken = "some-token";
      store.expiresAt = 9999999999;
      mockedInvoke.mockResolvedValueOnce(undefined);

      await store.signOut();

      expect(mockedInvoke).toHaveBeenCalledWith("sign_out");
      expect(store.isAuthenticated).toBe(false);
      expect(store.accessToken).toBeNull();
      expect(store.expiresAt).toBeNull();
      expect(store.isLoading).toBe(false);
    });

    it("sets error if sign out fails", async () => {
      const store = useAuthStore();
      mockedInvoke.mockRejectedValueOnce(new Error("Keychain error"));

      await store.signOut();

      expect(store.error).toBe("Error: Keychain error");
    });
  });
});
