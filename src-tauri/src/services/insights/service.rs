use tauri::AppHandle;

use crate::db::connection::get_conn;
use crate::db::game_analyses::repository as ga_repository;
use crate::db::games::repository as games_repository;
use crate::db::insight_day_history::repository as day_history_repository;
use crate::db::insights::model::Insight;
use crate::db::insights::repository as insights_repository;
use crate::db::users::repository as users_repository;

use super::daily_pick;
use super::generators::{
    blunder_moments, blunder_patterns, openings, psychology, tactics_analysis, time_controls,
};
use super::insight_common::apply_metric_prev;

pub fn regenerate_for_active_user(app: AppHandle) -> Result<Vec<Insight>, String> {
    let conn = get_conn(&app)?;

    let user = users_repository::get_active_user(&conn)?.ok_or("Active user not found")?;

    let previous = insights_repository::get_user_insights(&conn, &user.id).unwrap_or_default();

    let games = games_repository::get_games_by_username(&conn, &user.username, 1000)
        .map_err(|e| e.to_string())?;

    let game_ids: Vec<String> = games.iter().map(|g| g.id.clone()).collect();
    let analyses = ga_repository::load_done_analyses_by_game_ids(&conn, &game_ids)
        .map_err(|e| e.to_string())?;
    let moments = ga_repository::load_key_moments_by_game_ids(&conn, &game_ids)
        .map_err(|e| e.to_string())?;

    let mut insights: Vec<Insight> = Vec::new();

    insights.extend(openings::generate(&user.id, &games));
    insights.extend(time_controls::generate(&user.id, &games));
    insights.extend(psychology::generate(&user.id, &games));
    insights.extend(tactics_analysis::generate(
        &user.id,
        &games,
        &analyses,
        &moments,
    ));
    insights.extend(blunder_patterns::generate(&user.id, &games));
    insights.extend(blunder_moments::generate(&user.id, &games));

    apply_metric_prev(&mut insights, &previous);

    insights_repository::replace_user_insights(&conn, &user.id, &insights)
        .map_err(|e| e.to_string())?;

    let today = daily_pick::local_today_ymd();
    day_history_repository::delete_pick_for_date(&conn, &user.id, &today).map_err(|e| e.to_string())?;

    Ok(insights)
}

/// Resolves today's hero insight with rotation rules; persists pick in `insight_day_history`.
pub fn get_daily_insight_for_active_user(app: AppHandle) -> Result<Option<Insight>, String> {
    let conn = get_conn(&app)?;

    let user = users_repository::get_active_user(&conn)?.ok_or("Active user not found")?;

    let insights = insights_repository::get_user_insights(&conn, &user.id).map_err(|e| e.to_string())?;
    let today = daily_pick::local_today_ymd();

    if let Some((id, _)) =
        day_history_repository::get_pick_for_date(&conn, &user.id, &today).map_err(|e| e.to_string())?
    {
        if let Some(ins) = insights.iter().find(|i| i.id == id) {
            return Ok(Some(ins.clone()));
        }
        day_history_repository::delete_pick_for_date(&conn, &user.id, &today).map_err(|e| e.to_string())?;
    }

    let picked = daily_pick::pick_daily_insight(&conn, &user.id, &insights, &today).map_err(|e| e.to_string())?;
    if let Some(ref ins) = picked {
        day_history_repository::insert_pick(&conn, &user.id, &today, &ins.id, &ins.category)
            .map_err(|e| e.to_string())?;
    }

    Ok(picked)
}

pub fn get_for_active_user(app: AppHandle) -> Result<Vec<Insight>, String> {
    let conn = get_conn(&app)?;

    let user = users_repository::get_active_user(&conn)?.ok_or("Active user not found")?;

    insights_repository::get_user_insights(&conn, &user.id).map_err(|e| e.to_string())
}
