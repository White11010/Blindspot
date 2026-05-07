use rusqlite::{params, Connection};

use crate::db::game_analyses::repository as ga_repo;

use super::model::SystemConnection;
use super::pattern_detector::primary_tag;

pub fn build_system_connection(
    conn: &Connection,
    username: &str,
    user_id: &str,
    tags: &[(String, i64)],
) -> Result<SystemConnection, String> {
    let window: i32 = 10;
    if tags.is_empty() {
        return Ok(SystemConnection {
            text: "Play more rated games to unlock deeper pattern stats.".into(),
            tag: "general".into(),
            count: 0,
            window,
            secondary_text: None,
        });
    }

    let tag = primary_tag(tags).unwrap_or_else(|| tags[0].0.clone());

    let count = ga_repo::count_similar_in_recent(conn, username, user_id, &tag, window as u32)
        .map_err(|e| e.to_string())?;

    let text = if count <= 1 {
        format!(
            "This pattern ({}) appears in your recent games — keep an eye on it.",
            tag.replace('_', " ")
        )
    } else {
        format!(
            "This is your {} similar case in last {} games.",
            ordinal(count),
            window
        )
    };

    let secondary = winrate_note(conn, username, user_id, &tag).ok();

    Ok(SystemConnection {
        text,
        tag,
        count,
        window,
        secondary_text: secondary,
    })
}

fn ordinal(n: i64) -> String {
    let s = match n % 100 {
        11..=13 => format!("{}th", n),
        _ => match n % 10 {
            1 => format!("{}st", n),
            2 => format!("{}nd", n),
            3 => format!("{}rd", n),
            _ => format!("{}th", n),
        },
    };
    s
}

fn winrate_note(
    conn: &Connection,
    username: &str,
    user_id: &str,
    tag: &str,
) -> Result<String, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "
        SELECT g.player_result, COUNT(*) as c
        FROM games g
        INNER JOIN game_pattern_tags t ON t.game_id = g.id AND t.user_id = ?1 AND t.tag = ?2
        WHERE g.username = ?3
        GROUP BY g.player_result
        ",
    )?;

    let rows = stmt.query_map(params![user_id, tag, username], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
    })?;

    let mut wins = 0i64;
    let mut losses = 0i64;
    let mut draws = 0i64;
    for r in rows {
        let (res, c) = r?;
        match res.as_str() {
            "win" => wins += c,
            "loss" => losses += c,
            _ => draws += c,
        }
    }

    let total = wins + losses + draws;
    if total < 5 {
        return Ok(format!(
            "You often revisit positions tagged \"{}\" — review a few model games.",
            tag.replace('_', " ")
        ));
    }

    let wr = (wins as f64 + draws as f64 * 0.5) / total as f64 * 100.0;
    Ok(format!(
        "In {} games with this profile you score {:.0}% (wins/draws).",
        total, wr
    ))
}
