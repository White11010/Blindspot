import { defineStore } from 'pinia';

const STORAGE_KEY = 'app-background-analysis';

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
