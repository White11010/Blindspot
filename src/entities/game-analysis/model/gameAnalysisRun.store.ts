// UI mutex flag so game list rows can show a spinner on the game currently being analyzed by Stockfish batch or single run.
import { defineStore } from 'pinia';

export const useGameAnalysisRunStore = defineStore('game-analysis-run', {
  // `runningGameId` is set/cleared by analysis flows; list components read it to highlight the active row only.
  state: () => ({
    runningGameId: null as string | null,
  }),

  actions: {
    setRunningGameId(gameId: string | null) {
      this.runningGameId = gameId;
    },
  },
});
