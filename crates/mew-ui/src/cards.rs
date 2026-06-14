use owo_colors::OwoColorize;

use crate::layout::{fit, line, wrap_lines, TerminalLayout};

pub fn hint_card(lines: &[&str]) -> String {
    let layout = TerminalLayout::detect();
    let w = layout.card_width();
    let inner = w - 4;
    let mut out = String::new();

    out.push_str(&format!(
        "{}{}{}\n",
        "╭─ ".bright_black(),
        "hint".bright_blue(),
        format!(" {}", line('─', w.saturating_sub(8))).bright_black()
    ));

    for line_text in lines {
        for wrapped in wrap_lines(line_text, inner) {
            out.push_str(&format!(
                "{} {} {}\n",
                "│".bright_black(),
                fit(&wrapped, inner),
                "│".bright_black()
            ));
        }
    }

    out.push_str(&format!(
        "{}{}{}",
        "╰".bright_black(),
        line('─', w - 2),
        "╯".bright_black()
    ));

    out
}

pub fn error_card(title: &str, body: &str) -> String {
    let layout = TerminalLayout::detect();
    let w = layout.card_width();
    let inner = w - 4;
    let mut out = String::new();

    out.push_str(&format!(
        "{}{}{}\n",
        "╭─ ".bright_black(),
        format!("hiss! {title}").bright_red(),
        format!(" {}", line('─', w.saturating_sub(title.len() + 10))).bright_black()
    ));

    for wrapped in wrap_lines(body, inner) {
        out.push_str(&format!(
            "{} {} {}\n",
            "│".bright_black(),
            fit(&wrapped, inner),
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

pub fn tool_card(tool: &str, target: &str, risk: &str) -> String {
    let layout = TerminalLayout::detect();
    let w = layout.card_width();
    let inner = w - 4;

    let rows = [
        format!("tool    {tool}"),
        format!("target  {target}"),
        format!("risk    {risk}"),
    ];

    let mut out = String::new();
    out.push_str(&format!(
        "{}{}{}\n",
        "╭─ ".bright_black(),
        "paw step".bright_magenta(),
        format!(" {}", line('─', w.saturating_sub(12))).bright_black()
    ));

    for row in rows {
        out.push_str(&format!(
            "{} {} {}\n",
            "│".bright_black(),
            fit(&row, inner),
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

pub fn code_block(lang: &str, code: &str) -> String {
    crate::render::render_code(lang, code)
}

pub fn diff_sample() -> String {
    crate::render::render_diff(
        r#"diff --git a/src/main.rs b/src/main.rs
- println!("hello");
+ println!("mew~");"#,
    )
}
