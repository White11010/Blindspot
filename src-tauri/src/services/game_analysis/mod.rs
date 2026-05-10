// Full-game Stockfish pipeline: eval history, move classification, pattern tags, persistence, and similar-game lookup.
pub(crate) mod classifier;
mod engine_runner;
mod move_token;
mod key_insight;
mod key_moments;
pub mod model;
mod pattern_detector;
pub mod service;
mod system_connection;