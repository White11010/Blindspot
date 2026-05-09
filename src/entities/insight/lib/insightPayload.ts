export type InsightPayloadParsed = {
  subject_key: string;
  params: Record<string, unknown>;
};

export function parseInsightPayload(raw: string | null | undefined): InsightPayloadParsed {
  if (!raw?.trim()) {
    return { subject_key: 'default', params: {} };
  }
  try {
    const j = JSON.parse(raw) as Record<string, unknown>;
    const subject_key = typeof j.subject_key === 'string' ? j.subject_key : 'default';
    const params =
      typeof j.params === 'object' && j.params !== null && !Array.isArray(j.params)
        ? (j.params as Record<string, unknown>)
        : {};
    return { subject_key, params };
  } catch {
    return { subject_key: 'default', params: {} };
  }
}

/** Opening name from `opening:Eco Name` subject key (debuts not translated). */
export function openingNameFromSubjectKey(subjectKey: string): string | null {
  if (!subjectKey.startsWith('opening:')) {
    return null;
  }
  return subjectKey.slice('opening:'.length) || null;
}

/** Raw speed label from `speed:Bullet` etc. */
export function speedLabelFromSubjectKey(subjectKey: string): string | null {
  if (!subjectKey.startsWith('speed:')) {
    return null;
  }
  return subjectKey.slice('speed:'.length) || null;
}

export function mergeInsightDisplayParams(
  subjectKey: string,
  params: Record<string, unknown>,
): Record<string, string | number | boolean> {
  const out: Record<string, string | number | boolean> = {};
  for (const [k, v] of Object.entries(params)) {
    if (typeof v === 'string' || typeof v === 'number' || typeof v === 'boolean') {
      out[k] = v;
    }
  }
  const opening = openingNameFromSubjectKey(subjectKey);
  if (opening != null) {
    out.opening = opening;
  }
  const speedRaw = speedLabelFromSubjectKey(subjectKey);
  if (speedRaw != null) {
    out.speed_label = speedRaw;
  }
  return out;
}
