import { invoke } from '@tauri-apps/api/core';
import { defineStore } from 'pinia';

export const useGamesStore = defineStore('games', {
  state: () => ({
    games: [],
    loading: false,
  }),

  actions: {
    async sync() {
      this.loading = true;
      await invoke('sync_games');
      const games = await invoke('get_games');
      this.games = games;
      this.loading = false;
    },
  },
});
