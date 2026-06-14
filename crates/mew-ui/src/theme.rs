#[derive(Debug, Clone)]
pub struct MewTheme {
    pub name: &'static str,
    pub description: &'static str,
}

pub const THEMES: &[MewTheme] = &[
    MewTheme {
        name: "crush-catppuccin",
        description: "minimal soft purple/cyan crush aesthetic",
    },
    MewTheme {
        name: "claude-minimal",
        description: "clean low-noise Claude Code-like terminal style",
    },
    MewTheme {
        name: "vector-mocha",
        description: "vector-card dark mocha blocks",
    },
    MewTheme {
        name: "crush-rose",
        description: "warm rose rich terminal cards",
    },
    MewTheme {
        name: "mew-dark",
        description: "dark cozy terminal fur",
    },
    MewTheme {
        name: "mew-cave",
        description: "caveman low-token stone mode",
    },
    MewTheme {
        name: "mono",
        description: "clean monochrome",
    },
    MewTheme {
        name: "no-color",
        description: "plain output for strict terminals",
    },
];

pub fn theme_exists(name: &str) -> bool {
    THEMES.iter().any(|t| t.name == name)
}
