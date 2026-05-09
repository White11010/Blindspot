import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { storeToRefs } from 'pinia';
import { onMounted, onUnmounted, watch } from 'vue';

import { queryClient } from '@/app/providers/query';
import {
  cancelBackgroundAnalysisBatch,
  requestBackgroundAnalysisBatch,
  useAnalysisSettingsStore,
} from '@/entities/analysis-settings';
import { useGameAnalysisRunStore } from '@/entities/game-analysis';
import { useGamesStore } from '@/entities/game';
import { useUserStore } from '@/entities/user';

let unlisteners: UnlistenFn[] | null = null;

function readPayloadGameId(payload: unknown): string | undefined {
  if (payload && typeof payload === 'object' && 'game_id' in payload) {
    const v = (payload as { game_id?: unknown }).game_id;
    return typeof v === 'string' ? v : undefined;
  }
  return undefined;
}

async function ensureEventBridge(): Promise<void> {
  if (unlisteners) {
    return;
  }
  const gamesStore = useGamesStore();
  const runStore = useGameAnalysisRunStore();

  const u0 = await listen('game-analysis://analyzing', (event) => {
    const id = readPayloadGameId(event.payload);
    runStore.setRunningGameId(id ?? null);
  });

  const u1 = await listen('game-analysis://progress', (event) => {
    const gid = readPayloadGameId(event.payload);
    if (gid && runStore.runningGameId === gid) {
      runStore.setRunningGameId(null);
    }
    void gamesStore.loadFromDb();
    if (gid) {
      void queryClient.invalidateQueries({ queryKey: ['game-analysis', gid] });
    }
  });

  const u2 = await listen('game-analysis://done', () => {
    runStore.setRunningGameId(null);
    void gamesStore.loadFromDb();
  });

  unlisteners = [u0, u1, u2];
}

function teardownEventBridge(): void {
  if (unlisteners) {
    for (const u of unlisteners) {
      u();
    }
    unlisteners = null;
  }
}

/** Subscribes to Tauri analysis progress events (call once from app root). */
export function useBackgroundGameAnalysisBridge(): void {
  onMounted(() => {
    void ensureEventBridge();
  });
  onUnmounted(() => {
    teardownEventBridge();
  });
}

/**
 * Watches the settings toggle and coordinates invoke/cancel.
 * Call once from the app root (e.g. alongside the event bridge).
 */
export function useBackgroundGameAnalysisOrchestration(): void {
  const settings = useAnalysisSettingsStore();
  const { backgroundAnalysisEnabled } = storeToRefs(settings);
  const userStore = useUserStore();

  watch(
    backgroundAnalysisEnabled,
    (on) => {
      if (on) {
        void requestBackgroundAnalysisBatch();
      } else {
        cancelBackgroundAnalysisBatch();
      }
    },
    { flush: 'post' },
  );

  watch(
    () => userStore.user,
    () => {
      if (backgroundAnalysisEnabled.value) {
        void requestBackgroundAnalysisBatch();
      }
    },
    { flush: 'post' },
  );

  onMounted(() => {
    if (backgroundAnalysisEnabled.value) {
      void requestBackgroundAnalysisBatch();
    }
  });
}
