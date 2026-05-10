// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Binary entry only delegates to `chessanalytics_lib::run` so integration tests can link the library crate directly.
fn main() {
    chessanalytics_lib::run()
}
