use crate::core::{Config, RenderContext};

// ── Mode Enum ────────────────────────────────────────────────────────────────

/// Represents a Neovim mode for key mappings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Insert,
    Visual,
    VisualLine,
    VisualBlock,
    Select,
    CommandLine,
    Terminal,
    Operator,
}

impl Mode {
    pub fn as_str(self) -> &'static str {
        match self {
            Mode::Normal => "n",
            Mode::Insert => "i",
            Mode::Visual => "v",
            Mode::VisualLine => "V",
            Mode::VisualBlock => "\x16", // Ctrl-V
            Mode::Select => "s",
            Mode::CommandLine => "c",
            Mode::Terminal => "t",
            Mode::Operator => "o",
        }
    }
}

// ── Options ──────────────────────────────────────────────────────────────────

/// All supported options for `vim.keymap.set`.
#[derive(Debug, Clone, Default)]
pub struct MapOpts {
    pub silent: Option<bool>,
    pub noremap: Option<bool>,
    pub expr: Option<bool>,
    pub nowait: Option<bool>,
    pub remap: Option<bool>,
    pub unique: Option<bool>,
    pub desc: Option<String>,
    pub buffer: Option<i32>,
}

impl MapOpts {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn silent(mut self, v: bool) -> Self {
        self.silent = Some(v);
        self
    }
    pub fn noremap(mut self, v: bool) -> Self {
        self.noremap = Some(v);
        self
    }
    pub fn expr(mut self, v: bool) -> Self {
        self.expr = Some(v);
        self
    }
    pub fn nowait(mut self, v: bool) -> Self {
        self.nowait = Some(v);
        self
    }
    pub fn remap(mut self, v: bool) -> Self {
        self.remap = Some(v);
        self
    }
    pub fn unique(mut self, v: bool) -> Self {
        self.unique = Some(v);
        self
    }
    pub fn desc(mut self, d: &str) -> Self {
        self.desc = Some(d.to_string());
        self
    }
    pub fn buffer(mut self, buf: i32) -> Self {
        self.buffer = Some(buf);
        self
    }

    /// Commonly-used combination: silent + noremap.
    pub fn default_safe() -> Self {
        Self::new().silent(true).noremap(true)
    }

    pub fn to_lua_table(&self) -> String {
        let mut props = Vec::new();
        if let Some(v) = self.silent {
            props.push(format!("silent = {}", v));
        }
        if let Some(v) = self.noremap {
            props.push(format!("noremap = {}", v));
        }
        if let Some(v) = self.expr {
            props.push(format!("expr = {}", v));
        }
        if let Some(v) = self.nowait {
            props.push(format!("nowait = {}", v));
        }
        if let Some(v) = self.remap {
            props.push(format!("remap = {}", v));
        }
        if let Some(v) = self.unique {
            props.push(format!("unique = {}", v));
        }
        if let Some(ref d) = self.desc {
            props.push(format!("desc = '{}'", d));
        }
        if let Some(v) = self.buffer {
            props.push(format!("buffer = {}", v));
        }
        if props.is_empty() {
            "{}".to_string()
        } else {
            format!("{{ {} }}", props.join(", "))
        }
    }
}

// ── RHS Enum ─────────────────────────────────────────────────────────────────

/// The right-hand side of a mapping: a command string or a Lua callback.
#[derive(Debug, Clone)]
pub enum MapRhs {
    Command(String),
    Lua(String),
}

impl MapRhs {
    pub fn cmd(s: &str) -> Self {
        MapRhs::Command(s.to_string())
    }
    pub fn lua(s: &str) -> Self {
        MapRhs::Lua(s.to_string())
    }
    pub fn to_string(&self) -> String {
        match self {
            MapRhs::Command(s) => format!("'{}'", s),
            MapRhs::Lua(s) => s.clone(),
        }
    }
}

// ── Keymap Node ───────────────────────────────────────────────────────────────

