import { useAnalysisSettingsStore } from '../model/analysisSettings.store';
import { requestBackgroundAnalysisBatch } from './requestBatch';

/** Call after games are loaded from the server into the local DB (e.g. after `sync_games`). */
export function notifyGamesSynced(): void {
  const settings = useAnalysisSettingsStore();
  if (settings.backgroundAnalysisEnabled) {
    void requestBackgroundAnalysisBatch();
  }
}
