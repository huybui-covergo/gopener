import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";

interface AuthState {
  is_authenticated: boolean;
  access_token: string | null;
  expires_at: number | null;
}

export const useAuthStore = defineStore("auth", () => {
  const isAuthenticated = ref(false);
  const accessToken = ref<string | null>(null);
  const expiresAt = ref<number | null>(null);
  const isLoading = ref(false);
  const error = ref<string | null>(null);

  const isExpired = computed(() => {
    if (!expiresAt.value) return true;
    const now = Math.floor(Date.now() / 1000);
    return expiresAt.value <= now + 300; // 5 minute buffer
  });

  async function checkAuth() {
    isLoading.value = true;
    error.value = null;

    try {
      const state = await invoke<AuthState>("check_auth");
      isAuthenticated.value = state.is_authenticated;
      accessToken.value = state.access_token;
      expiresAt.value = state.expires_at;
    } catch (e) {
      error.value = String(e);
      isAuthenticated.value = false;
    } finally {
      isLoading.value = false;
    }
  }

  async function signIn() {
    isLoading.value = true;
    error.value = null;

    try {
      // Get the OAuth URL
      const authUrl = await invoke<string>("get_auth_url");

      // Open in default browser
      const { open } = await import("@tauri-apps/plugin-shell");
      await open(authUrl);

      // Start a local server to receive the callback
      // For simplicity, we'll prompt the user to paste the code
      // In production, you'd use a local HTTP server
      return authUrl;
    } catch (e) {
      error.value = String(e);
      throw e;
    } finally {
      isLoading.value = false;
    }
  }

  async function handleCallback(code: string) {
    isLoading.value = true;
    error.value = null;

    try {
      const state = await invoke<AuthState>("exchange_code", { code });
      isAuthenticated.value = state.is_authenticated;
      accessToken.value = state.access_token;
      expiresAt.value = state.expires_at;
    } catch (e) {
      error.value = String(e);
      throw e;
    } finally {
      isLoading.value = false;
    }
  }

  async function refreshToken() {
    try {
      const state = await invoke<AuthState>("refresh_token");
      isAuthenticated.value = state.is_authenticated;
      accessToken.value = state.access_token;
      expiresAt.value = state.expires_at;
    } catch (e) {
      error.value = String(e);
      isAuthenticated.value = false;
    }
  }

  async function signOut() {
    isLoading.value = true;
    error.value = null;

    try {
      await invoke("sign_out");
      isAuthenticated.value = false;
      accessToken.value = null;
      expiresAt.value = null;
    } catch (e) {
      error.value = String(e);
    } finally {
      isLoading.value = false;
    }
  }

  return {
    isAuthenticated,
    accessToken,
    expiresAt,
    isLoading,
    error,
    isExpired,
    checkAuth,
    signIn,
    handleCallback,
    refreshToken,
    signOut,
  };
});