/// A single keymap definition.
#[derive(Debug, Clone)]
pub struct KeymapNode {
    pub modes: Vec<Mode>,
    pub lhs: String,
    pub rhs: MapRhs,
    pub opts: MapOpts,
}

impl KeymapNode {
    pub fn new(modes: Vec<Mode>, lhs: &str, rhs: MapRhs) -> Self {
        Self {
            modes,
            lhs: lhs.to_string(),
            rhs,
            opts: MapOpts::default_safe(),
        }
    }

    // ── Mode shortcuts ───────────────────────────────────────────────────────
    pub fn n(lhs: &str, rhs: MapRhs) -> Self {
        Self::new(vec![Mode::Normal], lhs, rhs)
    }
    pub fn i(lhs: &str, rhs: MapRhs) -> Self {
        Self::new(vec![Mode::Insert], lhs, rhs)
    }
    pub fn v(lhs: &str, rhs: MapRhs) -> Self {
        Self::new(vec![Mode::Visual], lhs, rhs)
    }
    pub fn t(lhs: &str, rhs: MapRhs) -> Self {
        Self::new(vec![Mode::Terminal], lhs, rhs)
    }
    pub fn nv(lhs: &str, rhs: MapRhs) -> Self {
        Self::new(vec![Mode::Normal, Mode::Visual], lhs, rhs)
    }

    // ── Fluent opts ──────────────────────────────────────────────────────────
    pub fn opts(mut self, opts: MapOpts) -> Self {
        self.opts = opts;
        self
    }
    pub fn silent(mut self, v: bool) -> Self {
        self.opts.silent = Some(v);
        self
    }
    pub fn noremap(mut self, v: bool) -> Self {
        self.opts.noremap = Some(v);
        self
    }
    pub fn desc(mut self, d: &str) -> Self {
        self.opts.desc = Some(d.to_string());
        self
    }
    pub fn buffer(mut self, buf: i32) -> Self {
        self.opts.buffer = Some(buf);
        self
    }
    pub fn expr(mut self, v: bool) -> Self {
        self.opts.expr = Some(v);
        self
    }
}

impl Config for KeymapNode {
    fn render(&self, ctx: &RenderContext) -> String {
        let modes_str = if self.modes.len() == 1 {
            format!("'{}'", self.modes[0].as_str())
        } else {
            let m = self
                .modes
                .iter()
                .map(|m| format!("'{}'", m.as_str()))
                .collect::<Vec<_>>()
                .join(", ");
            format!("{{ {} }}", m)
        };
        format!(
            "{}vim.keymap.set({}, '{}', {}, {})",
            ctx.indent(),
            modes_str,
            self.lhs,
            self.rhs.to_string(),
            self.opts.to_lua_table()
        )
    }
    fn doc_comment(&self) -> Option<&str> {
        self.opts.desc.as_deref()
    }
}

// ── KeymapGroup ───────────────────────────────────────────────────────────────

/// A logical group of keymaps (e.g., for which-key.nvim).
/// All child mappings inherit the group prefix.
pub struct KeymapGroup {
    pub prefix: String,
    pub label: Option<String>,
    pub maps: Vec<KeymapNode>,
}

impl KeymapGroup {
    pub fn new(prefix: &str) -> Self {
        Self {
            prefix: prefix.to_string(),
            label: None,
            maps: Vec::new(),
        }
    }

    pub fn label(mut self, l: &str) -> Self {
        self.label = Some(l.to_string());
        self
    }

    pub fn add(mut self, mut map: KeymapNode) -> Self {
        map.lhs = format!("{}{}", self.prefix, map.lhs);
        self.maps.push(map);
        self
    }
}

impl Config for KeymapGroup {
    fn render(&self, ctx: &RenderContext) -> String {
        let mut out = Vec::new();
        if let Some(ref label) = self.label {
            out.push(format!("{}-- Keymaps: {}", ctx.indent(), label));
        }
        for m in &self.maps {
            out.push(m.render(ctx));
        }
        out.join("\n")
    }
}
