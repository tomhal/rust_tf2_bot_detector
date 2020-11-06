#![allow(dead_code)]
use rcon::{RConArgs, RConClient};
use std::io::Write;
use std::io::{stdin, stdout};
use structopt::StructOpt;

mod rcon;
mod utils;
#[derive(StructOpt, Debug)]
struct Options {
    #[structopt(long, default_value = "127.0.0.1")]
    ip: String,

    #[structopt(long)]
    port: u16,

    #[structopt(long)]
    password: String,
}

fn main() {
    let options = Options::from_args();
    println!("{:?}", options);

    println!("Source RCON prompt");

    let mut rcon_args = RConArgs::new();
    rcon_args.ip = options.ip;
    rcon_args.port = options.port;
    rcon_args.password = options.password;

    println!(
        "Connecting to {}:{} with password '{}'...",
        rcon_args.ip, rcon_args.port, rcon_args.password
    );

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
