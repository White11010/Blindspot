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

    pub created_at: i64,
    pub expires_at: Option<i64>,
}
