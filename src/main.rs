use std::process;

use clap::{Args, Parser, Subcommand};
mod dotfile;

mod config;
mod install;

#[derive(Debug, Parser)]
#[clap[author, version, about]]
pub struct Arg {
    pub command: String,
    #[arg(short, long)]
    pub manager: Option<String>,
    #[arg(short, long)]
    pub flag: Option<String>,
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
        "setup" => dotfile::read_dotfile(
            args.manager.unwrap(),
            args.flag,
            dotfile::DotfileConfig::None,
        ),
        "config" => dotfile::read_dotfile(
            args.manager.unwrap(),
            args.flag,
            dotfile::DotfileConfig::JustConfig,
        ),
        "install" => dotfile::read_dotfile(
            args.manager.unwrap(),
            args.flag,
            dotfile::DotfileConfig::JustInstall,
        ),
        _ => {}
    }
}
