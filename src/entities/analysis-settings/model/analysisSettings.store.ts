// User toggle for whether the app may kick off background Stockfish batches after sync (persisted in localStorage).
import { defineStore } from 'pinia';

const STORAGE_KEY = 'app-background-analysis'; // Isolated key so clearing unrelated prefs does not reset this choice.

function readInitial(): boolean {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (raw === null) {
      return true;
    }
    return raw === '1' || raw === 'true';
  } catch {
    return true;
  }
}

export const useAnalysisSettingsStore = defineStore('analysis-settings', {
  // `backgroundAnalysisEnabled` is read before scheduling post-sync analysis; settings UI is the primary writer.
  state: () => ({
    backgroundAnalysisEnabled: readInitial(),
  }),

  actions: {
    setBackgroundAnalysisEnabled(value: boolean) {
      this.backgroundAnalysisEnabled = value;
      try {
        localStorage.setItem(STORAGE_KEY, value ? '1' : '0');
      } catch {
        /* ignore */
      }
    },
  },
});
