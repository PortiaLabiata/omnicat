mod json;
mod transport_wrapper;

use json::*;
use transport::*;

use std::collections::HashSet;
use std::fs::File;
use std::process::exit;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct ArgsRun {
    command: String,

    #[arg(default_value = "ocat.json")]
    filename: String,
}

async fn run_command() {
    let args = ArgsRun::parse();
    let reader = match File::open(&args.filename) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open file {}: {}", &args.filename, e.to_string());
            exit(1);
        }
    };

    let config: JsonConfigs = match serde_json::from_reader(reader) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to parse file {}: {}", &args.filename, e.to_string());
            exit(1);
        }
    };

    println!("Loaded config {} OK", &args.filename);

    let transport_num = config.configs.len();
    let mut transports = Vec::with_capacity(transport_num);
    for cfg in config.configs.iter() {
        let transport = match create_transport(cfg).await {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Failed to create transport {}: {:?}", cfg.name, e);
                exit(1);
            }
        };

        println!("Found transport \"{}\" with kind {:?}", cfg.name, cfg.kind);
        transports.push(transport);
    }

    let mut unchecked_set = HashSet::with_capacity(transport_num);
    for transport in transports.iter() {
        unchecked_set.insert(transport);
    }

    let mut pending_futures = Vec::with_capacity(transport_num);
    loop {
        for transport in unchecked_set.drain() {
            let future = transport.transport.receive(&mut transport.rxbuf, 0);
            pending_futures.push(future);
        }
    }
}

async fn help_command() {}

#[tokio::main]
async fn main() {
    let args: Vec<_> = std::env::args().take(2).collect();

    if args.len() < 2 {
        eprintln!("Invalid number of arguments: need at least one command");
        exit(1);
    }

    let command = &args[1][..];
    match command {
        "run" => {
            run_command().await;
        }

        "help" => {
            help_command().await;
        }

        _ => {
            eprintln!("Invalid command: {}", command);
            exit(1);
        }
    }
}
