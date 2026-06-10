<template>
  <div class="flex-1 flex items-center justify-center bg-bg">
    <div class="flex flex-col items-center gap-6 w-80">
      <div class="relative">
        <div class="absolute inset-0 rounded-full bg-primary/20 animate-breathe"></div>
        <svg class="relative w-24 h-24 text-primary" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
          <rect x="3" y="11" width="18" height="11" rx="2" ry="2" />
          <path d="M7 11V7a5 5 0 0 1 10 0v4" />
          <circle cx="12" cy="16.5" r="1.5" class="animate-breathe" fill="currentColor" stroke="none" />
          <line x1="12" y1="18" x2="12" y2="20" stroke-width="2" class="animate-breathe" />
        </svg>
      </div>

      <h1 class="text-2xl font-semibold text-text tracking-wide">PasswordVault</h1>
      <p class="text-text-muted text-sm -mt-4">{{ isInit ? '输入主密码以解锁' : '首次使用，请设置主密码' }}</p>

      <div v-if="error" class="w-full bg-danger/10 border border-danger/30 text-danger text-sm px-4 py-2 rounded-lg">
        {{ error }}
      </div>

      <div class="w-full flex flex-col gap-3">
        <input
          v-model="password"
          type="password"
          :placeholder="isInit ? '输入主密码' : '设置主密码'"
          class="w-full bg-surface border border-white/10 rounded-lg px-4 py-3 text-text placeholder:text-text-muted/50 focus:outline-none focus:border-primary/50 focus:ring-1 focus:ring-primary/30 transition"
          @keyup.enter="handleSubmit"
        />

        <input
          v-if="!isInit"
          v-model="confirmPassword"
          type="password"
          placeholder="确认主密码"
          class="w-full bg-surface border border-white/10 rounded-lg px-4 py-3 text-text placeholder:text-text-muted/50 focus:outline-none focus:border-primary/50 focus:ring-1 focus:ring-primary/30 transition"
          @keyup.enter="handleSubmit"
        />

        <button
          @click="handleSubmit"
          :disabled="loading"
          class="w-full py-3 rounded-lg font-medium text-white bg-gradient-to-r from-primary to-indigo-500 hover:from-primary-dark hover:to-indigo-600 disabled:opacity-50 disabled:cursor-not-allowed transition-all duration-200 shadow-lg shadow-primary/20"
        >
          {{ loading ? '处理中...' : (isInit ? '解锁' : '创建密码库') }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

const emit = defineEmits(["unlocked"]);

const isInit = ref(true);
const password = ref("");
const confirmPassword = ref("");
const error = ref("");
const loading = ref(false);

onMounted(async () => {
  try {
    isInit.value = await invoke("is_vault_initialized");
  } catch (e) {
    isInit.value = false;
  }
});

async function handleSubmit() {
  error.value = "";
  loading.value = true;

  try {
    if (!isInit.value) {
      if (password.value.length < 6) {
        error.value = "主密码至少需要6个字符";
        return;
      }
      if (password.value !== confirmPassword.value) {
        error.value = "两次输入的密码不一致";
        return;
      }
      await invoke("setup_master_password", { password: password.value });
      emit("unlocked");
    } else {
      const valid = await invoke("verify_master_password", { password: password.value });
      if (valid) {
        emit("unlocked");
      } else {
        error.value = "主密码错误";
      }
    }
  } catch (e) {
    error.value = e.toString();
  } finally {
    loading.value = false;
  }
}
</script>
