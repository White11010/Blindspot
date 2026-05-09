<template>
  <v-card>
    <v-card-title class="d-flex align-center justify-space-between flex-wrap ga-2">
      <span>{{ t('gameDetails.snapshotTitle') }}</span>
      <v-chip size="small" variant="tonal" color="secondary">
        {{ heroTimestamp }}
      </v-chip>
    </v-card-title>

    <v-card-text>
      <v-row>
        <v-col cols="12" md="4" class="d-flex justify-center">
          <ChessStaticBoard
            v-if="canShowBoard"
            :fen="game.last_fen!"
            :last-move="lastMove"
            :orientation="game.player_color"
            :winner="game.winner"
            size="240px"
          />
          <v-sheet v-else rounded border class="d-flex align-center justify-center board-placeholder">
            <v-icon icon="mdi-chess-king" color="secondary" size="36" />
          </v-sheet>
        </v-col>

        <v-col cols="12" md="8">
          <v-list density="comfortable">
            <v-list-item :title="t('gameDetails.listOpponent')" :subtitle="game.opponent_name" />
            <v-list-item :title="t('gameDetails.listResult')" :subtitle="resultLabel" />
            <v-list-item
              :title="t('gameDetails.listOpening')"
              :subtitle="game.opening_name || t('gameDetails.unknownOpening')"
            />
            <v-list-item
              :title="t('gameDetails.listTimeControl')"
              :subtitle="`${game.speed} • ${game.time_control}`"
            />
            <v-list-item :title="t('gameDetails.listColor')" :subtitle="colorLabel" />
          </v-list>
        </v-col>
      </v-row>
    </v-card-text>
  </v-card>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import type { Key } from 'chessground/types';

import type { Game } from '@/entities/game';
import { formatTimestamp } from '@/shared/lib/dates';
import { useI18n } from '@/shared/lib/i18n';
import { ChessStaticBoard } from '@/shared/ui';

const props = defineProps<{
  game: Game;
}>();

const { t, locale } = useI18n();

const heroTimestamp = computed(() =>
  formatTimestamp(props.game.created_at, { locale: locale.value }),
);

const canShowBoard = computed(() => Boolean(props.game.last_fen && props.game.moves));

const lastMove = computed<[Key, Key]>(() => {
  const tokens = (props.game.moves || '').trim().split(/\s+/).filter(Boolean);
  const lastToken = tokens[tokens.length - 1] || 'e2e4';
  const from = lastToken.slice(0, 2) as Key;
  const to = lastToken.slice(2, 4) as Key;
  return [from, to];
});

const resultLabel = computed(() => {
  const r = props.game.player_result;
  if (r === 'win') {
    return t('game.resultWin');
  }
  if (r === 'loss') {
    return t('game.resultLoss');
  }
  return t('game.resultDraw');
});

const colorLabel = computed(() =>
  props.game.player_color === 'white' ? t('gameDetails.colorWhite') : t('gameDetails.colorBlack'),
);
</script>

<style scoped lang="scss">
.board-placeholder {
  width: 240px;
  height: 240px;
}
</style>
