<template>
  <div class="w-48 h-full bg-surface/50 border-r border-white/5 flex flex-col py-4">
    <div class="px-4 mb-4">
      <h2 class="text-xs font-semibold text-text-muted uppercase tracking-widest">分类</h2>
    </div>
    <nav class="flex flex-col gap-1 px-2">
      <button
        v-for="cat in categories"
        :key="cat.key"
        @click="$emit('select', cat.key)"
        :class="[
          'flex items-center justify-between px-3 py-2.5 rounded-lg text-sm transition-all duration-150',
          active === cat.key
            ? 'bg-primary text-white shadow-md shadow-primary/20'
            : 'text-text-muted hover:bg-surface-light hover:text-text'
        ]"
      >
        <span class="flex items-center gap-2">
          <span v-html="cat.icon" class="w-4 h-4"></span>
          {{ cat.label }}
        </span>
        <span
          :class="[
            'text-xs px-1.5 py-0.5 rounded-full',
            active === cat.key ? 'bg-white/20' : 'bg-surface-lighter'
          ]"
        >{{ counts[cat.key] || 0 }}</span>
      </button>
    </nav>
  </div>
</template>

<script setup>
defineProps({
  active: { type: String, default: "all" },
  counts: { type: Object, default: () => ({}) }
});

defineEmits(["select"]);

const categories = [
  { key: "all", label: "全部", icon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="3" y="3" width="7" height="7"/><rect x="14" y="3" width="7" height="7"/><rect x="3" y="14" width="7" height="7"/><rect x="14" y="14" width="7" height="7"/></svg>' },
  { key: "website", label: "网站", icon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><line x1="2" y1="12" x2="22" y2="12"/><path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"/></svg>' },
  { key: "app", label: "应用", icon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="5" y="2" width="14" height="20" rx="2"/><line x1="12" y1="18" x2="12.01" y2="18"/></svg>' },
  { key: "bank", label: "银行卡", icon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="1" y="4" width="22" height="16" rx="2"/><line x1="1" y1="10" x2="23" y2="10"/></svg>' },
  { key: "note", label: "安全笔记", icon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/><line x1="16" y1="13" x2="8" y2="13"/><line x1="16" y1="17" x2="8" y2="17"/></svg>' }
];
</script>
