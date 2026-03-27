use crate::core::{Config, RenderContext};
use crate::lua::LuaValue;

/// Neovim option scope
#[derive(Debug, Clone, Copy)]
pub enum OptionScope {
    /// vim.opt  – global options (with inheritance)
    Opt,
    /// vim.o    – raw global options
    O,
    /// vim.g    – global variables
    G,
    /// vim.b    – buffer-local options
    Bo,
    /// vim.w    – window-local options
    Wo,
}

impl OptionScope {
    pub fn as_str(self) -> &'static str {
        match self {
            OptionScope::Opt => "opt",
            OptionScope::O => "o",
            OptionScope::G => "g",
            OptionScope::Bo => "b",
            OptionScope::Wo => "w",
        }
    }
}

/// A single Neovim option assignment.
#[derive(Debug, Clone)]
pub struct OptionNode {
    pub scope: OptionScope,
    pub name: String,
    pub value: LuaValue,
    pub doc: Option<String>,
}

impl OptionNode {
    pub fn new(scope: OptionScope, name: &str, value: LuaValue) -> Self {
        Self {
            scope,
            name: name.to_string(),
            value,
            doc: None,
        }
    }

    // ── Scope constructors ──────────────────────────────────────────────────

    pub fn opt(name: &str, value: LuaValue) -> Self {
        Self::new(OptionScope::Opt, name, value)
    }
    pub fn g(name: &str, value: LuaValue) -> Self {
        Self::new(OptionScope::G, name, value)
    }
    pub fn o(name: &str, value: LuaValue) -> Self {
        Self::new(OptionScope::O, name, value)
    }
    pub fn bo(name: &str, value: LuaValue) -> Self {
        Self::new(OptionScope::Bo, name, value)
    }
    pub fn wo(name: &str, value: LuaValue) -> Self {
        Self::new(OptionScope::Wo, name, value)
    }

    pub fn doc(mut self, doc: &str) -> Self {
        self.doc = Some(doc.to_string());
        self
    }
}

impl Config for OptionNode {
    fn render(&self, ctx: &RenderContext) -> String {
        format!(
            "{}vim.{}.{} = {}",
            ctx.indent(),
            self.scope.as_str(),
            self.name,
            self.value.to_lua(ctx.indent_level)
        )
    }
    fn doc_comment(&self) -> Option<&str> {
        self.doc.as_deref()
    }
}

// ── Batch option blocks ──────────────────────────────────────────────────────

/// A logical group of option nodes rendered together, optionally under a comment.
pub struct OptionsBlock {
    pub comment: Option<String>,
    pub nodes: Vec<OptionNode>,
}

impl OptionsBlock {
    pub fn new() -> Self {
        Self {
            comment: None,
            nodes: Vec::new(),
        }
    }

    pub fn with_comment(mut self, c: &str) -> Self {
        self.comment = Some(c.to_string());
        self
    }

    pub fn add(mut self, node: OptionNode) -> Self {
        self.nodes.push(node);
        self
    }
}

impl Config for OptionsBlock {
    fn render(&self, ctx: &RenderContext) -> String {
        let mut out = Vec::new();
        if let Some(ref c) = self.comment {
            out.push(format!("{}-- {}", ctx.indent(), c));
        }
        for n in &self.nodes {
            out.push(n.render(ctx));
        }
        out.join("\n")
    }
}

// ── Common preset builders ───────────────────────────────────────────────────

/// Pre-built sensible defaults for common editor options.
pub fn default_editor_options() -> OptionsBlock {
    OptionsBlock::new()
        .with_comment("Editor defaults")
        .add(OptionNode::opt("number", LuaValue::bool(true)))
        .add(OptionNode::opt("relativenumber", LuaValue::bool(true)))
        .add(OptionNode::opt("tabstop", LuaValue::int(4)))
        .add(OptionNode::opt("shiftwidth", LuaValue::int(4)))
        .add(OptionNode::opt("expandtab", LuaValue::bool(true)))
        .add(OptionNode::opt("smartindent", LuaValue::bool(true)))
        .add(OptionNode::opt("wrap", LuaValue::bool(false)))
        .add(OptionNode::opt("cursorline", LuaValue::bool(true)))
        .add(OptionNode::opt("signcolumn", LuaValue::str("yes")))
        .add(OptionNode::opt("termguicolors", LuaValue::bool(true)))
        .add(OptionNode::opt("clipboard", LuaValue::str("unnamedplus")))
        .add(OptionNode::opt("scrolloff", LuaValue::int(8)))
        .add(OptionNode::opt("sidescrolloff", LuaValue::int(8)))
        .add(OptionNode::opt("updatetime", LuaValue::int(300)))
        .add(OptionNode::opt("timeoutlen", LuaValue::int(500)))
}
