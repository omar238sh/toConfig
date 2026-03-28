use crate::core::{Config, RenderContext};

// ── Pattern ───────────────────────────────────────────────────────────────────

/// Autocmd pattern: glob string or buffer-local.
#[derive(Debug, Clone)]
pub enum AutocmdPattern {
    Glob(Vec<String>),
    Buffer(Option<i32>), // None = current buffer (0)
}

impl AutocmdPattern {
    pub fn glob(patterns: &[&str]) -> Self {
        AutocmdPattern::Glob(patterns.iter().map(|s| s.to_string()).collect())
    }
    pub fn file(ext: &str) -> Self {
        AutocmdPattern::Glob(vec![format!("*.{}", ext)])
    }
    pub fn buffer() -> Self {
        AutocmdPattern::Buffer(None)
    }
    pub fn buffer_id(n: i32) -> Self {
        AutocmdPattern::Buffer(Some(n))
    }

    pub fn to_lua(&self) -> Option<String> {
        match self {
            AutocmdPattern::Glob(pats) => {
                if pats.len() == 1 {
                    Some(format!("pattern = '{}'", pats[0]))
                } else {
                    let s = pats
                        .iter()
                        .map(|p| format!("'{}'", p))
                        .collect::<Vec<_>>()
                        .join(", ");
                    Some(format!("pattern = {{ {} }}", s))
                }
            }
            AutocmdPattern::Buffer(Some(n)) => Some(format!("buffer = {}", n)),
            AutocmdPattern::Buffer(None) => Some("buffer = 0".to_string()),
        }
    }
}

// ── Action ────────────────────────────────────────────────────────────────────

/// The action to take when the autocmd fires.
#[derive(Debug, Clone)]
pub enum AutocmdAction {
    Command(String),
    Callback(String), // inline Lua function body or reference
}

impl AutocmdAction {
    pub fn cmd(s: &str) -> Self {
        AutocmdAction::Command(s.to_string())
    }
    pub fn callback(lua: &str) -> Self {
        AutocmdAction::Callback(lua.to_string())
    }
}

// ── Autocmd Node ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct AutocmdNode {
    pub events: Vec<String>,
    pub pattern: Option<AutocmdPattern>,
    pub action: AutocmdAction,
    pub group: Option<String>,
    pub desc: Option<String>,
    pub once: Option<bool>,
    pub nested: Option<bool>,
}

impl AutocmdNode {
    pub fn new(events: &[&str], action: AutocmdAction) -> Self {
        Self {
            events: events.iter().map(|s| s.to_string()).collect(),
            pattern: None,
            action,
            group: None,
            desc: None,
            once: None,
            nested: None,
        }
    }

    pub fn on_buf_write(action: AutocmdAction) -> Self {
        Self::new(&["BufWritePost"], action)
    }
    pub fn on_buf_enter(action: AutocmdAction) -> Self {
        Self::new(&["BufEnter"], action)
    }
    pub fn on_vim_enter(action: AutocmdAction) -> Self {
        Self::new(&["VimEnter"], action)
    }
    pub fn on_file_type(ft: &str, action: AutocmdAction) -> Self {
        let mut node = Self::new(&["FileType"], action);
        node.pattern = Some(AutocmdPattern::glob(&[ft]));
        node
    }

    // ── Fluent setters ───────────────────────────────────────────────────────
    pub fn pattern(mut self, p: AutocmdPattern) -> Self {
        self.pattern = Some(p);
        self
    }
    pub fn group(mut self, g: &str) -> Self {
        self.group = Some(g.to_string());
        self
    }
    pub fn desc(mut self, d: &str) -> Self {
        self.desc = Some(d.to_string());
        self
    }
    pub fn once(mut self, v: bool) -> Self {
        self.once = Some(v);
        self
    }
    pub fn nested(mut self, v: bool) -> Self {
        self.nested = Some(v);
        self
    }
}

impl Config for AutocmdNode {
    fn render(&self, ctx: &RenderContext) -> String {
        let events_str = if self.events.len() == 1 {
            format!("'{}'", self.events[0])
        } else {
            let e = self
                .events
                .iter()
                .map(|e| format!("'{}'", e))
                .collect::<Vec<_>>()
                .join(", ");
            format!("{{ {} }}", e)
        };

        let mut opts = Vec::new();
        if let Some(ref g) = self.group {
            opts.push(format!("group = '{}'", g));
        }
        if let Some(ref p) = self.pattern {
            if let Some(s) = p.to_lua() {
                opts.push(s);
            }
        }
        if let Some(v) = self.once {
            opts.push(format!("once = {}", v));
        }
        if let Some(v) = self.nested {
            opts.push(format!("nested = {}", v));
        }
        if let Some(ref d) = self.desc {
            opts.push(format!("desc = '{}'", d));
        }

        match &self.action {
            AutocmdAction::Command(s) => opts.push(format!("command = '{}'", s)),
            AutocmdAction::Callback(s) => opts.push(format!("callback = {}", s)),
        }

        format!(
            "{}vim.api.nvim_create_autocmd({}, {{ {} }})",
            ctx.indent(),
            events_str,
            opts.join(", ")
        )
    }
    fn doc_comment(&self) -> Option<&str> {
        self.desc.as_deref()
    }
}

// ── Augroup ───────────────────────────────────────────────────────────────────

/// Wraps a set of autocmds in an augroup, clearing it on reload to prevent duplication.
pub struct Augroup {
    pub name: String,
    pub clear: bool,
    pub autocmds: Vec<AutocmdNode>,
}

impl Augroup {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            clear: true,
            autocmds: Vec::new(),
        }
    }

    pub fn no_clear(mut self) -> Self {
        self.clear = false;
        self
    }

    pub fn add(mut self, mut autocmd: AutocmdNode) -> Self {
        autocmd.group = Some(self.name.clone());
        self.autocmds.push(autocmd);
        self
    }
}

impl Config for Augroup {
    fn render(&self, ctx: &RenderContext) -> String {
        let mut out = Vec::new();
        out.push(format!(
            "{}local {} = vim.api.nvim_create_augroup('{}', {{ clear = {} }})",
            ctx.indent(),
            self.name,
            self.name,
            self.clear
        ));
        for ac in &self.autocmds {
            out.push(ac.render(ctx));
        }
        out.join("\n")
    }
}
