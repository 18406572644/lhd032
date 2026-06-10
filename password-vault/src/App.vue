<template>
  <div class="h-screen w-screen flex flex-col rounded-xl overflow-hidden border border-white/5">
    <div class="drag-region h-8 w-full flex-shrink-0" data-tauri-drag-region></div>
    <UnlockPage v-if="!unlocked" @unlocked="unlocked = true" />
    <MainInterface v-else />
  </div>
</template>

<script setup>
import { ref } from "vue";
import UnlockPage from "./components/UnlockPage.vue";
import MainInterface from "./components/MainInterface.vue";

const unlocked = ref(false);
</script>
