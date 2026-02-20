import { defineStore } from "pinia";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

export interface OAuthConfig {
  use_custom: boolean;
  client_id: string | null;
  has_client_secret: boolean;
}

export const useOAuthConfigStore = defineStore("oauth-config", () => {
  const useCustom = ref(false);
  const clientId = ref<string | null>(null);
  const hasClientSecret = ref(false);
  const isLoading = ref(false);
  const error = ref<string | null>(null);

  async function loadConfig() {
    isLoading.value = true;
    error.value = null;

    try {
      const config = await invoke<OAuthConfig>("get_oauth_config");
      useCustom.value = config.use_custom;
      clientId.value = config.client_id;
      hasClientSecret.value = config.has_client_secret;
    } catch (e) {
      error.value = String(e);
    } finally {
      isLoading.value = false;
    }
  }

  async function saveConfig(newClientId: string, clientSecret: string) {
    isLoading.value = true;
    error.value = null;

    try {
      await invoke("save_oauth_config", {
        clientId: newClientId,
        clientSecret,
      });
      useCustom.value = true;
      clientId.value = newClientId;
      hasClientSecret.value = !!clientSecret;
    } catch (e) {
      error.value = String(e);
      throw e;
    } finally {
      isLoading.value = false;
    }
  }

  async function clearConfig() {
    isLoading.value = true;
    error.value = null;

    try {
      await invoke("clear_oauth_config");
      useCustom.value = false;
      clientId.value = null;
      hasClientSecret.value = false;
    } catch (e) {
      error.value = String(e);
    } finally {
      isLoading.value = false;
    }
  }

  return {
    useCustom,
    clientId,
    hasClientSecret,
    isLoading,
    error,
    loadConfig,
    saveConfig,
    clearConfig,
  };
});
