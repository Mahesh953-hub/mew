pub fn is_termux() -> bool {
    std::env::var("PREFIX")
        .map(|v| v.contains("com.termux"))
        .unwrap_or(false)
        || std::env::var("TERMUX_VERSION").is_ok()
}

pub fn os_name() -> &'static str {
    std::env::consts::OS
}

pub fn arch_name() -> &'static str {
    std::env::consts::ARCH
}
