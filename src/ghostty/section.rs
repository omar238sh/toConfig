use super::core::{GhosttyConfig, GhosttyRenderContext};

/// A comment line in the Ghostty config file.
///
/// ```text
/// # This is a comment
/// ```
#[derive(Clone, Debug)]
pub struct GhosttyComment(pub String);

impl GhosttyComment {
    pub fn new(text: impl Into<String>) -> Self {
        Self(text.into())
    }
}

impl GhosttyConfig for GhosttyComment {
    fn render(&self, _ctx: &GhosttyRenderContext) -> String {
        let mut out = String::new();
        for line in self.0.lines() {
            out.push_str("# ");
            out.push_str(line);
            out.push('\n');
        }
        out
    }
}

/// An empty line used to visually separate config sections.
#[derive(Clone, Debug, Default)]
pub struct GhosttyBlankLine;

impl GhosttyConfig for GhosttyBlankLine {
    fn render(&self, _ctx: &GhosttyRenderContext) -> String {
        "\n".to_owned()
    }
}

/// A raw, unvalidated `key = value` entry (or any arbitrary text).
///
/// Use this as an escape hatch for settings not yet modeled by the typed API.
///
/// ```rust
/// use toconfig::ghostty::section::RawGhostty;
///
/// let raw = RawGhostty::new("some-experimental-key = value");
/// ```
#[derive(Clone, Debug)]
pub struct RawGhostty {
    pub content: String,
}

impl RawGhostty {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
        }
    }
}

impl GhosttyConfig for RawGhostty {
    fn render(&self, _ctx: &GhosttyRenderContext) -> String {
        let mut s = self.content.clone();
        if !s.ends_with('\n') {
            s.push('\n');
        }
        s
    }
}
