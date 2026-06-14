use anyhow::Result;
use mew_common::{MewConfig, MewPaths};

use crate::args::{ConfigCommand, ConfigSubcommand};

pub fn run(paths: &MewPaths, cfg: &MewConfig, cmd: ConfigCommand) -> Result<()> {
    match cmd.command {
        ConfigSubcommand::Path => {
            println!("{}", paths.config_file.display());
        }
        ConfigSubcommand::Show => {
            println!("{}", toml::to_string_pretty(cfg)?);
        }
    }

    Ok(())
}
