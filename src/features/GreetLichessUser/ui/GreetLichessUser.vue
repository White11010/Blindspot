<template>
  <v-card v-if="userStore.user" class="h-100" :title="t('home.greetTitle', { username: userStore.user.username })">
  </v-card>
</template>

<script setup lang="ts">
import { useUserStore } from '@/entities/user';
import { useI18n } from '@/shared/lib/i18n';

const { t } = useI18n();
const userStore = useUserStore();
</script>
