use owo_colors::OwoColorize;

use crate::layout::{fit, line, TerminalLayout};

pub fn render_code(lang: &str, code: &str) -> String {
    let layout = TerminalLayout::detect();
    let w = layout.card_width();
    let inner = w - 4;

    let mut out = String::new();
    out.push_str(&format!(
        "{}{}{}\n",
        "╭─ ".bright_black(),
        lang.bright_cyan(),
        format!(" {}", line('─', w.saturating_sub(lang.len() + 5))).bright_black()
    ));

    for raw in code.lines() {
        out.push_str(&format!(
            "{} {} {}\n",
            "│".bright_black(),
            fit(raw, inner),
            "│".bright_black()
        ));
    }

    out.push_str(&format!(
        "{}{}{}",
        "╰".bright_black(),
        line('─', w - 2),
        "╯".bright_black()
    ));

    out
}

pub fn render_diff(diff: &str) -> String {
    let layout = TerminalLayout::detect();
    let w = layout.card_width();
    let inner = w - 4;

    let mut out = String::new();
    out.push_str(&format!(
        "{}{}{}\n",
        "╭─ ".bright_black(),
        "diff".bright_magenta(),
        format!(" {}", line('─', w.saturating_sub(7))).bright_black()
    ));

    for raw in diff.lines() {
        let styled = if raw.starts_with('+') {
            raw.bright_green().to_string()
        } else if raw.starts_with('-') {
            raw.bright_red().to_string()
        } else if raw.starts_with("diff") {
            raw.bright_black().to_string()
        } else {
            raw.to_string()
        };

        out.push_str(&format!(
            "{} {} {}\n",
            "│".bright_black(),
            fit(&styled, inner),
            "│".bright_black()
        ));
    }

    out.push_str(&format!(
        "{}{}{}",
        "╰".bright_black(),
        line('─', w - 2),
        "╯".bright_black()
    ));

    out
}

pub fn render_markdown_light(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            if line.starts_with("# ") {
                line.trim_start_matches("# ").bright_cyan().bold().to_string()
            } else if line.starts_with("## ") {
                line.trim_start_matches("## ").bright_magenta().bold().to_string()
            } else if line.starts_with("- ") {
                format!("  {} {}", "•".bright_magenta(), line.trim_start_matches("- "))
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}
