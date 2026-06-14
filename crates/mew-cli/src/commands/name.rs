use anyhow::Result;
use mew_common::{MewConfig, MewPaths};
use rand::seq::SliceRandom;

use crate::args::{NameCommand, NameSubcommand};

const NAMES: &[&str] = &[
    "mew",
    "purr",
    "paww",
    "nyan",
    "nekko",
    "kiki",
    "miko",
    "mimi",
    "luma",
    "bytecat",
    "clawdia",
    "sir-pounce",
    "patchpaw",
    "tinyclaw",
    "cavecat",
    "codemew",
];

pub fn run(paths: &MewPaths, cfg: &mut MewConfig, cmd: NameCommand) -> Result<()> {
    match cmd.command {
        NameSubcommand::Show => {
            println!("{}", cfg.identity.display_name);
        }
        NameSubcommand::Set { name } => {
            cfg.identity.display_name = name;
            cfg.save(paths)?;
            println!("saved");
        }
        NameSubcommand::Random => {
            let mut rng = rand::thread_rng();
            cfg.identity.display_name = NAMES.choose(&mut rng).unwrap().to_string();
            cfg.save(paths)?;
            println!("{}", cfg.identity.display_name);
        }
        NameSubcommand::Reset => {
            cfg.identity.display_name = "mew".to_string();
            cfg.save(paths)?;
            println!("mew");
        }
    }

    Ok(())
}
