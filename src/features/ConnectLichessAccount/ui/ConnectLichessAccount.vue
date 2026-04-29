<script setup lang="ts">
import {onMounted, ref} from 'vue'
import { invoke } from "@tauri-apps/api/core";

const isLoading = ref(true)

const tokenInput = ref<null | string>(null)
const saveToken = async () => {
  isLoading.value = true
  try {
    // Вызываем Rust-команду и передаем ей имя пользователя
    await invoke('save_token', { token: tokenInput.value })
    token.value = tokenInput.value
  } catch (e) {
    console.error("Ошибка при вызове Rust-команды:", e)
  } finally {
    isLoading.value = false
  }
}

function onSubmit () {
  console.log('submit')
  if (tokenInput.value) {
    saveToken()
  }
}

const token = ref<string | null>(null)
const loadStoredToken = async () => {
  try {
    const storedToken = await invoke('load_token')
    if (storedToken) {
      token.value = storedToken as string
    }
  } catch (err) {
    console.error('Ошибка загрузки токена:', err)
  } finally {
    isLoading.value = false
  }
}
onMounted(() => {
  loadStoredToken()
})
</script>

<template>
<v-card
    v-if="!isLoading && !token"
  title="Connect your chess account"
  subtitle="Import your games from Lichess to start querying"
>
  <v-card-text>
    <form class="d-flex ga-4 mt-4" @submit.prevent="onSubmit">
      <VTextField v-model="tokenInput" density="comfortable" variant="outlined" hide-details></VTextField>
      <VBtn type="submit" class="button--medium" size="large">Connect with Lichess</VBtn>
    </form>
  </v-card-text>
</v-card>
  <div v-else>
    Lichess is connected, {{token}}
  </div>
</template>

<style scoped>

</style>