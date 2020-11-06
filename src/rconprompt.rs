#![allow(dead_code)]
use rcon::{RConArgs, RConClient};
use std::io::Write;
use std::io::{stdin, stdout};
use structopt::StructOpt;

mod rcon;
mod utils;
#[derive(StructOpt, Debug)]
struct Options {
    #[structopt(short, long, default_value = "127.0.0.1")]
    ip: String,

    #[structopt(short, long)]
    port: u16,
}

fn main() {
    let options = Options::from_args();
    println!("{:?}", options);

    println!("Source RCON prompt");

    let mut rcon_args = RConArgs::new();
    rcon_args.ip = options.ip;
    rcon_args.port = options.port;

    println!("Connecting to {}:{}...", rcon_args.ip, rcon_args.port);

    let mut client = rcon::RConClient::new(&rcon_args).unwrap();
    println!("Authorizing...");

    client.authorize().unwrap();
    println!("Connected.");

    prompt(&mut client);
}

/// prompt is a read-eval-print-loop.
pub fn prompt(client: &mut RConClient) {
    println!("Enter rcon commands and press enter.");
    println!("!q to Quit.\n");

    loop {
        print!("{}", "> ");
        stdout().flush().unwrap();

        let mut cmd = String::new();
        stdin().read_line(&mut cmd).unwrap();
        trim_newline(&mut cmd);

        match cmd.as_str() {
            "!q" => break,
            _ => {
                let output = client.exec_command(&cmd).unwrap();
                println!("{}", output);
            }
        }
    }
}

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}
