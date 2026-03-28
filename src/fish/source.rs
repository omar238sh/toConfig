use crate::core::{Config, RenderContext};

// ── Source ────────────────────────────────────────────────────────────────────

/// A `source path` statement.
#[derive(Debug, Clone)]
pub struct FishSource {
    pub path: String,
}

impl FishSource {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }
}

impl Config for FishSource {
    fn render(&self, ctx: &RenderContext) -> String {
        use super::util::quote_fish_value;
        format!("{}source {}", ctx.indent(), quote_fish_value(&self.path))
    }
}

// ── Plugin (Fisher) ───────────────────────────────────────────────────────────

/// A Fisher plugin installation entry (`fisher install owner/repo`).
#[derive(Debug, Clone)]
pub struct FishPlugin {
    pub repo: String,
    pub doc: Option<String>,
}

impl FishPlugin {
    pub fn new(repo: &str) -> Self {
        Self {
            repo: repo.to_string(),
            doc: None,
        }
    }

    pub fn doc(mut self, d: &str) -> Self {
        self.doc = Some(d.to_string());
        self
    }
}

impl Config for FishPlugin {
    fn render(&self, ctx: &RenderContext) -> String {
        format!("{}fisher install {}", ctx.indent(), self.repo)
    }

    fn doc_comment(&self) -> Option<&str> {
        self.doc.as_deref()
    }
}

// ── Raw Line ──────────────────────────────────────────────────────────────────

/// An escape hatch: emit an arbitrary fish shell line verbatim.
#[derive(Debug, Clone)]
pub struct FishRawLine {
    pub code: String,
    pub doc: Option<String>,
}

impl FishRawLine {
    pub fn new(code: &str) -> Self {
        Self {
            code: code.to_string(),
            doc: None,
        }
    }

    pub fn doc(mut self, d: &str) -> Self {
        self.doc = Some(d.to_string());
        self
    }
}

impl Config for FishRawLine {
    fn render(&self, ctx: &RenderContext) -> String {
        format!("{}{}", ctx.indent(), self.code)
    }

    fn doc_comment(&self) -> Option<&str> {
        self.doc.as_deref()
    }
}
