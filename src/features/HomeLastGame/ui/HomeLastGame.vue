<template>
  <v-card :title="t('home.lastGameTitle')">
    <template v-if="lastGame">
      <v-card-text>
        <v-row>
          <v-col cols="5" class="d-flex pa-2">
            <ChessStaticBoard
              :fen="lastGame.last_fen!"
              size="256px"
              :last-move="getLastMoveFromMoves(lastGame.moves!)"
              :orientation="lastGame.player_color"
              :winner="lastGame.winner"
            />
          </v-col>
          <v-col cols="7" class="d-flex flex-column ga-2">
            <v-list density="compact" class="flex-grow-1">
              <v-list-item>
                <v-list-item-title>{{
                  t('home.lastGameVs', { name: lastGame.opponent_name })
                }}</v-list-item-title>
                <v-list-item-subtitle>
                  {{ t('home.lastGameRating', { rating: lastGame.opponent_rating }) }}
                </v-list-item-subtitle>
              </v-list-item>

              <v-list-item>
                <v-list-item-title>
                  {{ t('gameDetails.listResult') }}: {{ playerResultLabel(lastGame.player_result) }}
                </v-list-item-title>
              </v-list-item>

              <v-list-item>
                <v-list-item-title>
                  {{ t('gameDetails.listColor') }}: {{ colorLabel(lastGame.player_color) }}
                </v-list-item-title>
              </v-list-item>

              <v-list-item>
                <v-list-item-title>
                  {{ t('gameDetails.listOpening') }}:
                  {{ lastGame.opening_name || t('gameDetails.unknownOpening') }}
                </v-list-item-title>
              </v-list-item>
            </v-list>

            <v-card-actions>
              <v-spacer />
              <v-btn
                class="button-width-12"
                color="primary"
                variant="elevated"
                prepend-icon="mdi-open-in-new"
                @click="onAnalizeButtonClick(lastGame)"
              >
                {{ t('home.lastGameAnalyze') }}
              </v-btn>
              <v-btn
                class="button-width-12"
                color="secondary"
                variant="elevated"
                prepend-icon="mdi-file-chart"
                @click="onReviewButtonClick(lastGame)"
              >
                {{ 'Подробнее' }}
              </v-btn>
            </v-card-actions>
          </v-col>
        </v-row>
      </v-card-text>
    </template>

    <v-skeleton-loader v-else type="list-item-two-line" />
  </v-card>
</template>

<script setup lang="ts">
import { computed } from 'vue';

import { Game, openExternalGame, useSyncGamesQuery } from '@/entities/game';
import { useI18n } from '@/shared/lib/i18n';
import { ChessStaticBoard } from '@/shared/ui';
import { Chess } from 'chess.js';
import { Key } from 'chessground/types';
import { useRouter } from 'vue-router';

const { t } = useI18n();
const router = useRouter();
const { games } = useSyncGamesQuery();

const lastGame = computed(() => games.value[0]);

function playerResultLabel(result: Game['player_result']): string {
  switch (result) {
    case 'win':
      return t('game.resultWin');
    case 'loss':
      return t('game.resultLoss');
    case 'draw':
      return t('game.resultDraw');
    default:
      return result;
  }
}

function colorLabel(color: Game['player_color']): string {
  return color === 'white' ? t('gameDetails.colorWhite') : t('gameDetails.colorBlack');
}

function onAnalizeButtonClick(game: Game) {
  void openExternalGame(game);
}
function onReviewButtonClick(game: Game) {
  router.push(`/game-details/${game.id}`);
}

function getLastMoveFromMoves(moves: string): [Key, Key] {
  const game = new Chess();

  const movesArr = moves.split(' ').filter(Boolean);

  let lastMove = null;

  for (const san of movesArr) {
    const move = game.move(san);
    lastMove = move;
  }

  return [lastMove!.from, lastMove!.to];
}
</script>
