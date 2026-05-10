// Shared helpers for building `Insight` rows: category string constants, payload JSON, and metric_prev carry-over on regen.
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::Deserialize;

use crate::db::insights::model::Insight;

pub const CAT_OPENINGS: &str = "openings"; // UI tab + generator grouping for repertoire and ECO-style stats.
pub const CAT_TIME: &str = "time"; // Clock / session timing patterns (hour-of-day, speed mix).
pub const CAT_TACTICS: &str = "tactics"; // Blunders, phases, color performance, opponent rating buckets.
pub const CAT_PSYCHOLOGY: &str = "psychology"; // Streaks, tilt proxies, rest-day effects; interpretive stats.

#[derive(Debug, Deserialize)]
struct PayloadSk {
    #[serde(default)]
    subject_key: Option<String>,
}

/// Millisecond timestamps for `created_at` on generated cards (wall clock, same basis as other services).
pub fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}

/// Minimal JSON payload carrying only `subject_key` for Vue deep links and dedupe across regenerations.
pub fn payload_with_subject(subject_key: &str) -> Option<String> {
    payload_with_subject_and_params(subject_key, serde_json::json!({}))
}

/// Same as `payload_with_subject` but merges a `params` object for charts that need extra context (e.g. opening name).
pub fn payload_with_subject_and_params(subject_key: &str, params: serde_json::Value) -> Option<String> {
    let mut m = serde_json::Map::new();
    m.insert(
        "subject_key".to_string(),
        serde_json::Value::String(subject_key.to_string()),
    );
    if let serde_json::Value::Object(ref pmap) = params {
        if !pmap.is_empty() {
            m.insert("params".to_string(), params);
        }
    }
    serde_json::to_string(&serde_json::Value::Object(m)).ok()
}

/// Extracts stable subject key from stored JSON, or `default` when older rows lack payload (safe metric_prev match).
pub fn subject_key_from_insight(ins: &Insight) -> String {
    ins.payload_json
        .as_ref()
        .and_then(|j| serde_json::from_str::<PayloadSk>(j).ok())
        .and_then(|p| p.subject_key)
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "default".to_string())
}

/// Copies prior `metric_number` into `metric_prev` when kind+subject match so UI can show week-over-week arrows.
pub fn apply_metric_prev(insights: &mut [Insight], previous: &[Insight]) {
    let mut map: HashMap<(String, String), f64> = HashMap::new();
    for old in previous {
        let sk = subject_key_from_insight(old);
        if let Some(n) = old.metric_number {
            map.insert((old.kind.clone(), sk), n);
        }
    }

    for ins in insights.iter_mut() {
        if ins.metric_number.is_none() {
            continue;
        }
        let k = ins.kind.clone();
        let sk = subject_key_from_insight(ins);
        if let Some(prev) = map.get(&(k, sk)) {
            ins.metric_prev = Some(*prev);
        }
    }
}

/// Central factory for `Insight` structs so generators stay declarative (timestamps, payload, default prev=null).
#[allow(clippy::too_many_arguments)]
pub fn build_insight(
    id: String,
    user_id: &str,
    kind: &str,
    category: &str,
    title: String,
    summary: String,
    severity: &str,
    confidence: i64,
    metric_label: Option<String>,
    metric_value: Option<String>,
    metric_number: Option<f64>,
    recommendation: Option<String>,
    subject_key: &str,
    sort_priority: i64,
    params: serde_json::Value,
) -> Insight {
    Insight {
        id,
        user_id: user_id.to_string(),
        kind: kind.to_string(),
        title,
        summary,
        severity: severity.to_string(),
        confidence,
        metric_label,
        metric_value,
        recommendation,
        payload_json: payload_with_subject_and_params(subject_key, params),
        category: category.to_string(),
        metric_number,
        metric_prev: None,
        sort_priority,
        created_at: now_ms(),
        expires_at: None,
    }
}
