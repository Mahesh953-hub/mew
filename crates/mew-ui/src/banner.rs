use mew_common::MewConfig;
use owo_colors::OwoColorize;

use crate::phrases::phrase;

pub fn startup_banner(cfg: &MewConfig, project_status: &str) -> String {
    let name = &cfg.identity.display_name;
    let tagline = phrase("startup");

    format!(
        "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
        "╭────────────────────────────────────────────╮".bright_magenta(),
        "│  /\\_/\\\\                                    │".bright_magenta(),
        format!("│ ( o.o )   {:<31}│", format!("{name} agent")).bright_cyan(),
        format!("│  > ^ <    {:<31}│", tagline).bright_cyan(),
        "├────────────────────────────────────────────┤".bright_magenta(),
        format!("│  model     {:<31}│", cfg.providers.default).bright_white(),
        format!("│  project   {:<31}│", project_status).bright_white(),
        format!("│  style     {:<31}│", cfg.style.theme).bright_white(),
        "╰────────────────────────────────────────────╯".bright_magenta(),
    )
}
