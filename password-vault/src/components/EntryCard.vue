<template>
  <div :class="[
    'border rounded-xl px-5 py-4 hover:border-primary/30 transition-all duration-200 group animate-fade-in',
    entry.is_pwned
      ? 'bg-danger/10 border-danger/30'
      : 'bg-surface-light/60 border-white/5'
  ]">
    <div class="flex items-start justify-between gap-4">
      <div class="flex-1 min-w-0">
        <div class="flex items-center gap-2 mb-1 flex-wrap">
          <h3 class="text-text font-semibold text-base truncate">{{ entry.name }}</h3>
          <span class="text-xs px-2 py-0.5 rounded-full bg-primary/15 text-primary-light">{{ categoryLabel }}</span>
          <span v-if="entry.is_pwned" class="text-xs px-2 py-0.5 rounded-full bg-danger/20 text-danger flex items-center gap-1" title="该密码已在数据泄露中出现">
            <svg class="w-3 h-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/><line x1="12" y1="9" x2="12" y2="13"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg>
            已泄露 {{ entry.breach_count > 0 ? `(${entry.breach_count}次)` : '' }}
          </span>
        </div>
        <p class="text-text-muted text-sm truncate">{{ entry.username }}</p>
      </div>
      <div class="flex items-center gap-2 opacity-0 group-hover:opacity-100 transition-opacity">
        <button @click="$emit('edit', entry)" class="p-1.5 rounded-lg hover:bg-surface-lighter text-text-muted hover:text-text transition" title="编辑">
          <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/><path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"/></svg>
        </button>
        <button @click="handleCopy" class="p-1.5 rounded-lg hover:bg-surface-lighter text-text-muted hover:text-primary-light transition" title="复制密码">
          <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="9" y="9" width="13" height="13" rx="2"/><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/></svg>
        </button>
      </div>
    </div>
    <div class="flex items-center gap-2 mt-3">
      <div class="flex-1 bg-bg/50 rounded-lg px-3 py-2 flex items-center justify-between">
        <span v-if="!showPassword" class="font-mono-pw text-text-muted text-sm tracking-widest">••••••••••</span>
        <span v-else class="font-mono-pw text-text text-sm">{{ decryptedPassword || '解密中...' }}</span>
        <button @click="togglePassword" class="ml-2 p-1 rounded hover:bg-surface-lighter text-text-muted hover:text-text transition">
          <svg v-if="!showPassword" class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/><circle cx="12" cy="12" r="3"/></svg>
          <svg v-else class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24"/><line x1="1" y1="1" x2="23" y2="23"/></svg>
        </button>
      </div>
    </div>
    <div v-if="entry.is_pwned" class="mt-3 flex items-center gap-2">
      <button @click="$emit('edit', entry)" class="flex-1 py-2 px-3 bg-danger/20 hover:bg-danger/30 text-danger rounded-lg text-sm font-medium transition flex items-center justify-center gap-1.5">
        <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/><path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"/></svg>
        立即修改密码
      </button>
    </div>
    <div class="flex items-center gap-3 mt-2 text-xs text-text-muted/60">
      <span>{{ entry.created_at }}</span>
      <span v-if="entry.last_pwned_check" class="ml-auto">上次检测: {{ formatDate(entry.last_pwned_check) }}</span>
    </div>
  </div>
</template>

<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";

const props = defineProps({
  entry: { type: Object, required: true }
});

defineEmits(["edit"]);

const showPassword = ref(false);
const decryptedPassword = ref("");

const categoryMap = {
  website: "网站",
  app: "应用",
  bank: "银行卡",
  note: "安全笔记"
};

const categoryLabel = categoryMap[props.entry.category] || props.entry.category;

async function togglePassword() {
  if (!showPassword.value) {
    try {
      decryptedPassword.value = await invoke("decrypt_password", {
        entryId: props.entry.id,
        encrypted: props.entry.encrypted_password
      });
    } catch (e) {
      decryptedPassword.value = "解密失败";
    }
  }
  showPassword.value = !showPassword.value;
}

async function handleCopy() {
  try {
    const pw = await invoke("decrypt_password", {
      entryId: props.entry.id,
      encrypted: props.entry.encrypted_password
    });
    await writeText(pw);
    setTimeout(async () => {
      try {
        await writeText("");
      } catch {}
    }, 15000);
  } catch (e) {
    console.error("copy failed", e);
  }
}

function formatDate(dateStr) {
  try {
    const date = new Date(dateStr);
    return date.toLocaleDateString('zh-CN', {
      year: 'numeric',
      month: '2-digit',
      day: '2-digit'
    });
  } catch {
    return dateStr;
  }
}
</script>
