import { invoke } from '@tauri-apps/api/core';
import { defineStore } from 'pinia';

import type { Insight } from './insight.types';

interface State {
  items: Insight[];

  isLoading: boolean;
  isRefreshing: boolean;

  error: string | null;

  lastLoadedAt: number | null;

  /** Calendar day `YYYY-MM-DD` (local) for which `dailyInsight` was resolved. */
  dailyPickedForDate: string | null;
  dailyInsight: Insight | null;
}

export const useInsightsStore = defineStore('insights', {
  state: (): State => ({
    items: [],

    isLoading: false,
    isRefreshing: false,

    error: null,

    lastLoadedAt: null,

    dailyPickedForDate: null,
    dailyInsight: null,
  }),

  getters: {
    heroInsight(state): Insight | null {
      return state.dailyInsight;
    },

    goodInsights(state): Insight[] {
      return state.items.filter((item) => item.severity === 'good');
    },

    warningInsights(state): Insight[] {
      return state.items.filter(
        (item) => item.severity === 'warning' || item.severity === 'critical',
      );
    },
  },

  actions: {
    localCalendarDay(): string {
      return new Date().toLocaleDateString('en-CA');
    },

    async loadDailyInsight(): Promise<void> {
      const today = this.localCalendarDay();
      if (this.dailyPickedForDate === today && this.dailyInsight !== null) {
        return;
      }
      if (this.items.length === 0) {
        this.dailyInsight = null;
        this.dailyPickedForDate = today;
        return;
      }

      try {
        const data = await invoke<Insight | null>('get_daily_insight');
        this.dailyInsight = data;
        this.dailyPickedForDate = today;
      } catch {
        this.dailyInsight = null;
        this.dailyPickedForDate = today;
      }
    },

    async load() {
      this.isLoading = true;
      this.error = null;

      try {
        const data = await invoke<Insight[]>('get_insights');

        this.items = data;
        this.lastLoadedAt = Date.now();
        await this.loadDailyInsight();
      } catch (error) {
        this.error = String(error);
      } finally {
        this.isLoading = false;
      }
    },

    async regenerate() {
      this.isRefreshing = true;
      this.error = null;

      try {
        const data = await invoke<Insight[]>('regenerate_insights');

        this.items = data;
        this.lastLoadedAt = Date.now();
        this.dailyInsight = null;
        this.dailyPickedForDate = null;
        await this.loadDailyInsight();
      } catch (error) {
        this.error = String(error);
      } finally {
        this.isRefreshing = false;
      }

      return this.items as Insight[];
    },

    // async refreshBackground() {
    //   try {
    //     await invoke('refresh_insights_background');
    //   } catch (error) {
    //     console.error(error);
    //   }
    // },

    clear() {
      this.items = [];
      this.error = null;
      this.lastLoadedAt = null;
      this.dailyInsight = null;
      this.dailyPickedForDate = null;
    },
  },
});
