import { invoke } from "@tauri-apps/api/core";
import { defineStore } from "pinia";

export const useGamesStore = defineStore("games", {
  state: () => ({
    games: [],
    loading: false,
  }),

  actions: {
    async sync(username: string) {
      this.loading = true;
      await invoke("sync_lichess_games", { username });
      this.games = await invoke("get_games", { username });
      this.loading = false;
    },
  },
});
