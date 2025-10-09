#[expect(clippy::needless_raw_strings, reason = "Sometimes special characters are needed for this raw string.")]
pub const DEFAULT_SETTINGS_RAW:&str =
r#"
[spell_selector]
auto_refresh = true
"#;