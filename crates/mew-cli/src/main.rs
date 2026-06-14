mod args;
mod commands;

use anyhow::Result;
use args::{Cli, Commands};
use clap::Parser;
use mew_common::{MewConfig, MewPaths};
use mew_ui::{hint_card, startup_banner};

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .without_time()
        .with_env_filter("warn")
        .init();

    let cli = Cli::parse();
    let paths = MewPaths::discover()?;
    let mut cfg = MewConfig::load_or_create(&paths)?;

    match cli.command {
        Some(Commands::Doctor) => commands::doctor::run(&paths, &cfg)?,
        Some(Commands::Init { dry_run }) => commands::init::run(&paths, &cfg, dry_run)?,
        Some(Commands::Name(cmd)) => commands::name::run(&paths, &mut cfg, cmd)?,
        Some(Commands::Style(cmd)) => commands::style::run(&paths, &mut cfg, cmd)?,
        Some(Commands::Config(cmd)) => commands::config::run(&paths, &cfg, cmd)?,
        None => {
            println!("{}", startup_banner(&cfg, "not initialized"));
            println!();
            println!(
                "{}",
                hint_card(&[
                    "try: mew init",
                    "try: mew ask \"what does this repo do?\"",
                    "try: mew style preview",
                ])
            );
        }
    }

    Ok(())
}
