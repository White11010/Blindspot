// One module per insight domain; each exposes `generate` consumed by `insights::service::regenerate_for_active_user`.
pub mod blunder_moments;
pub mod blunder_patterns;
pub mod openings;
pub mod opponent_rating;
pub mod psychology;
pub mod tactics_analysis;
pub mod tactics_phase_accuracy;
pub mod time_controls;
