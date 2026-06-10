<template>
  <div class="fixed inset-0 bg-black/60 backdrop-blur-sm flex items-center justify-center z-50" @click.self="$emit('close')">
    <div class="bg-surface border border-white/10 rounded-2xl w-[440px] max-h-[80vh] overflow-y-auto shadow-2xl animate-fade-in">
      <div class="px-6 py-4 border-b border-white/5 flex items-center justify-between">
        <h2 class="text-lg font-semibold text-text">{{ isEdit ? '编辑条目' : '新建条目' }}</h2>
        <button @click="$emit('close')" class="p-1 rounded-lg hover:bg-surface-lighter text-text-muted hover:text-text transition">
          <svg class="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>
        </button>
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
              <input v-model="form.password" :type="showPw ? 'text' : 'password'" @blur="checkPasswordPwned" class="w-full bg-bg border border-white/10 rounded-lg px-3 py-2 pr-10 text-text text-sm font-mono-pw focus:outline-none focus:border-primary/50 transition" placeholder="密码" />
              <button @click="showPw = !showPw" class="absolute right-2 top-1/2 -translate-y-1/2 p-1 text-text-muted hover:text-text transition">
                <svg v-if="!showPw" class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/><circle cx="12" cy="12" r="3"/></svg>
                <svg v-else class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24"/><line x1="1" y1="1" x2="23" y2="23"/></svg>
              </button>
            </div>
            <button @click="showGenerator = !showGenerator" class="px-3 py-2 bg-primary/20 text-primary-light rounded-lg hover:bg-primary/30 transition text-sm flex items-center gap-1" title="密码生成器">
              <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 2L2 7l10 5 10-5-10-5z"/><path d="M2 17l10 5 10-5"/><path d="M2 12l10 5 10-5"/></svg>
              生成
            </button>
          </div>
          <div v-if="pwnedCheck.is_pwned" class="mt-2 p-2 bg-danger/10 border border-danger/30 rounded-lg flex items-start gap-2">
            <svg class="w-4 h-4 text-danger flex-shrink-0 mt-0.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/><line x1="12" y1="9" x2="12" y2="13"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg>
            <div>
              <p class="text-xs text-danger font-medium">此密码已在数据泄露中出现！</p>
              <p class="text-xs text-danger/70">出现次数: {{ pwnedCheck.breach_count }} 次，建议立即更换</p>
            </div>
          </div>
          <div v-else-if="pwnedCheck.checked && form.password" class="mt-2 p-2 bg-success/10 border border-success/30 rounded-lg flex items-center gap-2">
            <svg class="w-4 h-4 text-success" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg>
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
          <select v-model="form.category" class="w-full bg-bg border border-white/10 rounded-lg px-3 py-2 text-text text-sm focus:outline-none focus:border-primary/50 transition">
            <option value="website">网站</option>
            <option value="app">应用</option>
            <option value="bank">银行卡</option>
            <option value="note">安全笔记</option>
          </select>
        </div>
      </div>

      <div class="px-6 py-4 border-t border-white/5 flex justify-end gap-3">
        <button @click="$emit('close')" class="px-4 py-2 rounded-lg text-text-muted hover:bg-surface-lighter transition text-sm">取消</button>
        <button @click="handleSave" :disabled="!form.name || !form.username || !form.password" class="px-6 py-2 rounded-lg bg-gradient-to-r from-primary to-indigo-500 text-white font-medium text-sm hover:from-primary-dark hover:to-indigo-600 disabled:opacity-40 disabled:cursor-not-allowed transition-all shadow-lg shadow-primary/20">保存</button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";

const props = defineProps({
  editEntry: { type: Object, default: null }
});

const emit = defineEmits(["close", "saved"]);

const isEdit = computed(() => !!props.editEntry);
const showPw = ref(false);
const showGenerator = ref(false);

const pwnedCheck = reactive({
  checked: false,
  is_pwned: false,
  breach_count: 0
});

const form = reactive({
  name: props.editEntry?.name || "",
  username: props.editEntry?.username || "",
  password: "",
  url: props.editEntry?.url || "",
  notes: props.editEntry?.notes || "",
  category: props.editEntry?.category || "website"
});

const genLength = ref(16);
const genUpper = ref(true);
const genLower = ref(true);
const genDigits = ref(true);
const genSymbols = ref(false);

async function generatePw() {
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
  }
}

async function checkPasswordPwned() {
  if (!form.password || form.password.length < 4) {
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
  try {
    const entry = {
      id: props.editEntry?.id || crypto.randomUUID(),
      name: form.name,
      username: form.username,
      encrypted_password: form.password,
      url: form.url,
      notes: form.notes,
      category: form.category,
      created_at: props.editEntry?.created_at || new Date().toISOString().split("T")[0]
    };
    if (isEdit.value) {
      await invoke("update_entry", { entry });
    } else {
      await invoke("add_entry", { entry });
    }
    emit("saved");
  } catch (e) {
    console.error(e);
  }
}
</script>
