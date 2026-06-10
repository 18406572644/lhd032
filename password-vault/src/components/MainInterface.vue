<template>
  <div class="flex-1 flex h-full">
    <CategoryNav :active="activeCategory" :counts="categoryCounts" @select="activeCategory = $event" />

    <div class="flex-1 flex flex-col h-full overflow-hidden">
      <div class="px-6 py-3 border-b border-white/5">
        <div class="relative">
          <svg class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-text-muted" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/></svg>
          <input
            v-model="searchQuery"
            type="text"
            placeholder="搜索名称或用户名..."
            class="w-full bg-surface-light/50 border border-white/5 rounded-lg pl-10 pr-4 py-2.5 text-text text-sm placeholder:text-text-muted/50 focus:outline-none focus:border-primary/30 focus:ring-1 focus:ring-primary/20 transition"
          />
        </div>
      </div>

      <div class="flex-1 overflow-y-auto px-6 py-4">
        <div v-if="filteredEntries.length === 0" class="flex flex-col items-center justify-center h-full text-text-muted/50">
          <svg class="w-16 h-16 mb-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1"><rect x="3" y="11" width="18" height="11" rx="2"/><path d="M7 11V7a5 5 0 0 1 10 0v4"/></svg>
          <p class="text-sm">暂无密码条目</p>
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
        <svg class="w-6 h-6" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>
      </button>
    </div>

    <EntryForm
      v-if="showForm"
      :editEntry="editingEntry"
      @close="closeForm"
      @saved="onSaved"
    />
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import CategoryNav from "./CategoryNav.vue";
import EntryCard from "./EntryCard.vue";
import EntryForm from "./EntryForm.vue";

const entries = ref([]);
const activeCategory = ref("all");
const searchQuery = ref("");
const showForm = ref(false);
const editingEntry = ref(null);

const categoryCounts = computed(() => {
  const counts = { all: entries.value.length };
  for (const e of entries.value) {
    counts[e.category] = (counts[e.category] || 0) + 1;
  }
  return counts;
});

const filteredEntries = computed(() => {
  let list = entries.value;
  if (activeCategory.value !== "all") {
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
  try {
    entries.value = await invoke("get_entries");
  } catch (e) {
    console.error(e);
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

onMounted(loadEntries);
</script>
