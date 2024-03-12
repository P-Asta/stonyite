use std::process;

use clap::{Args, Parser, Subcommand};
mod dotfile;

#[derive(Debug, Parser)]
#[clap[author, version, about]]
pub struct Arg {
    pub command: String,
    #[arg(short, long)]
    pub manager: Option<String>,
}

fn main() {
    let mut log_builder = env_logger::builder();
    log_builder.filter_level(log::LevelFilter::Trace);
    log_builder.init();

    let mut args = Arg::parse();
    println!("{:?}", args);
    if args.manager.is_none() {
        args.manager = Some("brew".to_string());
    }

    match args.command.trim() {
        "setup" => dotfile::read_dotfile(args.manager.unwrap()),
        "test" => {
            process::Command::new("brew")
                .arg("install")
                .arg("fastfetch")
                .status()
                .unwrap();
        }
        _ => {}
    }
}
