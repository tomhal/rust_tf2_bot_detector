#![allow(dead_code)]
// #![warn(
//     clippy::all,
//     //clippy::restriction,
//     clippy::pedantic,
//     clippy::nursery,
//     clippy::cargo
// )]
//#![allow(clippy::non_ascii_literal)]
// #![allow(clippy::missing_errors_doc)]

// use main_window::run_counter;

// mod main_window;
mod console_log;
mod console_log_parser_line_based;
mod log_file_watcher;
mod player;
mod preferences;
mod rcon;
mod rules;
mod steam_api;
mod tf2process;
mod utils;

// See the main()s in rust_bot_detector.rs and rconprompt.rs instead.
fn main() {}

// fn test_read_rules_file() {
//     let filename = "rule_list.json";
//     let data = RulesFile::from_file(filename);
//     println!("{:#?}", data);
// }
