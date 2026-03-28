use crate::core::{Config, RenderContext};

// ── Key Binding ───────────────────────────────────────────────────────────────

/// The bind mode for a fish key binding.
#[derive(Debug, Clone)]
pub enum BindMode {
    Default,
    Insert,
    Visual,
    Custom(String),
}

impl BindMode {
    pub fn as_str(&self) -> &str {
        match self {
            BindMode::Default => "default",
            BindMode::Insert => "insert",
            BindMode::Visual => "visual",
            BindMode::Custom(s) => s.as_str(),
        }
    }
}

/// A `bind` key-binding statement.
#[derive(Debug, Clone)]
pub struct FishBind {
    pub key: String,
    pub command: String,
    pub mode: Option<BindMode>,
    /// Transition to a different mode after the binding fires.
    pub sets_mode: Option<String>,
    pub silent: bool,
    pub doc: Option<String>,
}

impl FishBind {
    pub fn new(key: &str, command: &str) -> Self {
        Self {
            key: key.to_string(),
            command: command.to_string(),
            mode: None,
            sets_mode: None,
            silent: false,
            doc: None,
        }
    }

    pub fn mode(mut self, m: BindMode) -> Self {
        self.mode = Some(m);
        self
    }

    pub fn sets_mode(mut self, m: &str) -> Self {
        self.sets_mode = Some(m.to_string());
        self
    }

    pub fn silent(mut self) -> Self {
        self.silent = true;
        self
    }

    pub fn doc(mut self, d: &str) -> Self {
        self.doc = Some(d.to_string());
        self
    }
}

impl Config for FishBind {
    fn render(&self, ctx: &RenderContext) -> String {
        let mut parts = vec!["bind".to_string()];
        if let Some(ref m) = self.mode {
            parts.push(format!("--mode {}", m.as_str()));
        }
        if let Some(ref m) = self.sets_mode {
            parts.push(format!("--sets-mode {}", m));
        }
        if self.silent {
            parts.push("--silent".to_string());
        }
        parts.push(self.key.clone());
        parts.push(self.command.clone());
        format!("{}{}", ctx.indent(), parts.join(" "))
    }

    fn doc_comment(&self) -> Option<&str> {
        self.doc.as_deref()
    }
}
