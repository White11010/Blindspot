<template>
  <template v-if="gamesStore.games.length">
    <div class="my-games-section">
      <aside class="my-games-section__sidebar">
        <my-games-toolbar :pattern-options="patternOptions" :opening-options="openingOptions" />
      </aside>
      <div class="my-games-section__main">
        <p v-if="showResultCount" class="text-body-2 text-medium-emphasis mb-2">
          {{
            t('myGames.section.shown', {
              shown: displayedGames.length,
              total: gamesStore.games.length,
            })
          }}
        </p>
        <div class="my-games-section__table-wrap">
          <my-games-list
            v-if="displayedGames.length"
            v-model:sort-by="sortBy"
            :games="displayedGames"
          />
          <v-card v-else variant="tonal" class="pa-6">
            <p class="text-body-1 text-center text-medium-emphasis mb-0">
              {{ t('myGames.section.noMatching') }}
            </p>
          </v-card>
        </div>
      </div>
    </div>
  </template>
</template>

<script setup lang="ts">
import { useMyGamesFiltersStore } from '@/entities/game';
import { MyGamesList } from '@/features/MyGamesList';
import { MyGamesToolbar } from '@/features/MyGamesToolbar';
import { useI18n } from '@/shared/lib/i18n';
import { storeToRefs } from 'pinia';

import { useMyGamesSection } from '../lib/useMyGamesSection';

const { t } = useI18n();

const filtersStore = useMyGamesFiltersStore();
const { sortBy } = storeToRefs(filtersStore);

const { gamesStore, patternOptions, openingOptions, displayedGames, showResultCount } =
  useMyGamesSection();
</script>

<style scoped lang="scss">
.my-games-section {
  width: 100%;
  display: grid;
  gap: 1.5rem;
  align-items: start;
}

.my-games-section__table-wrap {
  max-width: 100%;
  overflow-x: auto;
}

@media (min-width: 1200px) {
  .my-games-section {
    grid-template-columns: 1fr minmax(280px, 22rem);
  }

  .my-games-section__main {
    grid-column: 1;
    grid-row: 1;
    min-width: 0;
  }

  .my-games-section__sidebar {
    grid-column: 2;
    grid-row: 1;
  }
}

@media (max-width: 1199.98px) {
  .my-games-section {
    display: flex;
    flex-direction: column;
  }

  .my-games-section__main,
  .my-games-section__sidebar {
    align-self: stretch;
    max-width: 100%;
  }
}
</style>
