<template>
  <v-card v-if="userStore.user" class="h-100" :title="`Wellcome, ${userStore.user.username}!`">
  </v-card>
</template>

<script setup lang="ts">
import { useUserStore } from '@/entities/user';

const userStore = useUserStore();
</script>
