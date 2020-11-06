#![allow(dead_code)]

use main_window::run_counter;
use rcon::RConArgs;
use rules::RulesFile;
use tf2process::{Tf2Process, Tf2ProcessArgs};

mod main_window;
mod player;
mod rcon;
mod rules;
mod steam_api;
mod tf2process;
mod utils;

fn main() {
    // test_read_rules_file();
    // test_ui();
    // test_tf2_process();
    // test_steam_api();

    test_rcon();
}

fn test_read_rules_file() {
    let filename = "rule_list.json";
    let data = RulesFile::from_file(filename);
    println!("{:#?}", data);
}

fn test_ui() {
    run_counter();
}

fn test_tf2_process() {
    let rcon_args = RConArgs::new();
    let args = Tf2ProcessArgs::new();
    let process = Tf2Process::start(&args, &rcon_args);
    process.wait();
}

fn test_steam_api() {}

fn test_rcon() {
    let rcon_args = RConArgs::new();
    // let args = Tf2ProcessArgs::new();
    // let process = Tf2Process::start(&args, &rcon_args);

    let mut client = rcon::RConClient::new(&rcon_args).unwrap();
    client.authorize().unwrap();
    println!("{}", client.exec_command(&"cvarlist".to_string()).unwrap());

    // process.wait();
}
