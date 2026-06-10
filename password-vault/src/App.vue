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
import UnlockPage from "./components/UnlockPage.vue";
import MainInterface from "./components/MainInterface.vue";

const unlocked = ref(false);

const securitySettings = ref({
  auto_lock_timeout_minutes: 5,
  lock_on_minimize: false,
});

let lastActivityTime = 0;
let pollingTimer = null;
let unlistenClose = null;
let unlistenMinimize = null;
let listenersAttached = false;

const POLLING_INTERVAL_MS = 1000;

const ACTIVITY_EVENTS = [
  "mousedown",
  "mousemove",
  "keydown",
  "keyup",
  "touchstart",
  "touchmove",
  "wheel",
  "click",
  "scroll",
];

function log(...args) {
  console.log("[AutoLock]", ...args);
}

async function loadSecuritySettings() {
  try {
    const settings = await invoke("get_security_settings");
    securitySettings.value = settings;
    log("Loaded settings:", settings);
  } catch (e) {
    console.error("[AutoLock] Failed to load security settings:", e);
  }
}

function touchActivity() {
  lastActivityTime = Date.now();
}

function checkIdleTimeout() {
  if (!unlocked.value) return;

  const timeoutMs = securitySettings.value.auto_lock_timeout_minutes * 60 * 1000;
  if (timeoutMs <= 0) return;

  const now = Date.now();
  const elapsed = now - lastActivityTime;

  if (elapsed >= timeoutMs) {
    log(
      `Idle timeout reached: elapsed=${(elapsed / 1000).toFixed(0)}s, ` +
      `threshold=${(timeoutMs / 1000).toFixed(0)}s, locking vault...`
    );
    lockVault();
  }
}

function startPolling() {
  if (pollingTimer) return;
  pollingTimer = setInterval(() => {
    checkIdleTimeout();
  }, POLLING_INTERVAL_MS);
  log(`Polling started (interval=${POLLING_INTERVAL_MS}ms)`);
}

function stopPolling() {
  if (pollingTimer) {
    clearInterval(pollingTimer);
    pollingTimer = null;
    log("Polling stopped");
  }
}

function addActivityListeners() {
  if (listenersAttached) return;
  for (const event of ACTIVITY_EVENTS) {
    document.addEventListener(event, touchActivity, { passive: true });
  }
  document.addEventListener("visibilitychange", handleVisibilityChange);
  window.addEventListener("focus", touchActivity);
  listenersAttached = true;
  log("Activity listeners attached");
}

function removeActivityListeners() {
  if (!listenersAttached) return;
  for (const event of ACTIVITY_EVENTS) {
    document.removeEventListener(event, touchActivity);
  }
  document.removeEventListener("visibilitychange", handleVisibilityChange);
  window.removeEventListener("focus", touchActivity);
  listenersAttached = false;
  log("Activity listeners removed");
}

function handleVisibilityChange() {
  if (document.visibilityState === "visible") {
    log("Page became visible, checking idle timeout immediately");
    checkIdleTimeout();
    if (unlocked.value) {
      touchActivity();
    }
  }
}

async function lockVault() {
  if (!unlocked.value) return;
  log("Locking vault...");

  try {
    await invoke("lock_vault");
    log("Backend MasterKey cleared");
  } catch (e) {
    console.error("[AutoLock] Failed to invoke lock_vault:", e);
  }

  stopPolling();
  removeActivityListeners();
  unlocked.value = false;
  log("Vault locked");
}

async function onVaultUnlocked() {
  log("Vault unlocked event received");
  unlocked.value = true;
  touchActivity();
  await loadSecuritySettings();
  addActivityListeners();
  startPolling();
}

async function setupWindowListeners() {
  try {
    const { getCurrentWindow } = await import("@tauri-apps/api/window");
    const appWindow = getCurrentWindow();

    unlistenClose = await appWindow.onCloseRequested(() => {
      if (unlocked.value) {
        invoke("lock_vault").catch((e) =>
          console.error("[AutoLock] close lock_vault error:", e)
        );
      }
    });
    log("Close request listener registered");

    unlistenMinimize = await appWindow.listen("tauri://minimize", async () => {
      if (securitySettings.value.lock_on_minimize && unlocked.value) {
        log("Window minimized and lock-on-minimize enabled, locking vault...");
        await lockVault();
      }
    });
    log("Minimize listener registered");
  } catch (e) {
    console.error("[AutoLock] Failed to setup Tauri window listeners:", e);
  }
}

watch(
  () => securitySettings.value.auto_lock_timeout_minutes,
  (newVal) => {
    log(`Timeout setting changed to ${newVal}min, resetting activity timestamp`);
    touchActivity();
  }
);

watch(
  () => unlocked.value,
  (newVal) => {
    if (newVal) {
      touchActivity();
      addActivityListeners();
      startPolling();
    } else {
      stopPolling();
      removeActivityListeners();
    }
  }
);

onMounted(async () => {
  log("App mounted");
  try {
    await loadSecuritySettings();
    await setupWindowListeners();
  } catch (e) {
    console.error("[AutoLock] Mount initialization error:", e);
  }
});

onUnmounted(() => {
  stopPolling();
  removeActivityListeners();
  if (unlistenClose) {
    try {
      unlistenClose();
    } catch (_) {}
  }
  if (unlistenMinimize) {
    try {
      unlistenMinimize();
    } catch (_) {}
  }
});
</script>
