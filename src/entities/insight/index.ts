export {
  getInsightMetricLabel,
  getInsightRecommendation,
  getInsightSummary,
  getInsightTitle,
} from './lib/formatInsightForDisplay';
export {
  mergeInsightDisplayParams,
  openingNameFromSubjectKey,
  parseInsightPayload,
  speedLabelFromSubjectKey,
} from './lib/insightPayload';
export { useInsightsLoadQuery, useRegenerateInsightsQuery } from './lib/insightsQuery';
export { useInsightsStore } from './model/insight.store';
export type { Insight, InsightCategory } from './model/insight.types';