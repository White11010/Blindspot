<template>
  <div class="appearance-settings">
    <v-card :title="t('settings.appearanceCard')">
      <v-card-text>
        <change-language-settings />
      </v-card-text>
      <v-card-text>
        <change-theme-settings />
      </v-card-text>
      <v-card-text>
        <change-font-size />
      </v-card-text>
    </v-card>
  </div>
</template>

<script setup lang="ts">
// Composite widget: presents a focused dashboard block; reads shared Pinia stores and Tauri invoke where needed.

import { ChangeFontSize } from '@/features/ChangeFontSize';
import { ChangeLanguageSettings } from '@/features/ChangeLanguageSettings';
import { ChangeThemeSettings } from '@/features/ChangeThemeSettings';
import { useI18n } from '@/shared/lib/i18n';

const { t } = useI18n();
</script>
