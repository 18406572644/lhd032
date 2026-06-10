<template>
  <div class="h-screen w-screen flex flex-col rounded-xl overflow-hidden border border-white/5">
    <div class="drag-region h-8 w-full flex-shrink-0" data-tauri-drag-region></div>
    <UnlockPage v-if="!unlocked" @unlocked="onVaultUnlocked" />
    <MainInterface v-else @lock="lockVault" />
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import UnlockPage from "./components/UnlockPage.vue";
import MainInterface from "./components/MainInterface.vue";

const unlocked = ref(false);

const securitySettings = ref({
  auto_lock_timeout_minutes: 5,
  lock_on_minimize: false,
});

let idleTimer = null;
let unlistenClose = null;
const ACTIVITY_EVENTS = [
  "mousedown",
  "mousemove",
  "keydown",
  "keyup",
  "touchstart",
  "wheel",
  "click",
  "scroll",
];

async function loadSecuritySettings() {
  try {
    const settings = await invoke("get_security_settings");
    securitySettings.value = settings;
  } catch (e) {
    console.error("Failed to load security settings:", e);
  }
}

function resetIdleTimer() {
  if (idleTimer) {
    clearTimeout(idleTimer);
    idleTimer = null;
  }
  if (!unlocked.value) return;

  const timeoutMs = securitySettings.value.auto_lock_timeout_minutes * 60 * 1000;
  if (timeoutMs <= 0) return;

  idleTimer = setTimeout(() => {
    console.log("[AutoLock] Idle timeout reached, locking vault...");
    lockVault();
  }, timeoutMs);
}

function addActivityListeners() {
  for (const event of ACTIVITY_EVENTS) {
    document.addEventListener(event, resetIdleTimer, { passive: true });
  }
}

function removeActivityListeners() {
  for (const event of ACTIVITY_EVENTS) {
    document.removeEventListener(event, resetIdleTimer);
  }
}

async function lockVault() {
  if (!unlocked.value) return;

  try {
    await invoke("lock_vault");
  } catch (e) {
    console.error("Failed to lock vault:", e);
  }

  if (idleTimer) {
    clearTimeout(idleTimer);
    idleTimer = null;
  }
  removeActivityListeners();
  unlocked.value = false;
}

async function onVaultUnlocked() {
  unlocked.value = true;
  await loadSecuritySettings();
  addActivityListeners();
  resetIdleTimer();
}

async function setupWindowListeners() {
  const appWindow = getCurrentWindow();

  try {
    unlistenClose = await appWindow.onCloseRequested(() => {
      if (unlocked.value) {
        invoke("lock_vault").catch(console.error);
      }
    });
  } catch (e) {
    console.error("Failed to setup close listener:", e);
  }

  try {
    await appWindow.listen("tauri://minimize", async () => {
      if (securitySettings.value.lock_on_minimize && unlocked.value) {
        console.log("[AutoLock] Window minimized, locking vault...");
        await lockVault();
      }
    });
  } catch (e) {
    console.error("Failed to setup minimize listener:", e);
  }
}

watch(
  () => securitySettings.value.auto_lock_timeout_minutes,
  () => {
    if (unlocked.value) {
      resetIdleTimer();
    }
  }
);

watch(
  () => unlocked.value,
  (newVal) => {
    if (newVal) {
      addActivityListeners();
      resetIdleTimer();
    } else {
      if (idleTimer) {
        clearTimeout(idleTimer);
        idleTimer = null;
      }
      removeActivityListeners();
    }
  }
);

onMounted(async () => {
  await loadSecuritySettings();
  await setupWindowListeners();
});

onUnmounted(() => {
  if (idleTimer) {
    clearTimeout(idleTimer);
  }
  removeActivityListeners();
  if (unlistenClose) {
    unlistenClose();
  }
});
</script>
