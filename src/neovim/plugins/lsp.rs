use crate::core::{Config, RenderContext};

/// A single LSP server configuration.
#[derive(Debug, Clone, Default)]
pub struct ServerConfig {
    pub name: String,
    pub cmd: Vec<String>,
    pub filetypes: Vec<String>,
    pub root_markers: Vec<String>,
    pub on_attach: Option<String>,
    pub capabilities: Option<String>,
    pub settings: Option<String>,
    pub init_options: Option<String>,
    pub single_file_support: Option<bool>,
}

impl ServerConfig {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }
    pub fn cmd(mut self, c: &[&str]) -> Self {
        self.cmd = c.iter().map(|s| s.to_string()).collect();
        self
    }
    pub fn filetypes(mut self, fts: &[&str]) -> Self {
        self.filetypes = fts.iter().map(|s| s.to_string()).collect();
        self
    }
    pub fn root_markers(mut self, markers: &[&str]) -> Self {
        self.root_markers = markers.iter().map(|s| s.to_string()).collect();
        self
    }
    pub fn on_attach(mut self, lua: &str) -> Self {
        self.on_attach = Some(lua.to_string());
        self
    }
    pub fn capabilities(mut self, lua: &str) -> Self {
        self.capabilities = Some(lua.to_string());
        self
    }
    pub fn settings(mut self, lua: &str) -> Self {
        self.settings = Some(lua.to_string());
        self
    }
    pub fn init_options(mut self, lua: &str) -> Self {
        self.init_options = Some(lua.to_string());
        self
    }
    pub fn single_file_support(mut self, v: bool) -> Self {
        self.single_file_support = Some(v);
        self
    }
}

/// Specialized LSP trait — any struct implementing this is a valid LSP config.
pub trait LspConfig {
    fn servers(&self) -> &[ServerConfig];
    fn on_attach(&self) -> Option<&str> {
        None
    }
    fn capabilities(&self) -> Option<&str> {
        None
    }
}

/// The main lspconfig setup node using the specialized trait.
pub struct LspConfigNode {
    pub servers: Vec<ServerConfig>,
    pub global_on_attach: Option<String>,
    pub global_capabilities: Option<String>,
}

impl LspConfigNode {
    pub fn new() -> Self {
        Self {
            servers: Vec::new(),
            global_on_attach: None,
            global_capabilities: None,
        }
    }
    pub fn server(mut self, s: ServerConfig) -> Self {
        self.servers.push(s);
        self
    }
    pub fn on_attach(mut self, lua: &str) -> Self {
        self.global_on_attach = Some(lua.to_string());
        self
    }
    pub fn capabilities(mut self, lua: &str) -> Self {
        self.global_capabilities = Some(lua.to_string());
        self
    }
}

impl LspConfig for LspConfigNode {
    fn servers(&self) -> &[ServerConfig] {
        &self.servers
    }
    fn on_attach(&self) -> Option<&str> {
        self.global_on_attach.as_deref()
    }
    fn capabilities(&self) -> Option<&str> {
        self.global_capabilities.as_deref()
    }
}

impl Config for LspConfigNode {
    fn render(&self, ctx: &RenderContext) -> String {
        let i = ctx.indent();
        let mut out = vec![format!("{}local lspconfig = require('lspconfig')", i)];

        if let Some(ref c) = self.global_capabilities {
            out.push(format!("{}local capabilities = {}", i, c));
        }
        if let Some(ref oa) = self.global_on_attach {
            out.push(format!("{}local on_attach = {}", i, oa));
        }

        for srv in &self.servers {
            let mut opts = Vec::new();
            if !srv.cmd.is_empty() {
                let c = srv
                    .cmd
                    .iter()
                    .map(|s| format!("'{}'", s))
                    .collect::<Vec<_>>()
                    .join(", ");
                opts.push(format!("cmd = {{ {} }}", c));
            }
            if !srv.filetypes.is_empty() {
                let f = srv
                    .filetypes
                    .iter()
                    .map(|s| format!("'{}'", s))
                    .collect::<Vec<_>>()
                    .join(", ");
                opts.push(format!("filetypes = {{ {} }}", f));
            }
            if !srv.root_markers.is_empty() {
                let r = srv
                    .root_markers
                    .iter()
                    .map(|s| format!("'{}'", s))
                    .collect::<Vec<_>>()
                    .join(", ");
                opts.push(format!("root_dir = lspconfig.util.root_pattern({} )", r));
            }
            // Per-server overrides; fall back to global
            let oa = srv
                .on_attach
                .as_deref()
                .or(self.global_on_attach.as_deref().map(|_| "on_attach"));
            if let Some(oa) = oa {
                opts.push(format!("on_attach = {}", oa));
            }
            let cap = srv
                .capabilities
                .as_deref()
                .or(self.global_capabilities.as_deref().map(|_| "capabilities"));
            if let Some(cap) = cap {
                opts.push(format!("capabilities = {}", cap));
            }
            if let Some(ref st) = srv.settings {
                opts.push(format!("settings = {}", st));
            }
            if let Some(ref io) = srv.init_options {
                opts.push(format!("init_options = {}", io));
            }
            if let Some(v) = srv.single_file_support {
                opts.push(format!("single_file_support = {}", v));
            }

            let opts_str = if opts.is_empty() {
                "{}".to_string()
            } else {
                format!("{{\n  {}\n}}", opts.join(",\n  "))
            };
            out.push(format!("{}lspconfig.{}.setup({})", i, srv.name, opts_str));
        }
        out.join("\n")
    }
}
