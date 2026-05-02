<template>
  <p v-if="appStore.status === 'loading'">Loading</p>
  <greet-lichess-user v-if="appStore.status === 'ready' && userStore.user" />
  <connect-lichess-account v-if="appStore.status === 'ready' && userStore.user === null" />
</template>

<script setup lang="ts">
import { useAppStore } from '@/entities/app';
import { useUserStore } from '@/entities/user';
import { ConnectLichessAccount } from '@/features/ConnectLichessAccount';
import { GreetLichessUser } from '@/features/GreetLichessUser';

const appStore = useAppStore();
const userStore = useUserStore();
appStore.bootstrap();
</script>
