<template>
  <div class="analysis-page">
    <chess-board :fen="fen" @move="makeMove" />

    <!-- <div class="side">
      <h3>Ходы</h3>
      <div v-for="(move, index) in moves" :key="index">
        {{ move }}
      </div>
    </div> -->
  </div>
</template>

<script setup lang="ts">
// Route-level page: composes features/widgets for this screen and wires page-local queries or navigation.

import { Chess } from 'chess.js';
import { ref } from 'vue';

import { ChessBoard } from '@/widgets/ChessBoard';

const game = new Chess();

const fen = ref(game.fen());
const moves = ref<string[]>([]);

function makeMove(move: { from: string; to: string }): void {
  const result = game.move(move);

  if (result) {
    fen.value = game.fen();
    moves.value = game.history();
  }
}
</script>

<style lang="scss" scoped>
.analysis-page {
  padding: 2rem;
  max-width: 58rem;
  flex: 1;
  height: 100%;
}
</style>
