import { invoke } from '@tauri-apps/api/core';
import type { UseQueryReturnType } from '@tanstack/vue-query';
import { useQuery } from '@tanstack/vue-query';
import { computed, type MaybeRef, toValue } from 'vue';

import type { GameAnalysis } from '../model/gameAnalysis.types';

export function useGameAnalysisQuery(
  gameId: MaybeRef<string>,
): UseQueryReturnType<GameAnalysis | null, Error> {
  return useQuery<GameAnalysis | null>({
    queryKey: computed(() => ['game-analysis', toValue(gameId)]),

    queryFn: async () => {
      const id = toValue(gameId);
      if (!id) {
        return null;
      }

      return invoke<GameAnalysis | null>('get_game_analysis', { gameId: id });
    },

    staleTime: 15_000,
    enabled: computed(() => Boolean(toValue(gameId))),
  });
}
