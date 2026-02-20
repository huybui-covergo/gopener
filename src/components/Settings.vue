<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useSettingsStore } from "../stores/settings";
import { useAuthStore } from "../stores/auth";
import { useOAuthConfigStore } from "../stores/oauth-config";

const emit = defineEmits<{
  (e: "browse-folder"): void;
}>();

const settingsStore = useSettingsStore();
const authStore = useAuthStore();
const oauthConfigStore = useOAuthConfigStore();

const showOAuthConfig = ref(false);
const customClientId = ref("");
const customClientSecret = ref("");
const oauthError = ref<string | null>(null);

onMounted(async () => {
  await oauthConfigStore.loadConfig();
  if (oauthConfigStore.clientId) {
    customClientId.value = oauthConfigStore.clientId;
  }
});

async function saveOAuthConfig() {
  oauthError.value = null;

  if (!customClientId.value.trim()) {
    oauthError.value = "Client ID is required";
    return;
  }

  try {
    await oauthConfigStore.saveConfig(
      customClientId.value.trim(),
      customClientSecret.value.trim()
    );
    showOAuthConfig.value = false;

    // Sign out to use new credentials
    if (authStore.isAuthenticated) {
      await authStore.signOut();
    }
  } catch (e) {
    oauthError.value = String(e);
  }
}

async function clearOAuthConfig() {
  await oauthConfigStore.clearConfig();
  customClientId.value = "";
  customClientSecret.value = "";
  showOAuthConfig.value = false;

  // Sign out when reverting to default
  if (authStore.isAuthenticated) {
    await authStore.signOut();
  }
}

function toggleTheme() {
  const newTheme = settingsStore.theme === "dark" ? "light" : "dark";
  settingsStore.setTheme(newTheme);
}
</script>

