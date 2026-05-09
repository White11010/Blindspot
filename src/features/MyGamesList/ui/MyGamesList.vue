<template>
  <div
    v-if="games.length"
    class="my-games-list-hover-root"
    @mouseleave="onGamesTableMouseLeave"
    @mouseover="onGamesTableMouseOver"
  >
    <v-data-table
      v-model:sort-by="sortBy"
      :headers="headers"
      :items="games"
      class="my-games-list"
      :cell-props="cellDataAttrs"
      :custom-key-sort="customKeySort"
      items-per-page="25"
      must-sort
    >
      <template #item.result_marker="{ item }">
        <div
          class="result-marker text-body-2 font-weight-medium"
          :class="'result-marker--' + item.player_result"
        >
          {{ resultLetter(item.player_result) }}
        </div>
      </template>

      <template #item.opponent_name="{ item }">
        <span class="d-inline-flex align-center ga-1 flex-wrap">
          <span class="color-piece text-medium-emphasis" aria-hidden="true">{{
            colorPiece(item.player_color)
          }}</span>
          <span>{{ item.opponent_name || emDash }}</span>
          <span v-if="item.opponent_rating != null" class="text-medium-emphasis text-body-2">
            ({{ item.opponent_rating }})
          </span>
        </span>
      </template>

      <template #item.opening_name="{ item }">
        <v-tooltip
          v-if="openingTooltipText(item.opening_name, item.opening_eco)"
          location="top"
          max-width="320"
        >
          <template #activator="{ props: tipProps }">
            <span
              v-bind="tipProps"
              class="opening-short text-truncate d-inline-block"
              style="max-width: 14rem"
            >
              {{ shortOpeningDisplay(item.opening_name) }}
            </span>
          </template>
          <span style="white-space: pre-wrap">{{
            openingTooltipText(item.opening_name, item.opening_eco)
          }}</span>
        </v-tooltip>
        <span v-else class="text-medium-emphasis">{{ emDash }}</span>
      </template>

      <template #item.speed="{ item }">
        <v-tooltip v-if="item.time_control" location="top">
          <template #activator="{ props: tipProps }">
            <v-chip v-bind="tipProps" size="small" variant="tonal" class="text-none">
              {{ localizedSpeedChipLabel(item.speed) }}
            </v-chip>
          </template>
          {{ item.time_control }}
        </v-tooltip>
        <v-chip v-else size="small" variant="tonal" class="text-none">
          {{ localizedSpeedChipLabel(item.speed) }}
        </v-chip>
      </template>

      <template #item.player_rating="{ item }">
        {{ item.player_rating ?? emDash }}
      </template>

      <template #item.analysis_accuracy="{ item }">
        <span
          v-if="accuracyPercentRounded(item.analysis_accuracy) != null"
          :class="accuracyClass(item.analysis_accuracy)"
        >
          {{ accuracyPercentRounded(item.analysis_accuracy) }}%
        </span>
        <v-progress-circular
          v-else-if="backgroundAnalysisEnabled && showAnalysisPendingLoader(item)"
          indeterminate
          size="20"
          width="2"
        />
        <span v-else class="text-medium-emphasis">{{ emDash }}</span>
      </template>

      <template #item.analysis_acpl="{ item }">
        <span v-if="item.analysis_acpl != null">{{ formatAcpl(item.analysis_acpl) }}</span>
        <v-progress-circular
          v-else-if="backgroundAnalysisEnabled && showAnalysisPendingLoader(item)"
          indeterminate
          size="20"
          width="2"
        />
        <span v-else class="text-medium-emphasis">{{ emDash }}</span>
      </template>

      <template #item.pattern_tags="{ item }">
        <v-tooltip v-if="extraPatternTags(item).length" location="top">
          <template #activator="{ props: tipProps }">
            <span v-bind="tipProps" class="d-inline-flex flex-wrap ga-1 align-center">
              <v-chip
                v-for="tag in visiblePatternTags(item)"
                :key="tag"
                size="small"
                density="compact"
                variant="tonal"
                class="text-none"
              >
                {{ formatPatternTag(tag) }}
              </v-chip>
              <span v-if="extraPatternTags(item).length" class="text-caption text-medium-emphasis"
                >+{{ extraPatternTags(item).length }}</span
              >
            </span>
          </template>
          <span>{{ extraPatternTags(item).map(formatPatternTag).join(', ') }}</span>
        </v-tooltip>
        <span v-else class="d-inline-flex flex-wrap ga-1 align-center">
          <template v-if="visiblePatternTags(item).length">
            <v-chip
              v-for="tag in visiblePatternTags(item)"
              :key="tag"
              size="small"
              density="compact"
              variant="tonal"
              class="text-none"
            >
              {{ formatPatternTag(tag) }}
            </v-chip>
          </template>
          <v-progress-circular
            v-else-if="backgroundAnalysisEnabled && showAnalysisPendingLoader(item)"
            indeterminate
            size="20"
            width="2"
          />
          <span v-else class="text-medium-emphasis">{{ emDash }}</span>
        </span>
      </template>

      <template #item.created_at="{ item }">
        {{ formatMyGamesTableDate(item.created_at, locale) }}
      </template>

      <template #item.actions="{ item }">
        <div class="d-flex align-center ga-0">
          <v-btn
            variant="plain"
            color="secondary"
            icon="mdi-chart-timeline-variant-shimmer"
            :aria-label="t('myGames.table.ariaOpenDetails')"
            @click="onDetailsButtonClick(item.id)"
          />
          <v-btn
            v-if="getExternalGameUrl(item)"
            variant="plain"
            color="primary"
            icon="mdi-open-in-new"
            :aria-label="t('myGames.table.ariaOpenPlatform')"
            @click="onExternalClick(item)"
          />
        </div>
      </template>
    </v-data-table>
  </div>

  <Teleport to="body">
    <div
      v-show="boardPreview"
      class="my-games-list__board-preview"
      :style="boardPreviewStyle"
      @mouseenter="hideBoardPreview"
    >
      <template v-if="boardPreview && canShowFinalBoard(boardPreview.game)">
        <ChessStaticBoard
          :fen="boardPreview.game.last_fen!"
          :last-move="finalLastMove(boardPreview.game)"
          :orientation="boardPreview.game.player_color"
          :winner="boardPreview.game.winner"
          size="200px"
        />
      </template>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { useAnalysisSettingsStore } from '@/entities/analysis-settings';
