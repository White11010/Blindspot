<template>
  <div ref="boardEl" :style="{ width: props.size, height: props.size }"></div>
</template>

<script setup lang="ts">
// Shared presentational component: minimal state, reused across pages for consistent visuals.

import { nextTick, onMounted, ref, watch } from 'vue';
import { Chessground } from 'chessground';

import 'chessground/assets/chessground.base.css';
import 'chessground/assets/chessground.brown.css';

import type { Api } from 'chessground/api';
import { Key } from 'chessground/types';
import { Chess } from 'chess.js';

const props = withDefaults(
  defineProps<{
    fen: string;
    lastMove: [Key, Key];
    size?: string;
    orientation: 'white' | 'black';
    /** Loser side when checkmate; omit or null for draws / non-mate endings. */
    winner?: 'white' | 'black' | null;
  }>(),
  {
    size: '100%',
    winner: null,
  },
);

const boardEl = ref<HTMLDivElement | null>(null);
let ground: Api | null = null;

/** Chessground memoizes `cg-board` bounds; after Teleport / v-show / layout, first read can be wrong. */
function syncBoardLayout() {
  if (!ground) {
    return;
  }
  const dom = ground.state.dom as typeof ground.state.dom & {
    redrawNow: (skipSvg: boolean) => void;
  };
  dom.bounds.clear();
  dom.redrawNow(false);
}

onMounted(() => {
  if (!boardEl.value) return;

  const game = new Chess(props.fen);

  ground = Chessground(boardEl.value, {
    fen: props.fen,
    orientation: props.orientation,
    viewOnly: true,

    coordinates: false, // ✅ отключаем координаты

    movable: {
      free: false,
      color: undefined,
      dests: new Map(),
    },

    draggable: { enabled: false },
    selectable: { enabled: false },

    highlight: {
      lastMove: true,
      check: true, // ✅ подсветка шаха
    },

    drawable: {
      enabled: false,
      visible: false,
    },

    // ✅ подсветка последнего хода
    lastMove: props.lastMove,
  });

  highlightMate(game);
  void nextTick(() => {
    requestAnimationFrame(() => {
      syncBoardLayout();
      highlightMate(new Chess(props.fen));
    });
  });
});

// ---------------- WATCH ----------------
watch(
  () => [props.fen, props.lastMove[0], props.lastMove[1], props.winner, props.size] as const,
  () => {
    if (!ground) return;

    const game = new Chess(props.fen);

    ground.set({
      fen: props.fen,
      lastMove: props.lastMove,
    });

    highlightMate(game);
    void nextTick(() => {
      requestAnimationFrame(() => syncBoardLayout());
    });
  },
);

// ---------------- CHECKMATE HIGHLIGHT ----------------
/** Chessground only clears `check` when `check` is present in `set()` (see configure). */
function highlightMate(game: Chess) {
  if (!ground) return;

  if (game.isCheckmate()) {
    const mated: 'white' | 'black' =
      props.winner != null
        ? props.winner === 'white'
          ? 'black'
          : 'white'
        : game.turn() === 'w'
          ? 'white'
          : 'black';
    ground.set({
      check: mated,
    });
  } else {
    ground.set({ check: false });
  }
}
</script>

<style scoped>
.board {
  width: 100%;
  height: 200px;
}
</style>
