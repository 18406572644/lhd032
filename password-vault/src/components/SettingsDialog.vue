<template>
  <div class="fixed inset-0 bg-black/60 backdrop-blur-sm flex items-center justify-center z-50" @click.self="$emit('close')">
    <div class="bg-surface border border-white/10 rounded-2xl w-[520px] max-h-[85vh] overflow-y-auto shadow-2xl animate-fade-in">
      <div class="px-6 py-4 border-b border-white/5 flex items-center justify-between">
        <h2 class="text-lg font-semibold text-text flex items-center gap-2">
          <svg class="w-5 h-5 text-primary-light" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="3"></circle><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"></path></svg>
          安全设置
        </h2>
        <button @click="$emit('close')" class="p-1 rounded-lg hover:bg-surface-lighter text-text-muted hover:text-text transition">
          <svg class="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>
        </button>
      </div>

      <div class="px-6 py-4 flex flex-col gap-6">
        <div class="space-y-4">
          <h3 class="text-sm font-semibold text-text flex items-center gap-2">
            <svg class="w-4 h-4 text-primary" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="3" y="11" width="18" height="11" rx="2" ry="2"></rect><path d="M7 11V7a5 5 0 0 1 10 0v4"></path></svg>
            自动锁定
          </h3>

          <div>
            <label class="text-sm text-text block mb-2">无操作超时锁定</label>
            <select
              v-model.number="securitySettings.auto_lock_timeout_minutes"
              class="w-full bg-bg border border-white/10 rounded-lg px-3 py-2 text-text text-sm focus:outline-none focus:border-primary/50 transition"
            >
              <option :value="1">1 分钟</option>
              <option :value="5">5 分钟（推荐）</option>
              <option :value="10">10 分钟</option>
              <option :value="30">30 分钟</option>
              <option :value="0">从不</option>
            </select>
            <p class="text-xs text-text-muted mt-1.5">超过指定时间无鼠标/键盘操作将自动锁定密码库</p>
          </div>

          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-text">最小化时立即锁定</p>
              <p class="text-xs text-text-muted mt-0.5">窗口最小化后立即清除内存中的主密钥</p>
            </div>
            <button
              @click="securitySettings.lock_on_minimize = !securitySettings.lock_on_minimize"
              :class="[
                'relative w-11 h-6 rounded-full transition-colors duration-200',
                securitySettings.lock_on_minimize ? 'bg-primary' : 'bg-surface-lighter'
              ]"
            >
              <span
                :class="[
                  'absolute top-0.5 left-0.5 w-5 h-5 bg-white rounded-full transition-transform duration-200 shadow',
                  securitySettings.lock_on_minimize ? 'translate-x-5' : 'translate-x-0'
                ]"
              />
            </button>
          </div>
        </div>

        <div class="pt-4 border-t border-white/5">
          <div class="space-y-4">
          <h3 class="text-sm font-semibold text-text flex items-center gap-2">
            <svg class="w-4 h-4 text-danger" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"></path><line x1="12" y1="9" x2="12" y2="13"></line><line x1="12" y1="17" x2="12.01" y2="17"></line></svg>
            密码泄露检测 (HIBP)
          </h3>

          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-text">启用泄露检测</p>
              <p class="text-xs text-text-muted mt-0.5">使用 Have I Been Pwned API 检测密码是否已泄露</p>
            </div>
            <button
              @click="settings.enabled = !settings.enabled"
              :class="[
                'relative w-11 h-6 rounded-full transition-colors duration-200',
                settings.enabled ? 'bg-primary' : 'bg-surface-lighter'
              ]"
            >
              <span
                :class="[
                  'absolute top-0.5 left-0.5 w-5 h-5 bg-white rounded-full transition-transform duration-200 shadow',
                  settings.enabled ? 'translate-x-5' : 'translate-x-0'
                ]"
              />
            </button>
          </div>

          <div v-if="settings.enabled" class="pl-4 border-l border-white/10 space-y-4">
            <div class="flex items-center justify-between">
              <div>
                <p class="text-sm text-text">自动定期扫描</p>
                <p class="text-xs text-text-muted mt-0.5">自动检测已存储密码的泄露状态</p>
              </div>
              <button
                @click="settings.auto_scan_enabled = !settings.auto_scan_enabled"
                :class="[
                  'relative w-11 h-6 rounded-full transition-colors duration-200',
                  settings.auto_scan_enabled ? 'bg-primary' : 'bg-surface-lighter'
                ]"
              >
                <span
                  :class="[
                    'absolute top-0.5 left-0.5 w-5 h-5 bg-white rounded-full transition-transform duration-200 shadow',
                    settings.auto_scan_enabled ? 'translate-x-5' : 'translate-x-0'
                  ]"
                />
              </button>
            </div>

            <div v-if="settings.auto_scan_enabled">
              <label class="text-sm text-text block mb-2">扫描间隔</label>
              <select
                v-model.number="settings.auto_scan_interval_days"
                class="w-full bg-bg border border-white/10 rounded-lg px-3 py-2 text-text text-sm focus:outline-none focus:border-primary/50 transition"
              >
                <option :value="1">每天</option>
                <option :value="3">每 3 天</option>
                <option :value="7">每周</option>
                <option :value="14">每两周</option>
                <option :value="30">每月</option>
              </select>
            </div>

            <div class="flex items-center justify-between">
              <div>
                <p class="text-sm text-text">离线模式</p>
                <p class="text-xs text-text-muted mt-0.5">下载 HIBP 哈希数据库到本地进行比对</p>
              </div>
              <button
                @click="toggleOfflineMode"
                :class="[
                  'relative w-11 h-6 rounded-full transition-colors duration-200',
                  settings.offline_mode ? 'bg-primary' : 'bg-surface-lighter'
                ]"
              >
                <span
                  :class="[
                    'absolute top-0.5 left-0.5 w-5 h-5 bg-white rounded-full transition-transform duration-200 shadow',
                    settings.offline_mode ? 'translate-x-5' : 'translate-x-0'
                  ]"
                />
              </button>
            </div>

            <div v-if="settings.offline_mode" class="bg-bg/50 rounded-lg p-3 space-y-2">
              <div class="flex items-center justify-between text-sm">
                <span class="text-text-muted">本地数据库大小</span>
                <span class="text-text">{{ settings.offline_db_size_mb.toFixed(1) }} MB</span>
              </div>
              <button
                @click="downloadOfflineDb"
                :disabled="downloading"
                class="w-full py-2 bg-primary/20 text-primary-light rounded-lg hover:bg-primary/30 transition text-sm disabled:opacity-50"
              >
                {{ downloading ? '下载中...' : (settings.offline_db_size_mb > 0 ? '更新数据库' : '下载数据库') }}
              </button>
              <p v-if="downloadProgress > 0" class="text-xs text-text-muted text-center">
                已下载 {{ (downloadProgress / 1024 / 1024).toFixed(1) }} MB
              </p>
            </div>
          </div>
          </div>
        </div>

        <div class="pt-4 border-t border-white/5">
          <div class="flex items-center justify-between text-sm">
            <span class="text-text-muted">上次扫描时间</span>
            <span class="text-text">{{ settings.last_scan_time ? formatDate(settings.last_scan_time) : '从未扫描' }}</span>
          </div>
        </div>

        <div class="bg-primary/10 border border-primary/20 rounded-lg p-4">
          <p class="text-xs text-text-muted leading-relaxed">
            <span class="text-primary-light font-medium">隐私保护：</span>
            使用 k-匿名协议，仅发送密码 SHA-1 哈希的前 5 位十六进制字符到服务器，
            原始密码和完整哈希永远不会离开您的设备。
          </p>
        </div>
      </div>

      <div class="px-6 py-4 border-t border-white/5 flex justify-end gap-3">
        <button @click="$emit('close')" class="px-4 py-2 rounded-lg text-text-muted hover:bg-surface-lighter transition text-sm">取消</button>
        <button @click="handleSave" class="px-6 py-2 rounded-lg bg-gradient-to-r from-primary to-indigo-500 text-white font-medium text-sm hover:from-primary-dark hover:to-indigo-600 transition-all shadow-lg shadow-primary/20">保存</button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