import {
  type Game,
  type MyGamesTableSortItem,
  accuracyPercentRounded,
  accuracyToneFromRounded,
  getExternalGameUrl,
  openExternalGame,
  openingTooltipText,
  resultLetter,
  shortOpeningDisplay,
} from '@/entities/game';
import { ChessStaticBoard } from '@/shared/ui';
import { formatMyGamesTableDate, formatPatternTagLabel } from '@/shared/lib';
import { useI18n } from '@/shared/lib/i18n';
import type { Key } from 'chessground/types';
import { storeToRefs } from 'pinia';
import { computed, shallowRef } from 'vue';
import { useRouter } from 'vuetify/lib/composables/router.mjs';

const props = defineProps<{
  games: Game[];
}>();

const sortBy = defineModel<MyGamesTableSortItem[]>('sortBy', { required: true });

const { t, te, locale } = useI18n();
const emDash = computed(() => t('common.emDash'));

const router = useRouter();

const { backgroundAnalysisEnabled } = storeToRefs(useAnalysisSettingsStore());

const boardPreview = shallowRef<{ game: Game; rect: DOMRect } | null>(null);

function hideBoardPreview() {
  boardPreview.value = null;
}

function cellDataAttrs(data: { item: Game }) {
  return {
    'data-game-id': data.item.id,
  };
}

function canShowFinalBoard(game: Game): boolean {
  return Boolean(game.last_fen && game.moves);
}

