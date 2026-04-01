use super::core::{GhosttyConfig, GhosttyRenderContext};

/// Clipboard read/write access policy.
#[derive(Clone, Debug)]
pub enum ClipboardAccess {
    /// Allow clipboard access (default for write, ask for read).
    Allow,
    /// Deny clipboard access.
    Deny,
    /// Prompt the user before granting access.
    Ask,
}

impl std::fmt::Display for ClipboardAccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Allow => "allow",
            Self::Deny => "deny",
            Self::Ask => "ask",
        };
        write!(f, "{}", s)
    }
}

/// Terminal behavior configuration for Ghostty.
///
/// Renders as `term`, `scrollback-limit`, `clipboard-*`, and related entries.
#[derive(Default, Clone, Debug)]
pub struct TerminalConfig {
    /// Value of `$TERM`, e.g. `"xterm-ghostty"` (default).
    pub term: Option<String>,
    /// Number of lines kept in the scroll-back buffer (0 = unlimited).
    pub scrollback_limit: Option<i64>,
    /// Whether OSC 52 clipboard read is permitted.
    pub clipboard_read: Option<ClipboardAccess>,
    /// Whether OSC 52 clipboard write is permitted.
    pub clipboard_write: Option<ClipboardAccess>,
    /// Strip trailing whitespace when copying.
    pub clipboard_trim_trailing_spaces: Option<bool>,
    /// Warn before pasting text that looks dangerous.
    pub clipboard_paste_protection: Option<bool>,
    /// Allow bracketed-paste bypass of paste protection.
    pub clipboard_paste_bracketed_safe: Option<bool>,
    /// Enable the OSC 8 hyperlink protocol.
    pub links: Option<bool>,
    /// Open URLs in the browser on click.
    pub link_url: Option<bool>,
}

impl TerminalConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn term(mut self, v: impl Into<String>) -> Self {
        self.term = Some(v.into());
        self
    }

    pub fn scrollback_limit(mut self, lines: i64) -> Self {
        self.scrollback_limit = Some(lines);
        self
    }

    pub fn clipboard_read(mut self, a: ClipboardAccess) -> Self {
        self.clipboard_read = Some(a);
        self
    }

    pub fn clipboard_write(mut self, a: ClipboardAccess) -> Self {
        self.clipboard_write = Some(a);
        self
    }

    pub fn clipboard_trim_trailing_spaces(mut self, v: bool) -> Self {
        self.clipboard_trim_trailing_spaces = Some(v);
        self
    }

    pub fn clipboard_paste_protection(mut self, v: bool) -> Self {
        self.clipboard_paste_protection = Some(v);
        self
    }

    pub fn clipboard_paste_bracketed_safe(mut self, v: bool) -> Self {
        self.clipboard_paste_bracketed_safe = Some(v);
        self
    }

    pub fn links(mut self, v: bool) -> Self {
        self.links = Some(v);
        self
    }

    pub fn link_url(mut self, v: bool) -> Self {
        self.link_url = Some(v);
        self
    }
}

impl GhosttyConfig for TerminalConfig {
    fn render(&self, _ctx: &GhosttyRenderContext) -> String {
        let mut out = String::new();

        if let Some(ref v) = self.term {
            out.push_str(&format!("term = {}\n", v));
        }
        if let Some(v) = self.scrollback_limit {
            out.push_str(&format!("scrollback-limit = {}\n", v));
        }
        if let Some(ref v) = self.clipboard_read {
            out.push_str(&format!("clipboard-read = {}\n", v));
        }
        if let Some(ref v) = self.clipboard_write {
            out.push_str(&format!("clipboard-write = {}\n", v));
        }
        if let Some(v) = self.clipboard_trim_trailing_spaces {
            out.push_str(&format!("clipboard-trim-trailing-spaces = {}\n", v));
        }
        if let Some(v) = self.clipboard_paste_protection {
            out.push_str(&format!("clipboard-paste-protection = {}\n", v));
        }
        if let Some(v) = self.clipboard_paste_bracketed_safe {
            out.push_str(&format!("clipboard-paste-bracketed-safe = {}\n", v));
        }
        if let Some(v) = self.links {
            out.push_str(&format!("link = {}\n", v));
        }
        if let Some(v) = self.link_url {
            out.push_str(&format!("link-url = {}\n", v));
        }

        out
    }
}
