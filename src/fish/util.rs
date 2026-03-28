//! Internal quoting utility shared across fish sub-modules.

/// Quote a fish value with single quotes when needed.
/// Bare values (alphanumeric + safe symbols) are emitted without quotes.
pub(super) fn quote_fish_value(s: &str) -> String {
    if s.chars()
        .all(|c| c.is_alphanumeric() || matches!(c, '_' | '-' | '.' | '/' | '~' | ':'))
    {
        s.to_string()
    } else {
        format!("'{}'", s.replace('\\', "\\\\").replace('\'', "\\'"))
    }
}
