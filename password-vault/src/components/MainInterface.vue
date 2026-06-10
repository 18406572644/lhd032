<template>
  <div class="flex-1 flex h-full">
    <CategoryNav :active="activeCategory" :counts="categoryCounts" @select="activeCategory = $event" />

    <div class="flex-1 flex flex-col h-full overflow-hidden">
      <div class="px-6 py-3 border-b border-white/5 flex items-center gap-3">
        <div class="relative flex-1">
          <svg class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-text-muted" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"></circle><line x1="21" y1="21" x2="16.65" y2="16.65"></line></svg>
          <input
            v-model="searchQuery"
            type="text"
            placeholder="搜索名称或用户名..."
            class="w-full bg-surface-light/50 border border-white/5 rounded-lg pl-10 pr-4 py-2.5 text-text text-sm placeholder:text-text-muted/50 focus:outline-none focus:border-primary/30 focus:ring-1 focus:ring-primary/20 transition"
          />
        </div>
        <button
          @click="startScan"
          :disabled="scanning"
          class="px-4 py-2 bg-danger/20 text-danger rounded-lg hover:bg-danger/30 transition text-sm flex items-center gap-1.5 disabled:opacity-50"
          title="扫描所有密码是否泄露"
        >
          <svg v-if="!scanning" class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"></path><line x1="12" y1="9" x2="12" y2="13"></line><line x1="12" y1="17" x2="12.01" y2="17"></line></svg>
          <svg v-else class="w-4 h-4 animate-spin" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 12a9 9 0 1 1-6.219-8.56"></path></svg>
          {{ scanning ? '扫描中...' : '一键扫描' }}
        </button>
        <button
          @click="handleLock"
          class="p-2 rounded-lg hover:bg-danger/20 text-text-muted hover:text-danger transition"
          title="锁定密码库"
        >
          <svg class="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="3" y="11" width="18" height="11" rx="2" ry="2"></rect><path d="M7 11V7a5 5 0 0 1 10 0v4"></path></svg>
        </button>
        <button
          @click="showSettings = true"
          class="p-2 rounded-lg hover:bg-surface-light text-text-muted hover:text-text transition"
          title="设置"
        >
          <svg class="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="3"></circle><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"></path></svg>
        </button>
      </div>

      <div v-if="pwnedCount > 0 && activeCategory !== 'pwned'" class="mx-6 mt-4 p-3 bg-danger/10 border border-danger/30 rounded-lg flex items-center justify-between">
        <div class="flex items-center gap-2">
          <svg class="w-5 h-5 text-danger" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"></path><line x1="12" y1="9" x2="12" y2="13"></line><line x1="12" y1="17" x2="12.01" y2="17"></line></svg>
          <span class="text-sm text-danger">检测到 <strong>{{ pwnedCount }}</strong> 个密码已在数据泄露中出现，请尽快修改！</span>
        </div>
        <button @click="activeCategory = 'pwned'" class="text-sm text-danger hover:underline">查看</button>
      </div>

      <div class="flex-1 overflow-y-auto px-6 py-4">
        <div v-if="filteredEntries.length === 0" class="flex flex-col items-center justify-center h-full text-text-muted/50">
          <svg v-if="activeCategory === 'pwned'" class="w-16 h-16 mb-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1"><path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"></path><line x1="12" y1="9" x2="12" y2="13"></line><line x1="12" y1="17" x2="12.01" y2="17"></line></svg>
          <svg v-else class="w-16 h-16 mb-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1"><rect x="3" y="11" width="18" height="11" rx="2"></rect><path d="M7 11V7a5 5 0 0 1 10 0v4"></path></svg>
          <p class="text-sm">{{ activeCategory === 'pwned' ? '没有发现泄露的密码' : '暂无密码条目' }}</p>
        </div>
        <div v-else class="flex flex-col gap-3">
          <EntryCard
            v-for="entry in filteredEntries"
            :key="entry.id"
            :entry="entry"
            @edit="openEdit($event)"
          />
        </div>
      </div>

      <button
        @click="openCreate"
        class="fixed bottom-6 right-6 w-12 h-12 bg-gradient-to-r from-primary to-indigo-500 rounded-full shadow-lg shadow-primary/30 flex items-center justify-center text-white hover:scale-110 transition-transform duration-200 z-40"
      >
        <svg class="w-6 h-6" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><line x1="12" y1="5" x2="12" y2="19"></line><line x1="5" y1="12" x2="19" y2="12"></line></svg>
      </button>
    </div>

    <EntryForm
      v-if="showForm"
      :editEntry="editingEntry"
      @close="closeForm"
      @saved="onSaved"
    />

    <SettingsDialog
      v-if="showSettings"
      @close="showSettings = false"
      @saved="onSettingsSaved"
    />
  </div>
