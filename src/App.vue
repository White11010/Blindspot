<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<template>
  <main class="container">
    <router-view></router-view>
  </main>
</template>

<style scoped>
.container {
  padding: 2rem;
}
</style>
<style>
.button--medium {
  height: 3rem;
}
</style>