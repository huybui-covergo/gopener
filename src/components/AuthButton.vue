<script setup lang="ts">
import { ref } from "vue";
import { useAuthStore } from "../stores/auth";

const authStore = useAuthStore();
const showCodeInput = ref(false);
const authCode = ref("");

async function handleSignIn() {
  try {
    await authStore.signIn();
    showCodeInput.value = true;
  } catch (e) {
    console.error("Sign in failed:", e);
  }
}

async function handleSubmitCode() {
  if (!authCode.value.trim()) return;

  try {
    await authStore.handleCallback(authCode.value.trim());
    showCodeInput.value = false;
    authCode.value = "";
  } catch (e) {
    console.error("Auth callback failed:", e);
  }
}

function handleSignOut() {
  authStore.signOut();
}

function cancelCodeInput() {
  showCodeInput.value = false;
  authCode.value = "";
}
</script>

<template>
  <div class="auth-button">
    <template v-if="!authStore.isAuthenticated">
      <template v-if="!showCodeInput">
        <button
          class="btn btn-primary"
          @click="handleSignIn"
          :disabled="authStore.isLoading"
        >
          <svg
            width="16"
            height="16"
            viewBox="0 0 24 24"
            fill="currentColor"
          >
            <path
              d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z"
            />
            <path
              d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"
            />
            <path
              d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z"
            />
            <path
              d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z"
            />
          </svg>
          Sign in with Google
        </button>
      </template>

      <template v-else>
        <div class="code-input-container">
          <p class="code-hint">
            After signing in, paste the authorization code here:
          </p>
          <div class="code-input-row">
            <input
              v-model="authCode"
              type="text"
              class="input"
              placeholder="Paste authorization code"
              @keyup.enter="handleSubmitCode"
            />
            <button
              class="btn btn-primary btn-sm"
              @click="handleSubmitCode"
              :disabled="!authCode.trim() || authStore.isLoading"
            >
              Submit
            </button>
            <button class="btn btn-ghost btn-sm" @click="cancelCodeInput">
              Cancel
            </button>
          </div>
        </div>
      </template>
    </template>

    <template v-else>
      <button
        class="btn btn-ghost btn-sm"
        @click="handleSignOut"
        :disabled="authStore.isLoading"
      >
        Sign out
      </button>
    </template>

    <p v-if="authStore.error" class="error-text">{{ authStore.error }}</p>
  </div>
</template>

<style scoped>
.auth-button {
  display: flex;
  align-items: center;
  gap: 8px;
}

.code-input-container {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.code-hint {
  font-size: 12px;
  color: var(--text-secondary);
  margin: 0;
}

.code-input-row {
  display: flex;
  gap: 8px;
  align-items: center;
}

.code-input-row .input {
  width: 200px;
  font-size: 12px;
  padding: 6px 10px;
}

.error-text {
  font-size: 12px;
  color: var(--error-color);
  margin: 0;
}
</style>
