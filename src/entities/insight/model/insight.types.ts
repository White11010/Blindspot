export interface Insight {
  id: string;

  user_id: string;

  kind: string;

  title: string;
  summary: string;

  severity: 'good' | 'info' | 'warning' | 'critical';

  confidence: number;

  metric_label?: string | null;
  metric_value?: string | null;

  recommendation?: string | null;

  payload_json?: string | null;

  created_at: number;
  expires_at?: number | null;
}