<template>
  <div class="settings">
    <h2 class="settings-title">Settings</h2>

    <div class="settings-sections">
      <!-- Upload Settings -->
      <section class="settings-section">
        <h3 class="section-title">Upload</h3>

        <div class="setting-item">
          <div class="setting-info">
            <label class="setting-label">Default Folder</label>
            <p class="setting-description">
              Where files are uploaded by default
            </p>
          </div>
          <button class="btn btn-secondary btn-sm" @click="emit('browse-folder')">
            {{ settingsStore.defaultFolderName || "My Drive" }}
          </button>
        </div>

        <div class="setting-item">
          <div class="setting-info">
            <label class="setting-label">Open after upload</label>
            <p class="setting-description">
              Automatically open files in Google after uploading
            </p>
          </div>
          <button
            class="toggle"
            :class="{ active: settingsStore.autoOpenAfterUpload }"
            @click="
              settingsStore.setAutoOpenAfterUpload(
                !settingsStore.autoOpenAfterUpload
              )
            "
          ></button>
        </div>

        <div class="setting-item">
          <div class="setting-info">
            <label class="setting-label">Auto-close quick upload</label>
            <p class="setting-description">
              Close the quick upload dialog after successful upload
            </p>
          </div>
          <button
            class="toggle"
            :class="{ active: settingsStore.autoCloseAfterUpload }"
            @click="
              settingsStore.setAutoCloseAfterUpload(
                !settingsStore.autoCloseAfterUpload
              )
            "
          ></button>
        </div>
      </section>

      <!-- Appearance -->
      <section class="settings-section">
        <h3 class="section-title">Appearance</h3>

        <div class="setting-item">
          <div class="setting-info">
            <label class="setting-label">Theme</label>
            <p class="setting-description">Choose light or dark theme</p>
          </div>
          <button class="btn btn-secondary btn-sm" @click="toggleTheme">
            {{ settingsStore.theme === "dark" ? "Dark" : "Light" }}
          </button>
        </div>
      </section>

      <!-- OAuth Configuration -->
      <section class="settings-section">
        <h3 class="section-title">OAuth Credentials</h3>

        <div class="setting-item">
          <div class="setting-info">
            <label class="setting-label">Credential Type</label>
            <p class="setting-description">
              {{
                oauthConfigStore.useCustom
                  ? "Using custom OAuth credentials"
                  : "Using default app credentials"
              }}
            </p>
          </div>
          <button
            class="btn btn-secondary btn-sm"
            @click="showOAuthConfig = !showOAuthConfig"
          >
            {{ oauthConfigStore.useCustom ? "Edit" : "Use Custom" }}
          </button>
        </div>

        <!-- OAuth Config Form -->
        <div v-if="showOAuthConfig" class="oauth-config-form">
          <div class="form-group">
            <label class="form-label">Client ID</label>
            <input
              v-model="customClientId"
              type="text"
              class="input"
              placeholder="xxxxx.apps.googleusercontent.com"
            />
          </div>

          <div class="form-group">
            <label class="form-label">Client Secret</label>
            <input
              v-model="customClientSecret"
              type="password"
              class="input"
              placeholder="Your client secret"
            />
            <p class="form-hint">
              Required for some OAuth configurations
            </p>
          </div>

          <p v-if="oauthError" class="error-text">{{ oauthError }}</p>

          <div class="oauth-actions">
            <a
              href="https://console.cloud.google.com/apis/credentials"
              target="_blank"
              class="btn btn-ghost btn-sm"
            >
              Get credentials
            </a>
            <div class="spacer"></div>
            <button
              v-if="oauthConfigStore.useCustom"
              class="btn btn-ghost btn-sm"
              @click="clearOAuthConfig"
            >
              Use Default
            </button>
            <button class="btn btn-primary btn-sm" @click="saveOAuthConfig">
              Save
            </button>
          </div>
        </div>
      </section>

      <!-- Account -->
      <section class="settings-section">
        <h3 class="section-title">Account</h3>

        <div class="setting-item">
          <div class="setting-info">
            <label class="setting-label">Google Account</label>
            <p class="setting-description">
              {{
                authStore.isAuthenticated
                  ? "Signed in"
                  : "Not signed in"
              }}
            </p>
          </div>
          <button
            v-if="authStore.isAuthenticated"
            class="btn btn-danger btn-sm"
            @click="authStore.signOut()"
          >
            Sign Out
          </button>
        </div>
      </section>

      <!-- Data -->
      <section class="settings-section">
        <h3 class="section-title">Data</h3>

        <div class="setting-item">
          <div class="setting-info">
            <label class="setting-label">Recent Files</label>
            <p class="setting-description">
              {{ settingsStore.recentFiles.length }} files in history
            </p>
          </div>
          <button
            class="btn btn-secondary btn-sm"
            @click="settingsStore.clearRecentFiles()"
            :disabled="settingsStore.recentFiles.length === 0"
          >
            Clear History
          </button>
        </div>
      </section>
    </div>
  </div>
</template>

<style scoped>
.settings {
  max-width: 600px;
  margin: 0 auto;
}

.settings-title {
  font-size: 16px;
  font-weight: 600;
  margin: 0 0 24px;
}

.settings-sections {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.settings-section {
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-lg);
  padding: 16px;
}

.section-title {
  font-size: 12px;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  color: var(--text-secondary);
  margin: 0 0 12px;
}

.setting-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  padding: 12px 0;
}

.setting-item:not(:last-child) {
  border-bottom: 1px solid var(--border-color);
}

.setting-info {
  flex: 1;
}

.setting-label {
  font-size: 13px;
  font-weight: 500;
  display: block;
}

.setting-description {
  font-size: 12px;
  color: var(--text-secondary);
  margin: 2px 0 0;
}

.oauth-config-form {
  padding: 16px;
  margin-top: 12px;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.oauth-actions {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-top: 8px;
}

.spacer {
  flex: 1;
}

.error-text {
  font-size: 12px;
  color: var(--error-color);
  margin: 0;
}
</style>