function finalLastMove(game: Game): [Key, Key] {
  const tokens = (game.moves || '').trim().split(/\s+/).filter(Boolean);
  const lastToken = tokens[tokens.length - 1] || 'e2e4';
  return [lastToken.slice(0, 2) as Key, lastToken.slice(2, 4) as Key];
}

function onGamesTableMouseOver(e: MouseEvent) {
  const host = (e.target as HTMLElement | null)?.closest?.('[data-game-id]') as HTMLElement | null;
  if (!host?.dataset.gameId) {
    hideBoardPreview();
    return;
  }
  const game = props.games.find((g) => g.id === host.dataset.gameId);
  if (!game || !canShowFinalBoard(game)) {
    hideBoardPreview();
    return;
  }
  const tr = host.closest('tr');
  const rect = (tr ?? host).getBoundingClientRect();
  boardPreview.value = { game, rect };
}

function onGamesTableMouseLeave() {
  hideBoardPreview();
}

const boardPreviewStyle = computed(() => {
  const b = boardPreview.value;
  if (!b) {
    return {};
  }
  const pad = 8;
  const boardPx = 200;
  const vw = typeof window !== 'undefined' ? window.innerWidth : 1200;
  const vh = typeof window !== 'undefined' ? window.innerHeight : 800;
  let left = b.rect.right + pad;
  if (left + boardPx > vw - pad) {
    left = Math.max(pad, b.rect.left - pad - boardPx);
  }
  const top = Math.max(pad, Math.min(b.rect.top, vh - boardPx - pad));
  return {
    position: 'fixed' as const,
    left: `${left}px`,
    top: `${top}px`,
    zIndex: 3000,
    width: `calc(${boardPx}px + ${pad * 2}px)`,
    height: `calc(${boardPx}px + ${pad * 2}px)`,
    overflow: 'hidden',
  };
});

function showAnalysisPendingLoader(item: Game): boolean {
  return accuracyPercentRounded(item.analysis_accuracy) == null;
}

function formatAcpl(v: number): string {
  return Number.isInteger(v) ? String(v) : v.toFixed(1);
}

function nullableNumSort(a: unknown, b: unknown): number | null {
  const an = a == null || a === '' ? null : Number(a);
  const bn = b == null || b === '' ? null : Number(b);
  if (an === null && bn === null) {
    return 0;
  }
  if (an === null) {
    return 1;
  }
  if (bn === null) {
    return -1;
  }
  if (Number.isNaN(an) || Number.isNaN(bn)) {
    return null;
  }
  if (an === bn) {
    return 0;
  }
  return an < bn ? -1 : 1;
}

function strEmptyLastSort(a: unknown, b: unknown): number | null {
  const sa = a == null ? '' : String(a).toLowerCase();
  const sb = b == null ? '' : String(b).toLowerCase();
  const ae = sa === '';
  const be = sb === '';
  if (ae && be) {
    return 0;
  }
  if (ae) {
    return 1;
  }
  if (be) {
    return -1;
  }
  if (sa < sb) {
    return -1;
  }
  if (sa > sb) {
    return 1;
  }
  return 0;
}

function speedRank(s: unknown): number {
  const g = String(s ?? '').toLowerCase();
  if (g === 'ultrabullet') {
    return 0;
  }
  if (g === 'bullet') {
    return 1;
  }
  if (g === 'blitz') {
    return 2;
  }
  if (g === 'rapid') {
    return 3;
  }
  if (g === 'classical') {
    return 4;
  }
  return 99;
}

function speedSort(a: unknown, b: unknown): number | null {
  const ra = speedRank(a);
  const rb = speedRank(b);
  if (ra !== rb) {
    return ra - rb;
  }
  return null;
}

