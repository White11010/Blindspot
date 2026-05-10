// Insights page toolbar: filter/sort snapshot persisted in sessionStorage so navigation does not reset user choices.
import { defineStore } from 'pinia';

import type { InsightCategory } from './insight.types';

const STORAGE_KEY = 'chessanalytics:insightsFilters'; // sessionStorage scope: per tab session, mirrors myGamesFilters policy.

export type InsightsFilterKey = 'all' | 'attention' | InsightCategory;

export type InsightsSortOrder = 'highFirst' | 'lowFirst';

const VALID_FILTERS: readonly InsightsFilterKey[] = [
  'all',
  'attention',
  'openings',
  'time',
  'tactics',
  'psychology',
];

const VALID_SORTS: readonly InsightsSortOrder[] = ['highFirst', 'lowFirst'];

export interface InsightsFiltersSnapshot {
  selectedFilter: InsightsFilterKey;
  sortOrder: InsightsSortOrder;
}

function defaultSnapshot(): InsightsFiltersSnapshot {
  return {
    selectedFilter: 'all',
    sortOrder: 'highFirst',
  };
}

function loadSnapshot(): InsightsFiltersSnapshot {
  try {
    const raw = sessionStorage.getItem(STORAGE_KEY);
    if (!raw) {
      return defaultSnapshot();
    }
    const parsed = JSON.parse(raw) as Partial<InsightsFiltersSnapshot>;
    const base = defaultSnapshot();
    // Validate against known unions so a stale schema cannot leak invalid values into the UI.
    const selectedFilter =
      parsed.selectedFilter && VALID_FILTERS.includes(parsed.selectedFilter)
        ? parsed.selectedFilter
        : base.selectedFilter;
    const sortOrder =
      parsed.sortOrder && VALID_SORTS.includes(parsed.sortOrder)
        ? parsed.sortOrder
        : base.sortOrder;
    return { selectedFilter, sortOrder };
  } catch {
    return defaultSnapshot();
  }
}

function saveSnapshot(s: InsightsFiltersSnapshot) {
  sessionStorage.setItem(STORAGE_KEY, JSON.stringify(s));
}

export const useInsightsFiltersStore = defineStore('insightsFilters', {
  state: (): InsightsFiltersSnapshot => loadSnapshot(),

  actions: {
    persist() {
      saveSnapshot({
        selectedFilter: this.selectedFilter,
        sortOrder: this.sortOrder,
      });
    },

    setFilter(filter: InsightsFilterKey) {
      this.selectedFilter = filter;
      this.persist();
    },

    setSortOrder(order: InsightsSortOrder) {
      this.sortOrder = order;
      this.persist();
    },

    reset() {
      Object.assign(this, defaultSnapshot());
      this.persist();
    },
  },
});
