import { defineStore } from 'pinia';

/** Which game is currently being analyzed (background batch or explicit run). */
export const useGameAnalysisRunStore = defineStore('game-analysis-run', {
  state: () => ({
    runningGameId: null as string | null,
  }),

  actions: {
    setRunningGameId(gameId: string | null) {
      this.runningGameId = gameId;
    },
  },
});
