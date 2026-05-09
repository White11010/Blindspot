use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Insight {
    pub id: String,

    pub user_id: String,

    pub kind: String,

    pub title: String,
    pub summary: String,

    pub severity: String,

    pub confidence: i64,

    pub metric_label: Option<String>,
    pub metric_value: Option<String>,

    pub recommendation: Option<String>,

    pub payload_json: Option<String>,

    /// `openings` | `time` | `tactics` | `psychology`
    pub category: String,

    /// Primary numeric metric for deltas (e.g. win rate %, streak length).
    pub metric_number: Option<f64>,

    /// Same metric from the previous regeneration when `subject_key` matches.
    pub metric_prev: Option<f64>,

    /// Higher = more prominent in the "All" tab.
    pub sort_priority: i64,

    pub created_at: i64,
    pub expires_at: Option<i64>,
}
