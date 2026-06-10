<template>
  <div class="fixed inset-0 bg-black/60 backdrop-blur-sm flex items-center justify-center z-50" @click.self="$emit('close')">
    <div class="bg-surface border border-white/10 rounded-2xl w-[440px] max-h-[85vh] overflow-y-auto shadow-2xl animate-fade-in">
      <div class="px-6 py-4 border-b border-white/5 flex items-center justify-between">
        <h2 class="text-lg font-semibold text-text">{{ isEdit ? '编辑条目' : '新建条目' }}</h2>
        <button @click="$emit('close')" class="p-1 rounded-lg hover:bg-surface-lighter text-text-muted hover:text-text transition">
          <svg class="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>
        </button>
      </div>

      <div v-if="toast.show" :class="[
        'mx-6 mt-4 px-3 py-2 rounded-lg text-sm flex items-center gap-2 animate-fade-in',
        toast.type === 'success' ? 'bg-success/10 border border-success/30 text-success' : 'bg-danger/10 border border-danger/30 text-danger'
      ]">
        <svg v-if="toast.type === 'success'" class="w-4 h-4 flex-shrink-0" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path><polyline points="22 4 12 14.01 9 11.01"></polyline></svg>
        <svg v-else class="w-4 h-4 flex-shrink-0" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"></circle><line x1="12" y1="8" x2="12" y2="12"></line><line x1="12" y1="16" x2="12.01" y2="16"></line></svg>
        <span>{{ toast.message }}</span>
      </div>

      <div class="px-6 py-4 flex flex-col gap-4">
        <div>
          <label class="text-xs text-text-muted mb-1 block">名称 *</label>
          <input v-model="form.name" class="w-full bg-bg border border-white/10 rounded-lg px-3 py-2 text-text text-sm focus:outline-none focus:border-primary/50 transition" placeholder="例如 GitHub" />
        </div>

        <div>
          <label class="text-xs text-text-muted mb-1 block">用户名 *</label>
          <input v-model="form.username" class="w-full bg-bg border border-white/10 rounded-lg px-3 py-2 text-text text-sm focus:outline-none focus:border-primary/50 transition" placeholder="用户名或邮箱" />
        </div>

        <div>
          <label class="text-xs text-text-muted mb-1 block">密码 *</label>
          <div class="flex gap-2">
            <div class="flex-1 relative">
              <input
                v-model="form.password"
                :type="showPw ? 'text' : 'password'"
                :disabled="isEdit && passwordLocked"
                @blur="checkPasswordPwned"
                class="w-full bg-bg border border-white/10 rounded-lg px-3 py-2 pr-24 text-text text-sm font-mono-pw focus:outline-none focus:border-primary/50 transition disabled:opacity-60"
                :placeholder="isEdit && passwordLocked ? '点击右侧编辑按钮修改密码' : '请输入密码'"
              />
              <div v-if="isEdit" class="absolute right-2 top-1/2 -translate-y-1/2 flex items-center gap-0.5">
                <button v-if="passwordLocked" @click="passwordLocked = false" class="p-1 text-primary-light hover:text-primary transition" title="修改密码">
                  <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"></path><path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"></path></svg>
                </button>
                <button @click="showPw = !showPw" class="p-1 text-text-muted hover:text-text transition">
                  <svg v-if="!showPw" class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"></path><circle cx="12" cy="12" r="3"></circle></svg>
                  <svg v-else class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24"></path><line x1="1" y1="1" x2="23" y2="23"></line></svg>
                </button>
              </div>
              <button v-else @click="showPw = !showPw" class="absolute right-2 top-1/2 -translate-y-1/2 p-1 text-text-muted hover:text-text transition">
                <svg v-if="!showPw" class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"></path><circle cx="12" cy="12" r="3"></circle></svg>
                <svg v-else class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24"></path><line x1="1" y1="1" x2="23" y2="23"></line></svg>
              </button>
            </div>
            <button @click="showGenerator = !showGenerator" class="px-3 py-2 bg-primary/20 text-primary-light rounded-lg hover:bg-primary/30 transition text-sm flex items-center gap-1" title="密码生成器">
              <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 2L2 7l10 5 10-5-10-5z"></path><path d="M2 17l10 5 10-5"></path><path d="M2 12l10 5 10-5"></path></svg>
              生成
            </button>
          </div>
          <div v-if="pwnedCheck.is_pwned" class="mt-2 p-2 bg-danger/10 border border-danger/30 rounded-lg flex items-start gap-2">
            <svg class="w-4 h-4 text-danger flex-shrink-0 mt-0.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"></path><line x1="12" y1="9" x2="12" y2="13"></line><line x1="12" y1="17" x2="12.01" y2="17"></line></svg>
            <div>
              <p class="text-xs text-danger font-medium">此密码已在数据泄露中出现！</p>
              <p class="text-xs text-danger/70">出现次数: {{ pwnedCheck.breach_count }} 次，建议立即更换</p>
            </div>
          </div>
          <div v-else-if="pwnedCheck.checked && form.password" class="mt-2 p-2 bg-success/10 border border-success/30 rounded-lg flex items-center gap-2">
            <svg class="w-4 h-4 text-success" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path><polyline points="22 4 12 14.01 9 11.01"></polyline></svg>
            <p class="text-xs text-success">密码安全，未在已知数据泄露中发现</p>
          </div>
        </div>

        <div v-if="showGenerator" class="bg-bg/80 border border-white/5 rounded-xl p-4 flex flex-col gap-3">
          <div class="flex items-center justify-between">
            <span class="text-xs text-text-muted">密码长度: {{ genLength }}</span>
            <input type="range" v-model.number="genLength" min="8" max="32" class="w-32 accent-primary" />
          </div>
          <div class="flex flex-wrap gap-2">
            <label class="flex items-center gap-1.5 text-xs text-text-muted cursor-pointer">
              <input type="checkbox" v-model="genUpper" class="accent-primary" /> 大写字母
            </label>
            <label class="flex items-center gap-1.5 text-xs text-text-muted cursor-pointer">
              <input type="checkbox" v-model="genLower" class="accent-primary" /> 小写字母
            </label>
            <label class="flex items-center gap-1.5 text-xs text-text-muted cursor-pointer">
              <input type="checkbox" v-model="genDigits" class="accent-primary" /> 数字
            </label>
            <label class="flex items-center gap-1.5 text-xs text-text-muted cursor-pointer">
              <input type="checkbox" v-model="genSymbols" class="accent-primary" /> 符号
            </label>
          </div>
          <button @click="generatePw" class="w-full py-2 bg-primary/20 text-primary-light rounded-lg hover:bg-primary/30 transition text-sm">生成密码</button>
        </div>

        <div>
          <label class="text-xs text-text-muted mb-1 block">URL（可选）</label>
          <input v-model="form.url" class="w-full bg-bg border border-white/10 rounded-lg px-3 py-2 text-text text-sm focus:outline-none focus:border-primary/50 transition" placeholder="https://" />
        </div>

        <div>
          <label class="text-xs text-text-muted mb-1 block">备注</label>
          <textarea v-model="form.notes" rows="2" class="w-full bg-bg border border-white/10 rounded-lg px-3 py-2 text-text text-sm focus:outline-none focus:border-primary/50 transition resize-none" placeholder="备注信息"></textarea>
        </div>

        <div>
          <label class="text-xs text-text-muted mb-1 block">分类</label>
          <select v-model="form.category" @change="onCategoryChange" class="w-full bg-bg border border-white/10 rounded-lg px-3 py-2 text-text text-sm focus:outline-none focus:border-primary/50 transition">
            <option value="website">网站</option>
            <option value="app">应用</option>
            <option value="bank">银行卡</option>
            <option value="note">安全笔记</option>
          </select>
        </div>

        <div class="border-t border-white/5 pt-4">
          <div class="flex items-center gap-2 mb-3">
            <svg class="w-4 h-4 text-text-muted" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"></circle><polyline points="12 6 12 12 16 14"></polyline></svg>
            <span class="text-sm font-medium text-text">密码过期设置</span>
          </div>
          <div class="space-y-3 pl-6">
            <div>
              <label class="text-xs text-text-muted mb-1 block">轮换周期（天）</label>
              <div class="flex gap-2">
                <input
                  v-model.number="form.rotation_days"
                  type="number"
                  min="1"
                  placeholder="留空使用默认值"
                  class="flex-1 bg-bg border border-white/10 rounded-lg px-3 py-2 text-text text-sm focus:outline-none focus:border-primary/50 transition"
                />
                <button
                  @click="resetRotationDays"
                  type="button"
                  class="px-3 py-2 bg-surface-light/50 text-text-muted rounded-lg hover:bg-surface-light hover:text-text transition text-xs"
                  title="恢复默认值"
                >
                  默认
                </button>
              </div>
              <p v-if="defaultRotationDays" class="text-xs text-text-muted/60 mt-1">
                {{ categoryLabel }}默认轮换周期: {{ defaultRotationDays }} 天
              </p>
            </div>
            <div>
              <label class="text-xs text-text-muted mb-1 block">过期日期（可选）</label>
              <input
                v-model="form.expires_at"
                type="date"
                class="w-full bg-bg border border-white/10 rounded-lg px-3 py-2 text-text text-sm focus:outline-none focus:border-primary/50 transition"
              />
              <p class="text-xs text-text-muted/60 mt-1">
                留空则根据轮换周期自动计算
              </p>
            </div>
          </div>
        </div>

        <div v-if="isEdit && editEntry?.history && editEntry.history.length > 0" class="border-t border-white/5 pt-4">
          <div class="flex items-center justify-between mb-3">
            <div class="flex items-center gap-2">
              <svg class="w-4 h-4 text-text-muted" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"></circle><polyline points="12 6 12 12 16 14"></polyline></svg>
              <span class="text-sm font-medium text-text">密码变更历史</span>
              <span class="text-xs text-text-muted">(最近 {{ editEntry.history.length }} 条)</span>
            </div>
            <button
              v-if="!showHistory"
              @click="showHistory = true"
              class="text-xs text-primary-light hover:text-primary transition"
            >展开</button>
            <button
              v-else
              @click="showHistory = false"
              class="text-xs text-text-muted hover:text-text transition"
            >收起</button>
          </div>

          <div v-if="showHistory" class="space-y-2 max-h-64 overflow-y-auto pr-1">
            <div
              v-for="(item, index) in [...editEntry.history].reverse()"
              :key="index"
              class="relative pl-6 pb-3 border-l border-white/10 last:border-l-0 last:pb-0"
            >
              <div class="absolute left-0 top-0 -translate-x-1/2 w-3 h-3 rounded-full bg-primary/50 border-2 border-surface-light"></div>

              <div class="bg-surface-light/40 rounded-lg p-3 border border-white/5">
                <div class="flex items-center justify-between mb-2">
                  <span class="text-xs text-text-muted">{{ formatHistoryDate(item.changed_at) }}</span>
                  <div class="flex items-center gap-1">
                    <button
                      @click="toggleHistoryPassword(item, editEntry.history.length - 1 - index)"
                      class="p-1 rounded hover:bg-surface-lighter text-text-muted hover:text-text transition"
                      :title="visibleHistory.has(editEntry.history.length - 1 - index) ? '隐藏密码' : '查看密码'"
                    >
                      <svg v-if="!visibleHistory.has(editEntry.history.length - 1 - index)" class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"></path><circle cx="12" cy="12" r="3"></circle></svg>
                      <svg v-else class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24"></path><line x1="1" y1="1" x2="23" y2="23"></line></svg>
                    </button>
                    <button
                      @click="copyHistoryPassword(item, editEntry.history.length - 1 - index)"
                      class="p-1 rounded hover:bg-surface-lighter text-text-muted hover:text-primary-light transition"
                      title="复制此密码"
                    >
                      <svg class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="9" y="9" width="13" height="13" rx="2"></rect><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path></svg>
                    </button>
                    <button
                      @click="confirmRestore(editEntry.history.length - 1 - index, item.changed_at)"
                      class="p-1 rounded hover:bg-primary/20 text-text-muted hover:text-primary-light transition"
                      title="恢复到此版本"
                    >
                      <svg class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="1 4 1 10 7 10"></polyline><path d="M3.51 15a9 9 0 1 0 2.13-9.36L1 10"></path></svg>
                    </button>
                  </div>
                </div>
                <div class="bg-bg/60 rounded px-3 py-2 flex items-center justify-between">
                  <span v-if="!visibleHistory.has(editEntry.history.length - 1 - index)" class="font-mono-pw text-text-muted text-sm tracking-widest">••••••••••</span>
                  <span v-else class="font-mono-pw text-text text-sm break-all">{{ decryptedHistory[editEntry.history.length - 1 - index] || '解密中...' }}</span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div class="px-6 py-4 border-t border-white/5 flex justify-end gap-3">
        <button @click="$emit('close')" class="px-4 py-2 rounded-lg text-text-muted hover:bg-surface-lighter transition text-sm">取消</button>
        <button
          @click="handleSave"
          :disabled="saving || !form.name || !form.username || (!isEdit && !form.password) || (isEdit && !passwordLocked && !form.password)"
          class="px-6 py-2 rounded-lg bg-gradient-to-r from-primary to-indigo-500 text-white font-medium text-sm hover:from-primary-dark hover:to-indigo-600 disabled:opacity-40 disabled:cursor-not-allowed transition-all shadow-lg shadow-primary/20 flex items-center gap-2"
        >
          <svg v-if="saving" class="w-4 h-4 animate-spin" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 12a9 9 0 1 1-6.219-8.56"></path></svg>
          {{ saving ? '保存中...' : '保存' }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive, computed, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";

const isTauri = typeof window !== "undefined" && !!window.__TAURI_INTERNALS__;

const props = defineProps({
  editEntry: { type: Object, default: null }
});

const emit = defineEmits(["close", "saved"]);

const isEdit = computed(() => !!props.editEntry);
const showPw = ref(false);
const showGenerator = ref(false);
const saving = ref(false);
const passwordLocked = ref(true);
const showHistory = ref(true);
const visibleHistory = ref(new Set());
const decryptedHistory = ref({});

const toast = reactive({
  show: false,
  type: "success",
  message: "",
  timer: null
});

function showToast(type, message) {
  if (toast.timer) clearTimeout(toast.timer);
  toast.type = type;
  toast.message = message;
  toast.show = true;
  toast.timer = setTimeout(() => {
    toast.show = false;
  }, 3000);
}

const pwnedCheck = reactive({
  checked: false,
  is_pwned: false,
  breach_count: 0
});

const defaultRotationDaysMap = {
  website: 180,
  app: 180,
  bank: 90,
  note: 365
};

const categoryLabelMap = {
  website: "网站",
  app: "应用",
  bank: "银行卡",
  note: "安全笔记"
};

const form = reactive({
  name: props.editEntry?.name || "",
  username: props.editEntry?.username || "",
  password: "",
  url: props.editEntry?.url || "",
  notes: props.editEntry?.notes || "",
  category: props.editEntry?.category || "website",
  rotation_days: props.editEntry?.rotation_days || null,
  expires_at: props.editEntry?.expires_at ? formatDateForInput(props.editEntry.expires_at) : ""
});

function formatDateForInput(dateStr) {
  try {
    const date = new Date(dateStr);
    return date.toISOString().split("T")[0];
  } catch {
    return "";
  }
}

const defaultRotationDays = computed(() => {
  return defaultRotationDaysMap[form.category] || null;
});

const categoryLabel = computed(() => {
  return categoryLabelMap[form.category] || form.category;
});

function onCategoryChange() {
  if (!form.rotation_days) {
    form.rotation_days = defaultRotationDaysMap[form.category] || null;
  }
}

function resetRotationDays() {
  form.rotation_days = defaultRotationDaysMap[form.category] || null;
  form.expires_at = "";
}

const genLength = ref(16);
const genUpper = ref(true);
const genLower = ref(true);
const genDigits = ref(true);
const genSymbols = ref(false);

async function generatePw() {
  if (!isTauri) {
    showToast("error", "Tauri 运行时不可用");
    return;
  }
  try {
    const pw = await invoke("generate_password", {
      length: genLength.value,
      useUppercase: genUpper.value,
      useLowercase: genLower.value,
      useDigits: genDigits.value,
      useSymbols: genSymbols.value
    });
    form.password = pw;
    checkPasswordPwned();
  } catch (e) {
    console.error(e);
    showToast("error", "密码生成失败: " + (e.message || e));
  }
}

function formatHistoryDate(dateStr) {
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

async function toggleHistoryPassword(item, realIndex) {
  const newSet = new Set(visibleHistory.value);
  if (newSet.has(realIndex)) {
    newSet.delete(realIndex);
    visibleHistory.value = newSet;
    return;
  }
  if (!isTauri) {
    showToast("error", "Tauri 运行时不可用");
    return;
  }
  try {
    const pw = await invoke("decrypt_history_password", {
      entryId: props.editEntry.id,
      encrypted: item.encrypted_password
    });
    decryptedHistory.value[realIndex] = pw;
    newSet.add(realIndex);
    visibleHistory.value = newSet;
  } catch (e) {
    console.error("decrypt history failed:", e);
    showToast("error", "解密历史密码失败");
  }
}

async function copyHistoryPassword(item, realIndex) {
  if (!isTauri) return;
  try {
    let pw = decryptedHistory.value[realIndex];
    if (!pw) {
      pw = await invoke("decrypt_history_password", {
        entryId: props.editEntry.id,
        encrypted: item.encrypted_password
      });
      decryptedHistory.value[realIndex] = pw;
      const newSet = new Set(visibleHistory.value);
      newSet.add(realIndex);
      visibleHistory.value = newSet;
    }
    const { writeText } = await import("@tauri-apps/plugin-clipboard-manager");
    await writeText(pw);
    showToast("success", "历史密码已复制到剪贴板");
    setTimeout(async () => {
      try {
        await writeText("");
      } catch {}
    }, 15000);
  } catch (e) {
    console.error("copy history failed:", e);
    showToast("error", "复制失败");
  }
}

async function confirmRestore(realIndex, changedAt) {
  const dateStr = formatHistoryDate(changedAt);
  if (!confirm(`确定要将密码恢复到 ${dateStr} 的版本吗？\n\n当前密码将被保存到历史记录中。`)) {
    return;
  }
  if (!isTauri) {
    showToast("error", "Tauri 运行时不可用");
    return;
  }
  saving.value = true;
  try {
    await invoke("restore_history_password", {
      entryId: props.editEntry.id,
      historyIndex: realIndex
    });
    showToast("success", "密码已恢复到历史版本");
    setTimeout(() => {
      emit("saved");
    }, 600);
  } catch (e) {
    console.error("restore failed:", e);
    const msg = (typeof e === "string" ? e : (e?.message || String(e)));
    if (msg.includes("not unlocked")) {
      showToast("error", "密码库已锁定，请重新解锁后再试");
    } else {
      showToast("error", "恢复失败: " + msg);
    }
  } finally {
    saving.value = false;
  }
}

async function checkPasswordPwned() {
  if (!isTauri || !form.password || form.password.length < 4) {
    pwnedCheck.checked = false;
    pwnedCheck.is_pwned = false;
    pwnedCheck.breach_count = 0;
    return;
  }
  try {
    const [is_pwned, breach_count] = await invoke("check_single_password", {
      password: form.password
    });
    pwnedCheck.checked = true;
    pwnedCheck.is_pwned = is_pwned;
    pwnedCheck.breach_count = breach_count;
  } catch (e) {
    console.error("pwned check failed", e);
    pwnedCheck.checked = false;
  }
}

async function handleSave() {
  if (!isTauri) {
    showToast("error", "Tauri 运行时不可用");
    return;
  }
  if (!form.name.trim() || !form.username.trim()) {
    showToast("error", "请填写名称和用户名");
    return;
  }
  if (!isEdit.value && !form.password) {
    showToast("error", "请输入密码");
    return;
  }
  if (isEdit.value && !passwordLocked.value && !form.password) {
    showToast("error", "请输入新密码，或点击编辑按钮回退");
    return;
  }

  saving.value = true;
  try {
    const entry = {
      id: props.editEntry?.id || crypto.randomUUID(),
      name: form.name.trim(),
      username: form.username.trim(),
      encrypted_password: form.password,
      url: form.url.trim(),
      notes: form.notes.trim(),
      category: form.category,
      created_at: props.editEntry?.created_at || new Date().toISOString().split("T")[0],
      is_pwned: pwnedCheck.is_pwned,
      breach_count: pwnedCheck.breach_count,
      last_pwned_check: pwnedCheck.checked ? new Date().toISOString() : (props.editEntry?.last_pwned_check || ""),
      rotation_days: form.rotation_days ? Number(form.rotation_days) : null,
      expires_at: form.expires_at ? new Date(form.expires_at).toISOString() : null
    };
    if (isEdit.value) {
      await invoke("update_entry", { entry });
      showToast("success", "条目已更新");
    } else {
      await invoke("add_entry", { entry });
      showToast("success", "条目已保存");
    }
    setTimeout(() => {
      emit("saved");
    }, 600);
  } catch (e) {
    console.error("save failed:", e);
    const msg = (typeof e === "string" ? e : (e?.message || String(e)));
    if (msg.includes("not unlocked")) {
      showToast("error", "密码库已锁定，请重新解锁后再试");
    } else {
      showToast("error", "保存失败: " + msg);
    }
  } finally {
    saving.value = false;
  }
}

watch(() => props.editEntry, (newVal) => {
  visibleHistory.value = new Set();
  decryptedHistory.value = {};
  if (newVal) {
    form.name = newVal.name || "";
    form.username = newVal.username || "";
    form.password = "";
    form.url = newVal.url || "";
    form.notes = newVal.notes || "";
    form.category = newVal.category || "website";
    form.rotation_days = newVal.rotation_days || null;
    form.expires_at = newVal.expires_at ? formatDateForInput(newVal.expires_at) : "";
    passwordLocked.value = !newVal.is_pwned;
    pwnedCheck.checked = !!newVal.last_pwned_check;
    pwnedCheck.is_pwned = !!newVal.is_pwned;
    pwnedCheck.breach_count = newVal.breach_count || 0;
  } else {
    form.name = "";
    form.username = "";
    form.password = "";
    form.url = "";
    form.notes = "";
    form.category = "website";
    form.rotation_days = null;
    form.expires_at = "";
    passwordLocked.value = false;
    pwnedCheck.checked = false;
    pwnedCheck.is_pwned = false;
    pwnedCheck.breach_count = 0;
  }
}, { immediate: true });
</script>
