use super::util::quote_fish_value;
use crate::core::{Config, RenderContext};

// ── Greeting ──────────────────────────────────────────────────────────────────

/// Sets the fish greeting shown at session start.
#[derive(Debug, Clone)]
pub enum FishGreeting {
    /// `set -g fish_greeting "message"` – a plain string.
    Message(String),
    /// Custom `fish_greeting` function with body lines.
    Function(Vec<String>),
    /// `set -g fish_greeting` (empty) – disables the greeting.
    Disabled,
}

impl FishGreeting {
    pub fn message(msg: &str) -> Self {
        FishGreeting::Message(msg.to_string())
    }

    pub fn function(body: &[&str]) -> Self {
        FishGreeting::Function(body.iter().map(|s| s.to_string()).collect())
    }

    pub fn disabled() -> Self {
        FishGreeting::Disabled
    }
}

impl Config for FishGreeting {
    fn render(&self, ctx: &RenderContext) -> String {
        match self {
            FishGreeting::Disabled => format!("{}set -g fish_greeting", ctx.indent()),
            FishGreeting::Message(msg) => {
                format!(
                    "{}set -g fish_greeting {}",
                    ctx.indent(),
                    quote_fish_value(msg)
                )
            }
            FishGreeting::Function(body) => {
                let inner = ctx.deeper();
                let mut out = Vec::new();
                out.push(format!("{}function fish_greeting", ctx.indent()));
                for line in body {
                    out.push(format!("{}{}", inner.indent(), line));
                }
                out.push(format!("{}end", ctx.indent()));
                out.join("\n")
            }
        }
    }
}

// ── Prompt ────────────────────────────────────────────────────────────────────

/// The `fish_prompt` function (left prompt).
#[derive(Debug, Clone)]
pub struct FishPrompt {
    pub body: Vec<String>,
    pub doc: Option<String>,
}

impl FishPrompt {
    pub fn new(body: &[&str]) -> Self {
        Self {
            body: body.iter().map(|s| s.to_string()).collect(),
            doc: None,
        }
    }

    pub fn doc(mut self, d: &str) -> Self {
        self.doc = Some(d.to_string());
        self
    }
}

impl Config for FishPrompt {
    fn render(&self, ctx: &RenderContext) -> String {
        let indent = ctx.indent();
        let inner = ctx.deeper();
        let mut out = Vec::new();
        out.push(format!("{}function fish_prompt", indent));
        for line in &self.body {
            out.push(format!("{}{}", inner.indent(), line));
        }
        out.push(format!("{}end", indent));
        out.join("\n")
    }

    fn doc_comment(&self) -> Option<&str> {
        self.doc.as_deref()
    }
}

/// The `fish_right_prompt` function (right side of prompt line).
#[derive(Debug, Clone)]
pub struct FishRightPrompt {
    pub body: Vec<String>,
    pub doc: Option<String>,
}

impl FishRightPrompt {
    pub fn new(body: &[&str]) -> Self {
        Self {
            body: body.iter().map(|s| s.to_string()).collect(),
            doc: None,
        }
    }

    pub fn doc(mut self, d: &str) -> Self {
        self.doc = Some(d.to_string());
        self
    }
}

impl Config for FishRightPrompt {
    fn render(&self, ctx: &RenderContext) -> String {
        let indent = ctx.indent();
        let inner = ctx.deeper();
        let mut out = Vec::new();
        out.push(format!("{}function fish_right_prompt", indent));
        for line in &self.body {
            out.push(format!("{}{}", inner.indent(), line));
        }
        out.push(format!("{}end", indent));
        out.join("\n")
    }

    fn doc_comment(&self) -> Option<&str> {
        self.doc.as_deref()
    }
}

/// The `fish_mode_prompt` function (shows vi-mode indicator).
#[derive(Debug, Clone)]
pub struct FishModePrompt {
    pub body: Vec<String>,
}

impl FishModePrompt {
    pub fn new(body: &[&str]) -> Self {
        Self {
            body: body.iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl Config for FishModePrompt {
    fn render(&self, ctx: &RenderContext) -> String {
        let indent = ctx.indent();
        let inner = ctx.deeper();
        let mut out = Vec::new();
        out.push(format!("{}function fish_mode_prompt", indent));
        for line in &self.body {
            out.push(format!("{}{}", inner.indent(), line));
        }
        out.push(format!("{}end", indent));
        out.join("\n")
    }
}
