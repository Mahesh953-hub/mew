use mew_common::MewConfig;
use mew_ui::{hint_card, startup_banner, tool_card};

#[test]
fn banner_renders() {
    let cfg = MewConfig::default();
    let out = startup_banner(&cfg, "test");
    assert!(out.contains("mew"));
}

#[test]
fn cards_render() {
    assert!(hint_card(&["hello"]).contains("hello"));
    assert!(tool_card("fs.read", "a.rs", "safe").contains("fs.read"));
}
