use console_log_parser::{ConsoleLogParser, LogLineInfo};
use log_file_watcher::LogFileWatcher;
use preferences::Preferences;
use rcon::{RConArgs, RConClient};
use std::{thread, time};
use structopt::StructOpt;
use thread::sleep;

mod console_log_parser;
mod log_file_watcher;
mod player;
mod preferences;
mod rcon;
mod rules;
mod steam_api;
mod tf2process;
mod utils;

#[derive(StructOpt, Debug)]
struct Options {
    #[structopt(long)]
    pub ip: Option<String>,

    #[structopt(long)]
    port: Option<u16>,

    #[structopt(long)]
    password: Option<String>,

    #[structopt(long)]
    tf2_exe: Option<String>,

    #[structopt(long)]
    tf2_log_file: Option<String>,
}

fn main() {
    let options = Options::from_args();

    println!("Rust TF2 Bot Detector");

    let mut preferences = Preferences::load_or_default();

    // Update the preferences with what the user supplied.
    preferences.ip = options.ip.unwrap_or(preferences.ip);
    preferences.port = options.port.unwrap_or(preferences.port);
    preferences.password = options.password.unwrap_or(preferences.password);
    preferences.tf2_exe = options.tf2_exe.unwrap_or(preferences.tf2_exe);
    preferences.tf2_log_file = options.tf2_log_file.unwrap_or(preferences.tf2_log_file);

    // Save the updated values
    preferences.save();

    println!("Using settings: {:#?}", preferences);

    let mut bot_detector = RustBotDetector::new(preferences);
    bot_detector.start();
}

#[derive(Debug)]
struct RustBotDetector {
    preferences: Preferences,
}

impl RustBotDetector {
    pub fn new(preferences: Preferences) -> Self {
        RustBotDetector { preferences }
    }

    pub fn start(&mut self) {
        let parser = ConsoleLogParser::new();
        let mut log_file_watcher =
            LogFileWatcher::new(self.preferences.tf2_log_file.as_str(), parser);

        let mut rcon_args = RConArgs::new();
        rcon_args.ip = self.preferences.ip.clone();
        rcon_args.port = self.preferences.port;
        rcon_args.password = self.preferences.password.clone();
        let mut rcon_client = RConClient::new(&rcon_args).unwrap();
        rcon_client.authorize().unwrap();

        let rcon_delay = time::Duration::from_millis(500);
        let loop_delay = time::Duration::from_millis(3000);
        loop {
            Self::send_rcon_command(&self.preferences, &"status".to_string());
            sleep(rcon_delay);

            let lines = log_file_watcher.process_new_data();
            if !lines.is_empty() {
                println!("");
            }

            for line in lines {
                match line {
                    LogLineInfo::Nothing => {
                        // Don't spam the console with Nothings
                    }
                    LogLineInfo::PlayerInfo {
                        steam_id: _,
                        name: _,
                        id: _,
                    } => {
                        println!("{:?}", line);
                    }
                }
            }

            sleep(loop_delay);
        }
    }

    fn send_rcon_command(preferences: &Preferences, cmd: &String) {
        let mut rcon_args = RConArgs::new();
        rcon_args.ip = preferences.ip.clone();
        rcon_args.port = preferences.port;
        rcon_args.password = preferences.password.clone();

        let mut rcon_client = RConClient::new(&rcon_args);
        match rcon_client {
            Ok(mut client) => match client.authorize() {
                Ok(_) => match client.exec_command(cmd) {
                    Ok(_) => {}
                    Err(error) => {
                        println!("RCON: exec_command failed: {:?}", error);
                    }
                },
                Err(error) => println!("RCON: auth failed: {:?}", error),
            },
            Err(error) => println!("RCON: connect failed: {:?}", error),
        }
    }
}
