use owo_colors::OwoColorize;

pub fn hint_card(lines: &[&str]) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "{}\n",
        "╭─ hint ─────────────────────────────────────╮".bright_blue()
    ));
    for line in lines {
        out.push_str(&format!("│ {:<42} │\n", line));
    }
    out.push_str(&format!(
        "{}",
        "╰────────────────────────────────────────────╯".bright_blue()
    ));
    out
}

pub fn error_card(title: &str, body: &str) -> String {
    format!(
        "{}\n│ {:<42} │\n│ {:<42} │\n{}",
        format!("╭─ hiss! {:<31}╮", title).bright_red(),
        "something scratched back",
        body,
        "╰────────────────────────────────────────────╯".bright_red()
    )
}

pub fn tool_card(tool: &str, target: &str, risk: &str) -> String {
    format!(
        "{}\n│ tool      {:<31}│\n│ target    {:<31}│\n│ risk      {:<31}│\n{}",
        "╭─ paw step ─────────────────────────────────╮".bright_magenta(),
        tool,
        target,
        risk,
        "╰────────────────────────────────────────────╯".bright_magenta()
    )
}

pub fn code_block(lang: &str, code: &str) -> String {
    format!("```{}\n{}\n```", lang, code)
}

pub fn diff_sample() -> String {
    [
        "diff --git a/src/main.rs b/src/main.rs",
        "- println!(\"hello\");",
        "+ println!(\"mew~\");",
    ]
    .join("\n")
}
