use crate::core::{Config, RenderContext};

/// Completion provider for a user command.
#[derive(Debug, Clone)]
pub enum CmdCompletion {
    File,
    Buffer,
    Command,
    Help,
    Custom(String),
}

impl CmdCompletion {
    pub fn to_lua(&self) -> String {
        match self {
            CmdCompletion::File => "'file'".to_string(),
            CmdCompletion::Buffer => "'buffer'".to_string(),
            CmdCompletion::Command => "'command'".to_string(),
            CmdCompletion::Help => "'help'".to_string(),
            CmdCompletion::Custom(s) => format!("'{}'", s),
        }
    }
}

/// A custom Neovim user command defined via `vim.api.nvim_create_user_command`.
pub struct UserCommand {
    pub name: String,
    pub callback: String, // Lua function or string
    pub desc: Option<String>,
    pub nargs: Option<String>, // '0', '1', '*', '?', '+'
    pub range: Option<bool>,
    pub bang: Option<bool>,
    pub complete: Option<CmdCompletion>,
}

impl UserCommand {
    pub fn new(name: &str, callback: &str) -> Self {
        Self {
            name: name.to_string(),
            callback: callback.to_string(),
            desc: None,
            nargs: None,
            range: None,
            bang: None,
            complete: None,
        }
    }

    pub fn desc(mut self, d: &str) -> Self {
        self.desc = Some(d.to_string());
        self
    }
    pub fn nargs(mut self, n: &str) -> Self {
        self.nargs = Some(n.to_string());
        self
    }
    pub fn range(mut self, v: bool) -> Self {
        self.range = Some(v);
        self
    }
    pub fn bang(mut self, v: bool) -> Self {
        self.bang = Some(v);
        self
    }
    pub fn complete(mut self, c: CmdCompletion) -> Self {
        self.complete = Some(c);
        self
    }
}

impl Config for UserCommand {
    fn render(&self, ctx: &RenderContext) -> String {
        let mut opts = Vec::new();
        if let Some(ref d) = self.desc {
            opts.push(format!("desc = '{}'", d));
        }
        if let Some(ref n) = self.nargs {
            opts.push(format!("nargs = '{}'", n));
        }
        if let Some(v) = self.range {
            opts.push(format!("range = {}", v));
        }
        if let Some(v) = self.bang {
            opts.push(format!("bang = {}", v));
        }
        if let Some(ref c) = self.complete {
            opts.push(format!("complete = {}", c.to_lua()));
        }

        let opts_str = if opts.is_empty() {
            "{}".to_string()
        } else {
            format!("{{ {} }}", opts.join(", "))
        };

        format!(
            "{}vim.api.nvim_create_user_command('{}', {}, {})",
            ctx.indent(),
            self.name,
            self.callback,
            opts_str
        )
    }
    fn doc_comment(&self) -> Option<&str> {
        self.desc.as_deref()
    }
}
