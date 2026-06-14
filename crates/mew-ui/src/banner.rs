use mew_common::MewConfig;
use owo_colors::OwoColorize;

use crate::layout::{center, fit, kv_line, line, ScreenClass, TerminalLayout};
use crate::phrases::phrase;

pub fn startup_banner(cfg: &MewConfig, project_status: &str) -> String {
    let layout = TerminalLayout::detect();
    match layout.class {
        ScreenClass::Tiny => tiny_banner(cfg, project_status, layout),
        ScreenClass::Narrow => narrow_banner(cfg, project_status, layout),
        ScreenClass::Normal => normal_banner(cfg, project_status, layout),
        ScreenClass::Wide => wide_banner(cfg, project_status, layout),
    }
}

fn tiny_banner(cfg: &MewConfig, project_status: &str, layout: TerminalLayout) -> String {
    let w = layout.card_width();
    let inner = w - 4;
    let name = format!("{} agent", cfg.identity.display_name);
    let tagline = phrase("startup");

    vec![
        format!("{}", "╭".bright_black()) + &line('─', w - 2) + &format!("{}", "╮".bright_black()),
        format!("{} {} {}", "│".bright_black(), fit("/\\_/\\\\", inner), "│".bright_black()),
        format!("{} {} {}", "│".bright_black(), fit("( o.o )", inner), "│".bright_black()),
        format!("{} {} {}", "│".bright_black(), fit(" > ^ <", inner), "│".bright_black()),
        format!("{}{}{}", "├".bright_black(), line('─', w - 2), "┤".bright_black()),
        format!("{} {} {}", "│".bright_black(), fit(&name, inner).bright_cyan(), "│".bright_black()),
        format!("{} {} {}", "│".bright_black(), fit(tagline, inner), "│".bright_black()),
        format!("{}{}{}", "├".bright_black(), line('─', w - 2), "┤".bright_black()),
        format!("{} {} {}", "│".bright_black(), fit(&kv_line("model", &cfg.providers.default, inner), inner), "│".bright_black()),
        format!("{} {} {}", "│".bright_black(), fit(&kv_line("project", project_status, inner), inner), "│".bright_black()),
        format!("{} {} {}", "│".bright_black(), fit(&kv_line("style", &cfg.style.theme, inner), inner), "│".bright_black()),
        format!("{}", "╰".bright_black()) + &line('─', w - 2) + &format!("{}", "╯".bright_black()),
    ]
    .join("\n")
}

fn narrow_banner(cfg: &MewConfig, project_status: &str, layout: TerminalLayout) -> String {
    let w = layout.card_width();
    let inner = w - 4;
    let name = format!("{} agent", cfg.identity.display_name);
    let tagline = phrase("startup");

    vec![
        format!("{}", "╭".bright_black()) + &line('─', w - 2) + &format!("{}", "╮".bright_black()),
        format!("{} {} {}", "│".bright_black(), center("╱|、", inner).bright_magenta(), "│".bright_black()),
        format!("{} {} {}", "│".bright_black(), center("(˚ˎ 。7", inner).bright_magenta(), "│".bright_black()),
        format!("{} {} {}", "│".bright_black(), center("|、˜〵", inner).bright_magenta(), "│".bright_black()),
        format!("{} {} {}", "│".bright_black(), center("じしˍ,)ノ", inner).bright_magenta(), "│".bright_black()),
        format!("{}{}{}", "├".bright_black(), line('─', w - 2), "┤".bright_black()),
        format!("{} {} {}", "│".bright_black(), center(&name, inner).bright_cyan(), "│".bright_black()),
        format!("{} {} {}", "│".bright_black(), center(tagline, inner), "│".bright_black()),
        format!("{}{}{}", "├".bright_black(), line('─', w - 2), "┤".bright_black()),
        format!("{} {} {}", "│".bright_black(), fit(&kv_line("model", &cfg.providers.default, inner), inner), "│".bright_black()),
        format!("{} {} {}", "│".bright_black(), fit(&kv_line("project", project_status, inner), inner), "│".bright_black()),
        format!("{} {} {}", "│".bright_black(), fit(&kv_line("style", &cfg.style.theme, inner), inner), "│".bright_black()),
        format!("{}", "╰".bright_black()) + &line('─', w - 2) + &format!("{}", "╯".bright_black()),
    ]
    .join("\n")
}

