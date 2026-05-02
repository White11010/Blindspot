<template>
  <v-layout class="rounded rounded-md border" style="height: 100vh;">
    <v-navigation-drawer permanent :width="240">
      <v-list-item class="pa-4" title="ChessLens" subtitle="Analytics platform" @click="router?.push('/')"></v-list-item>
      <v-divider></v-divider>
      <v-list-item class="mt-4" :active="route?.name === 'Home'" link title="Home" prepend-icon="mdi-home" @click="router?.push('/')"></v-list-item>
      <v-list-item class="mt-2" :active="route?.name === 'MyGames'" link title="My games" prepend-icon="mdi-controller" @click="router?.push('/my-games')"></v-list-item>
      <v-list-item class="mt-2" :active="route?.name === 'Insights'" link title="Insights" prepend-icon="mdi-creation"></v-list-item>
    </v-navigation-drawer>
    <v-main class="d-flex align-center justify-center">
      <router-view></router-view>
    </v-main>
  </v-layout>
</template>

<script setup lang="ts">
import { useRoute, useRouter } from 'vuetify/lib/composables/router.mjs';

const route = useRoute()
const router = useRouter()
</script>

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