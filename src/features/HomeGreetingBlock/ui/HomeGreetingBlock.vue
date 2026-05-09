<template>
  <v-card v-if="user" :title="t('home.greetingTitle')">
    {{ t('home.greetingConnected', { username: user.username }) }}
  </v-card>
</template>

<script setup lang="ts">
import { useGetUserQuery } from '@/entities/user';
import { useI18n } from '@/shared/lib/i18n';

const { t } = useI18n();
const { user } = useGetUserQuery();
</script>
