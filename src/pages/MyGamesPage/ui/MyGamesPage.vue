<script setup lang="ts">
import {invoke} from "@tauri-apps/api/core";
import {ref} from "vue";

const data = ref<any>(null)
const loadPlayerData = async () => {
  // Токен автоматически подставится на стороне Rust
  data.value = await invoke('fetch_lichess_player')
  console.log(data)
}
</script>

<template>
  <v-card
      title="Expore your games"
  >
    <v-card-text>
      <VBtn @click="loadPlayerData">Load your games</VBtn>
    </v-card-text>
  </v-card>
  <div v-if="data">{{JSON.stringify(data)}}</div>
  <router-link to="/">Home</router-link>
</template>

<style scoped>

</style>