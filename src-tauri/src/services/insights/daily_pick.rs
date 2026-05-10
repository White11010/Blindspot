//! Picks one hero insight per local calendar day using history, metric deltas, and category spacing (see `pick_daily_insight`).

use std::collections::{HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{Duration, Local, NaiveDate};
use rusqlite::Connection;

use crate::db::insight_day_history::repository as day_hist;
use crate::db::insights::model::Insight;

/// Local `YYYY-MM-DD` string key for `insight_day_history` rows (matches user’s OS timezone expectations on Home).
pub fn local_today_ymd() -> String {
    Local::now().date_naive().format("%Y-%m-%d").to_string()
}

/// Scores candidates with anti-repeat rules; caller persists to keep DB writes out of pure selection for easier tests.
pub fn pick_daily_insight(
    conn: &Connection,
    user_id: &str,
    insights: &[Insight],
    today: &str,
) -> rusqlite::Result<Option<Insight>> {
    if insights.is_empty() {
        return Ok(None);
    }

    let today_nd =
        NaiveDate::parse_from_str(today, "%Y-%m-%d").unwrap_or_else(|_| Local::now().date_naive());
    let from_nd = today_nd - Duration::days(13);
    let from_s = from_nd.format("%Y-%m-%d").to_string();

    let rows = day_hist::list_history_since(conn, user_id, &from_s)?;
    let mut by_date: HashMap<String, String> = HashMap::new();
    let mut by_date_cat: HashMap<String, String> = HashMap::new();
    for (d, id, cat) in rows {
        by_date.insert(d.clone(), id);
        by_date_cat.insert(d, cat);
    }

    let mut banned_week: HashSet<String> = HashSet::new();
    for i in 1..=7 {
        if let Some(d) = today_nd.checked_sub_signed(Duration::days(i)) {
            let ds = d.format("%Y-%m-%d").to_string();
            if let Some(id) = by_date.get(&ds) {
                banned_week.insert(id.clone());
            }
        }
    }

    let mut last_cat_day: HashMap<String, NaiveDate> = HashMap::new();
    for i in 1..=30 {
        if let Some(d) = today_nd.checked_sub_signed(Duration::days(i)) {
            let ds = d.format("%Y-%m-%d").to_string();
            if let Some(cat) = by_date_cat.get(&ds) {
                last_cat_day.entry(cat.clone()).or_insert(d);
            }
        }
    }

    let mut scored: Vec<(f64, usize, &Insight)> = Vec::new();
    for (idx, ins) in insights.iter().enumerate() {
        if would_be_third_consecutive_day(&by_date, today_nd, &ins.id) {
            continue;
        }
        if banned_week.contains(&ins.id) {
            continue;
        }

        let mut score = ins.sort_priority as f64;
        if let (Some(c), Some(p)) = (ins.metric_number, ins.metric_prev) {
            score += (c - p).abs() * 3.0;
        }
        let days_since_cat = last_cat_day
            .get(&ins.category)
            .map(|d| (today_nd - *d).num_days().max(0) as f64)
            .unwrap_or(30.0);
        score += days_since_cat * 4.0;

        scored.push((score, idx, ins));
    }

    if scored.is_empty() {
        // Relax 7-day ban: keep 3-day rule only
        for (idx, ins) in insights.iter().enumerate() {
            if would_be_third_consecutive_day(&by_date, today_nd, &ins.id) {
                continue;
            }
            let mut score = ins.sort_priority as f64;
            if let (Some(c), Some(p)) = (ins.metric_number, ins.metric_prev) {
                score += (c - p).abs() * 3.0;
            }
            scored.push((score, idx, ins));
        }
    }

    if scored.is_empty() {
        // Last resort: any insight (still avoid 3-in-a-row if possible)
        let first = insights
            .iter()
            .find(|ins| !would_be_third_consecutive_day(&by_date, today_nd, &ins.id))
            .or_else(|| insights.first());
        return Ok(first.cloned());
    }

    scored.sort_by(|a, b| {
        b.0.partial_cmp(&a.0)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.1.cmp(&b.1))
    });

    let top_n = scored.len().min(5);
    let top: Vec<&Insight> = scored.iter().take(top_n).map(|(_, _, i)| *i).collect();
    let pick_idx = pseudo_random_index(top.len());
    Ok(Some(top[pick_idx].clone()))
}

fn would_be_third_consecutive_day(
    by_date: &HashMap<String, String>,
    today_nd: NaiveDate,
    candidate_id: &str,
) -> bool {
    let y1 = today_nd
        .checked_sub_signed(Duration::days(1))
        .map(|d| d.format("%Y-%m-%d").to_string());
    let y2 = today_nd
        .checked_sub_signed(Duration::days(2))
        .map(|d| d.format("%Y-%m-%d").to_string());
    match (y1, y2) {
        (Some(ref d1), Some(ref d2)) => {
            by_date.get(d1).map(|s| s.as_str()) == Some(candidate_id)
                && by_date.get(d2).map(|s| s.as_str()) == Some(candidate_id)
        }
        _ => false,
    }
}

fn pseudo_random_index(len: usize) -> usize {
    if len == 0 {
        return 0;
    }
    let n = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as usize)
        .unwrap_or(0);
    n % len
}
