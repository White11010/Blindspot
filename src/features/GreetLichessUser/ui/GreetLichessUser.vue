<template>
  <v-card v-if="userStore.user">
    <p>Wellcome, {{ userStore.user.username }}!</p>
  </v-card>
</template>

<script setup lang="ts">
import { useUserStore } from '@/entities/user';

const userStore = useUserStore();
</script>