fn normal_banner(cfg: &MewConfig, project_status: &str, layout: TerminalLayout) -> String {
    let w = layout.card_width();
    let inner = w - 4;
    let left = 23;
    let right = inner - left - 2;
    let name = format!("{} agent", cfg.identity.display_name);
    let tagline = phrase("startup");

    vec![
        format!("{}", "╭".bright_black()) + &line('─', w - 2) + &format!("{}", "╮".bright_black()),
        format!(
            "{} {}  {} {}",
            "│".bright_black(),
            fit("     /\\_/\\\\", left).bright_magenta(),
            fit(&name, right).bright_cyan(),
            "│".bright_black()
        ),
        format!(
            "{} {}  {} {}",
            "│".bright_black(),
            fit("    ( o.o )", left).bright_magenta(),
            fit(tagline, right),
            "│".bright_black()
        ),
        format!(
            "{} {}  {} {}",
            "│".bright_black(),
            fit("     > ^ <", left).bright_magenta(),
            fit("CLI-first · safe · fast", right).bright_black(),
            "│".bright_black()
        ),
        format!("{}{}{}", "├".bright_black(), line('─', w - 2), "┤".bright_black()),
        format!(
            "{} {}  {} {}",
            "│".bright_black(),
            fit(&kv_line("model", &cfg.providers.default, left), left),
            fit(&kv_line("project", project_status, right), right),
            "│".bright_black()
        ),
        format!(
            "{} {}  {} {}",
            "│".bright_black(),
            fit(&kv_line("style", &cfg.style.theme, left), left),
            fit(&kv_line("guard", "awake", right), right),
            "│".bright_black()
        ),
        format!("{}", "╰".bright_black()) + &line('─', w - 2) + &format!("{}", "╯".bright_black()),
    ]
    .join("\n")
}

fn wide_banner(cfg: &MewConfig, project_status: &str, layout: TerminalLayout) -> String {
    let w = layout.card_width();
    let inner = w - 4;
    let left = 30;
    let right = inner - left - 2;
    let name = format!("{} agent", cfg.identity.display_name);
    let tagline = phrase("startup");

    vec![
        format!("{}", "╭".bright_black()) + &line('─', w - 2) + &format!("{}", "╮".bright_black()),
        format!(
            "{} {}  {} {}",
            "│".bright_black(),
            fit("        ╱|、", left).bright_magenta(),
            fit(&name, right).bright_cyan(),
            "│".bright_black()
        ),
        format!(
            "{} {}  {} {}",
            "│".bright_black(),
            fit("      (˚ˎ 。7", left).bright_magenta(),
            fit(tagline, right),
            "│".bright_black()
        ),
        format!(
            "{} {}  {} {}",
            "│".bright_black(),
            fit("       |、˜〵", left).bright_magenta(),
            fit("CLI-first · token-smart · guard-protected", right).bright_black(),
            "│".bright_black()
        ),
        format!(
            "{} {}  {} {}",
            "│".bright_black(),
            fit("       じしˍ,)ノ", left).bright_magenta(),
            fit("from Termux caves to x86 castles", right).bright_black(),
            "│".bright_black()
        ),
        format!("{}{}{}", "├".bright_black(), line('─', w - 2), "┤".bright_black()),
        format!(
            "{} {}  {} {}",
            "│".bright_black(),
            fit(&kv_line("model", &cfg.providers.default, left), left),
            fit(&kv_line("project", project_status, right), right),
            "│".bright_black()
        ),
        format!(
            "{} {}  {} {}",
            "│".bright_black(),
            fit(&kv_line("style", &cfg.style.theme, left), left),
            fit(&kv_line("guard", "awake", right), right),
            "│".bright_black()
        ),
        format!("{}", "╰".bright_black()) + &line('─', w - 2) + &format!("{}", "╯".bright_black()),
    ]
    .join("\n")
}
