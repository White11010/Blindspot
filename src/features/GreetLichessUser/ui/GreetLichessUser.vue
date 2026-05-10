<template>
  <v-card v-if="userStore.user" class="h-100" :title="t('home.greetTitle', { username: userStore.user.username })">
  </v-card>
</template>

<script setup lang="ts">
// Feature slice: encapsulates one user flow or form; parent pages/widgets compose it and pass props/events.

import { useUserStore } from '@/entities/user';
import { useI18n } from '@/shared/lib/i18n';

const { t } = useI18n();
const userStore = useUserStore();
</script>