const isTauri = typeof window !== "undefined" && !!window.__TAURI_INTERNALS__;

const emit = defineEmits(["close", "saved"]);

const settings = reactive({
  enabled: true,
  auto_scan_enabled: false,
  auto_scan_interval_days: 7,
  last_scan_time: null,
  offline_mode: false,
  offline_db_size_mb: 0.0
});

const securitySettings = reactive({
  auto_lock_timeout_minutes: 5,
  lock_on_minimize: false,
});

const downloading = ref(false);
const downloadProgress = ref(0);

async function loadSettings() {
  if (!isTauri) return;
  try {
    const s = await invoke("get_hibp_settings");
    Object.assign(settings, s);
  } catch (e) {
    console.error(e);
  }
  try {
    const s = await invoke("get_security_settings");
    Object.assign(securitySettings, s);
  } catch (e) {
    console.error(e);
  }
}

async function handleSave() {
  if (!isTauri) return;
  try {
    await invoke("save_hibp_settings", { settings });
    await invoke("save_security_settings", { settings: securitySettings });
    emit("saved");
    emit("close");
  } catch (e) {
    console.error(e);
  }
}

async function toggleOfflineMode() {
  settings.offline_mode = !settings.offline_mode;
  if (settings.offline_mode && isTauri) {
    try {
      await invoke("load_offline_hibp_db");
      const s = await invoke("get_hibp_settings");
      settings.offline_db_size_mb = s.offline_db_size_mb;
    } catch (e) {
      console.error(e);
    }
  }
}

function downloadOfflineDb() {
  downloading.value = true;
  downloadProgress.value = 0;
}

function formatDate(dateStr) {
  try {
    const date = new Date(dateStr);
    return date.toLocaleString('zh-CN', {
      year: 'numeric',
      month: '2-digit',
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit'
    });
  } catch {
    return dateStr;
  }
}

onMounted(loadSettings);
</script>
