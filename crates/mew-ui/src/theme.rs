#[derive(Debug, Clone)]
pub struct MewTheme {
    pub name: &'static str,
    pub description: &'static str,
}

pub const THEMES: &[MewTheme] = &[
    MewTheme {
        name: "crush-catppuccin",
        description: "soft rich catppuccin-inspired crush theme",
    },
    MewTheme {
        name: "crush-mocha",
        description: "deep soft terminal velvet",
    },
    MewTheme {
        name: "crush-rose",
        description: "warm rosy crush palette",
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
