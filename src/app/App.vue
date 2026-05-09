<template>
  <v-layout class="rounded rounded-md border" style="height: 100vh">
    <v-navigation-drawer permanent :width="240">
      <v-list-item
        class="pa-4"
        :title="t('layout.brandTitle')"
        :subtitle="t('layout.brandSubtitle')"
        @click="router?.push('/')"
      ></v-list-item>
      <v-divider></v-divider>
      <v-list-item
        class="mt-4"
        :active="route?.name === 'Home'"
        link
        :title="t('layout.navHome')"
        prepend-icon="mdi-home"
        @click="router?.push('/')"
      ></v-list-item>
      <v-list-item
        class="mt-2"
        :active="route?.name === 'Insights'"
        link
        :title="t('layout.navInsights')"
        prepend-icon="mdi-creation"
        @click="router?.push('/insights')"
      ></v-list-item>
      <v-list-item
        class="mt-2"
        :active="route?.name === 'MyGames' || route?.name === 'GameDetails'"
        link
        :title="t('layout.navMyGames')"
        prepend-icon="mdi-table-large"
        @click="router?.push('/my-games')"
      ></v-list-item>
      <!-- <v-list-item
        class="mt-2"
        :active="route?.name === 'AnalizeBoard'"
        link
        title="Analize Board"
        prepend-icon="mdi-microscope"
        @click="router?.push('/analize-board')"
      ></v-list-item> -->

      <template #append>
        <v-list-item
          class="mb-4"
          :active="route?.name === 'Settings'"
          link
          :title="t('layout.navSettings')"
          prepend-icon="mdi-cog"
          @click="router?.push('/settings')"
        ></v-list-item>
      </template>
    </v-navigation-drawer>
    <v-main class="d-flex align-center justify-center">
      <router-view></router-view>
    </v-main>
  </v-layout>
</template>

<script setup lang="ts">
import {
  useBackgroundGameAnalysisBridge,
  useBackgroundGameAnalysisOrchestration,
} from '@/app/init/initBackgroundGameAnalysis';
import { useLocaleStore } from '@/entities/locale';
import { useI18n } from '@/shared/lib/i18n';
import { storeToRefs } from 'pinia';
import { watch } from 'vue';
import { useLocale } from 'vuetify';
import { useRoute, useRouter } from 'vuetify/lib/composables/router.mjs';

useBackgroundGameAnalysisBridge();
useBackgroundGameAnalysisOrchestration();

const { t } = useI18n();
const route = useRoute();
const router = useRouter();

const vuetifyLocale = useLocale();
const { locale: appLocale } = storeToRefs(useLocaleStore());
watch(
  appLocale,
  (code) => {
    vuetifyLocale.current.value = code;
  },
  { immediate: true },
);
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
.button-width-12 {
  width: 12rem;
}

:root {
  --app-font-scale: 1;
}

html {
  font-size: calc(16px * var(--app-font-scale));

  scrollbar-gutter: stable;

  scrollbar-width: thin;
  scrollbar-color: rgba(var(--v-theme-on-surface), 0.32) rgba(var(--v-theme-surface-variant), 0.45);

  &::-webkit-scrollbar {
    width: 10px;
  }

  &::-webkit-scrollbar-track {
    margin-block: 4px;
    background: rgba(var(--v-theme-surface-variant), 0.4);
    border-radius: 999px;
  }

  &::-webkit-scrollbar-thumb {
    border-radius: 999px;
    border: 2px solid transparent;
    background-clip: padding-box;
    background-color: rgba(var(--v-theme-on-surface), 0.22);
  }

  &::-webkit-scrollbar-thumb:hover {
    background-color: rgba(var(--v-theme-on-surface), 0.38);
  }
}
</style>