const customKeySort: Record<string, (a: unknown, b: unknown) => number | null> = {
  analysis_accuracy: nullableNumSort,
  analysis_acpl: nullableNumSort,
  player_rating: nullableNumSort,
  created_at: nullableNumSort,
  opponent_name: strEmptyLastSort,
  opening_name: strEmptyLastSort,
  speed: speedSort,
};

const headers = computed(() => [
  { key: 'result_marker', title: '', width: '40px', sortable: false },
  { key: 'opponent_name', title: t('myGames.table.opponent'), sortable: true },
  { key: 'opening_name', title: t('myGames.table.opening'), sortable: true },
  { key: 'speed', title: t('myGames.table.time'), width: '100px', sortable: true },
  { key: 'player_rating', title: t('myGames.table.rating'), width: '88px', sortable: true },
  { key: 'analysis_accuracy', title: t('myGames.table.accuracy'), width: '88px', sortable: true },
  { key: 'analysis_acpl', title: t('myGames.table.acpl'), width: '72px', sortable: true },
  { key: 'pattern_tags', title: t('myGames.table.patterns'), sortable: false },
  { key: 'created_at', title: t('myGames.table.date'), width: '144px', sortable: true },
  { key: 'actions', title: '', width: '96px', sortable: false },
]);

function localizedSpeedChipLabel(speed: string): string {
  const s = speed.toLowerCase();
  const normalized = s === 'ultrabullet' ? 'bullet' : s;
  if (
    normalized === 'bullet' ||
    normalized === 'blitz' ||
    normalized === 'rapid' ||
    normalized === 'classical'
  ) {
    return t(`myGames.speed.${normalized}`);
  }
  return speed ? speed.charAt(0).toUpperCase() + speed.slice(1).toLowerCase() : t('common.emDash');
}

function colorPiece(playerColor: Game['player_color']): string {
  return playerColor === 'white' ? '♔' : '♚';
}

function accuracyClass(acc: number | null | undefined): string {
  const r = accuracyPercentRounded(acc);
  const tone = accuracyToneFromRounded(r);
  if (tone === 'high') {
    return 'text-success';
  }
  if (tone === 'mid') {
    return 'text-warning';
  }
  if (tone === 'low') {
    return 'text-error';
  }
  return '';
}

function visiblePatternTags(item: Game): string[] {
  return (item.pattern_tags ?? []).slice(0, 2);
}

function extraPatternTags(item: Game): string[] {
  return (item.pattern_tags ?? []).slice(2);
}

function formatPatternTag(tag: string): string {
  return formatPatternTagLabel(tag, t, te);
}

function onDetailsButtonClick(id: string) {
  router?.push(`/game-details/${id}`);
}

function onExternalClick(item: Game) {
  void openExternalGame(item);
}
</script>

<style lang="scss" scoped>
.my-games-list {
  :deep(td) {
    height: 64px;
  }

  :deep(td:first-child) {
    padding-inline: 0;
  }
}

.result-marker {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 64px;
  margin-block: -8px;
  margin-inline-start: 0;
  margin-inline-end: 0;
  padding-inline: 8px;
  box-shadow: inset 3px 0 0 0 rgb(var(--v-theme-on-surface));
  opacity: 0.92;
}

.result-marker--win {
  box-shadow: inset 3px 0 0 0 rgb(var(--v-theme-success));
}

.result-marker--loss {
  box-shadow: inset 3px 0 0 0 rgb(var(--v-theme-error));
}

.result-marker--draw {
  box-shadow: inset 3px 0 0 0 rgb(var(--v-theme-outline));
}

.color-piece {
  font-size: 1rem;
  line-height: 1;
}

.my-games-list-hover-root {
  width: 100%;
}

.my-games-list__board-preview {
  padding: 0.5rem;
  border-radius: 0.5rem;
  background: rgb(var(--v-theme-surface));
  box-shadow:
    0 0.25rem 0.75rem rgba(0, 0, 0, 0.25),
    0 0 0 1px rgba(var(--v-border-color), var(--v-border-opacity));
  pointer-events: auto;
}
</style>
