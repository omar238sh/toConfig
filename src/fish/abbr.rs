use super::util::quote_fish_value;
use crate::core::{Config, RenderContext};

// ── Abbreviation ──────────────────────────────────────────────────────────────

/// Where in the command line an abbreviation expands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbbrPosition {
    /// Only when typed as a command (the default).
    Command,
    /// Anywhere in the command line.
    Anywhere,
}

/// An `abbr --add` abbreviation.
#[derive(Debug, Clone)]
pub struct FishAbbr {
    pub name: String,
    pub expansion: Option<String>,
    pub position: AbbrPosition,
    pub regex: Option<String>,
    /// Name of a fish function that returns the expansion.
    pub function: Option<String>,
    /// Place the cursor at `%` in the expansion.
    pub set_cursor: bool,
    pub doc: Option<String>,
}

impl FishAbbr {
    pub fn new(name: &str, expansion: &str) -> Self {
        Self {
            name: name.to_string(),
            expansion: Some(expansion.to_string()),
            position: AbbrPosition::Command,
            regex: None,
            function: None,
            set_cursor: false,
            doc: None,
        }
    }

    /// Abbreviation whose expansion is computed by a function.
    pub fn with_function(name: &str, function: &str) -> Self {
        Self {
            name: name.to_string(),
            expansion: None,
            position: AbbrPosition::Command,
            regex: None,
            function: Some(function.to_string()),
            set_cursor: false,
            doc: None,
        }
    }

    pub fn position(mut self, p: AbbrPosition) -> Self {
        self.position = p;
        self
    }

    /// Expand the abbreviation anywhere in the command line.
    pub fn anywhere(self) -> Self {
        self.position(AbbrPosition::Anywhere)
    }

    pub fn regex(mut self, r: &str) -> Self {
        self.regex = Some(r.to_string());
        self
    }

    /// Place the cursor at `%` in the expansion string.
    pub fn set_cursor(mut self) -> Self {
        self.set_cursor = true;
        self
    }

    pub fn doc(mut self, d: &str) -> Self {
        self.doc = Some(d.to_string());
        self
    }
}

impl Config for FishAbbr {
    fn render(&self, ctx: &RenderContext) -> String {
        let mut parts = vec!["abbr".to_string(), "--add".to_string()];

        if self.position == AbbrPosition::Anywhere {
            parts.push("--position anywhere".to_string());
        }
        if let Some(ref r) = self.regex {
            parts.push(format!("--regex {}", quote_fish_value(r)));
        }
        if let Some(ref f) = self.function {
            parts.push(format!("--function {}", f));
        }
        if self.set_cursor {
            parts.push("--set-cursor".to_string());
        }

        parts.push(quote_fish_value(&self.name));
        if let Some(ref e) = self.expansion {
            parts.push(quote_fish_value(e));
        }

        format!("{}{}", ctx.indent(), parts.join(" "))
    }

    fn doc_comment(&self) -> Option<&str> {
        self.doc.as_deref()
    }
}
