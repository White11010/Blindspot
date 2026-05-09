import { invoke } from '@tauri-apps/api/core';

import { useUserStore } from '@/entities/user';

import { useAnalysisSettingsStore } from '../model/analysisSettings.store';

function isBatchBusyMessage(msg: string): boolean {
  return msg.includes('already running') || msg.includes('batch-busy');
}

export async function requestBackgroundAnalysisBatch(): Promise<void> {
  const settings = useAnalysisSettingsStore();
  if (!settings.backgroundAnalysisEnabled) {
    return;
  }
  if (!useUserStore().user) {
    return;
  }
  try {
    await invoke('analyze_pending_games', { depth: null });
  } catch (e) {
    const msg = e instanceof Error ? e.message : String(e);
    if (isBatchBusyMessage(msg)) {
      return;
    }
    console.error(e);
  }
}

export function cancelBackgroundAnalysisBatch(): void {
  void invoke('cancel_pending_analysis');
}
