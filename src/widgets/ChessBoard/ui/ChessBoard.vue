<template>
  <section>
    <!-- <div>
      <button @click="boardAPI?.toggleOrientation()">Toggle orientation</button>
      <button @click="boardAPI?.resetBoard()">Reset</button>
      <button @click="boardAPI?.undoLastMove()">Undo</button>
      <button @click="boardAPI?.toggleMoves()">Threats</button>
    </div> -->
    <the-chessboard
      :board-config="boardConfig"
      @board-created="(api) => (boardAPI = api)"
      @checkmate="handleCheckmate"
    />
  </section>
</template>

<script setup lang="ts">
import 'vue3-chessboard/style.css';

import type { BoardApi, BoardConfig } from 'vue3-chessboard';
import { TheChessboard } from 'vue3-chessboard';

let boardAPI: BoardApi;
const boardConfig: BoardConfig = {
  coordinates: true,
};

function handleCheckmate(isMated: string): void {
  if (isMated === 'w') {
    alert('Black wins!');
  } else {
    alert('White wins!');
  }
}
</script>