</template>

<script setup>
import { ref, computed, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import CategoryNav from "./CategoryNav.vue";
import EntryCard from "./EntryCard.vue";
import EntryForm from "./EntryForm.vue";
import SettingsDialog from "./SettingsDialog.vue";

const isTauri = typeof window !== "undefined" && !!window.__TAURI_INTERNALS__;

const emit = defineEmits(["lock"]);

const entries = ref([]);
const activeCategory = ref("all");
const searchQuery = ref("");
const showForm = ref(false);
const editingEntry = ref(null);
const showSettings = ref(false);
const scanning = ref(false);
let autoScanTimer = null;

const pwnedCount = computed(() => {
  return entries.value.filter(e => e.is_pwned).length;
});

const categoryCounts = computed(() => {
  const counts = { all: entries.value.length, pwned: pwnedCount.value };
  for (const e of entries.value) {
    counts[e.category] = (counts[e.category] || 0) + 1;
  }
  return counts;
});

const filteredEntries = computed(() => {
  let list = entries.value;
  if (activeCategory.value === "pwned") {
    list = list.filter(e => e.is_pwned);
  } else if (activeCategory.value !== "all") {
    list = list.filter(e => e.category === activeCategory.value);
  }
  if (searchQuery.value.trim()) {
    const q = searchQuery.value.toLowerCase();
    list = list.filter(e =>
      e.name.toLowerCase().includes(q) ||
      e.username.toLowerCase().includes(q)
    );
  }
  return list;
});

async function loadEntries() {
  if (!isTauri) return;
  try {
    entries.value = await invoke("get_entries");
  } catch (e) {
    console.error(e);
  }
}

async function startScan() {
  if (!isTauri || scanning.value) return;
  scanning.value = true;
  try {
    const results = await invoke("batch_scan_passwords");
    await loadEntries();

    const pwnedCount = results.filter(r => r.is_pwned).length;
    if (pwnedCount > 0) {
      try {
        const { isPermissionGranted, requestPermission, sendNotification } = await import("@tauri-apps/plugin-notification");
        const granted = await isPermissionGranted();
        if (!granted) {
          await requestPermission();
        }
        if (await isPermissionGranted()) {
          sendNotification({
            title: "密码泄露检测完成",
            body: `检测到 ${pwnedCount} 个密码已泄露，请尽快修改！`,
          });
        }
      } catch (notifErr) {
        console.error("Notification failed:", notifErr);
      }
    }
  } catch (e) {
    console.error("scan failed", e);
  } finally {
    scanning.value = false;
  }
}

async function setupAutoScan() {
  if (!isTauri) return;
  try {
    const settings = await invoke("get_hibp_settings");
    if (settings.auto_scan_enabled && settings.auto_scan_interval_days > 0) {
      if (settings.last_scan_time) {
        const lastScan = new Date(settings.last_scan_time);
        const now = new Date();
        const diffDays = (now - lastScan) / (1000 * 60 * 60 * 24);
        if (diffDays >= settings.auto_scan_interval_days) {
          startScan();
        }
      } else {
        startScan();
      }
    }
  } catch (e) {
    console.error(e);
  }
}

function onSettingsSaved() {
  setupAutoScan();
}

function handleLock() {
  if (confirm("确定要锁定密码库吗？需要重新输入主密码才能解锁。")) {
    emit("lock");
  }
}

function openCreate() {
  editingEntry.value = null;
  showForm.value = true;
}

function openEdit(entry) {
  editingEntry.value = entry;
  showForm.value = true;
}

function closeForm() {
  showForm.value = false;
  editingEntry.value = null;
}

async function onSaved() {
  closeForm();
  await loadEntries();
}

onMounted(async () => {
  try {
    await loadEntries();
    await setupAutoScan();
  } catch (e) {
    console.error("MainInterface mount error:", e);
  }
});

onUnmounted(() => {
  if (autoScanTimer) {
    clearInterval(autoScanTimer);
  }
});
</script>
