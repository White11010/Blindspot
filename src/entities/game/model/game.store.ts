// Cached game list from SQLite (`get_games`); My Games and home widgets read `games`, sync flow calls `loadFromDb`.
import { invoke } from '@tauri-apps/api/core';
import { defineStore } from 'pinia';

import type { Game } from './games.types';

interface State {
  games: Game[];
  wasLoadedEmptyList: boolean;
}

export const useGamesStore = defineStore('games', {
  // `games` mirrors the backend list; `wasLoadedEmptyList` avoids flashing “no games” before first successful load.
  state: (): State => ({
    games: [],
    wasLoadedEmptyList: false,
  }),

  actions: {
    async loadFromDb() {
      const games = await invoke('get_games');
      this.games = games as Game[];
      this.wasLoadedEmptyList = this.games.length === 0;
    },
  },
});
