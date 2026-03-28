use crate::core::{Config, RenderContext};

/// A single completion source for nvim-cmp.
#[derive(Debug, Clone)]
pub struct CmpSource {
    pub name: String,
    pub priority: Option<u32>,
    pub keyword_length: Option<u32>,
    pub group_index: Option<u32>,
}

impl CmpSource {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            priority: None,
            keyword_length: None,
            group_index: None,
        }
    }
    pub fn priority(mut self, n: u32) -> Self {
        self.priority = Some(n);
        self
    }
    pub fn keyword_length(mut self, n: u32) -> Self {
        self.keyword_length = Some(n);
        self
    }
    pub fn group_index(mut self, n: u32) -> Self {
        self.group_index = Some(n);
        self
    }

    pub fn to_lua(&self) -> String {
        let mut props = vec![format!("name = '{}'", self.name)];
        if let Some(v) = self.priority {
            props.push(format!("priority = {}", v));
        }
        if let Some(v) = self.keyword_length {
            props.push(format!("keyword_length = {}", v));
        }
        if let Some(v) = self.group_index {
            props.push(format!("group_index = {}", v));
        }
        format!("{{ {} }}", props.join(", "))
    }
}

/// Full nvim-cmp setup configuration.
pub struct CmpConfig {
    pub sources: Vec<CmpSource>,
    pub snippet_engine: Option<String>,
    pub mappings: Option<String>,
    pub formatting: Option<String>,
    pub experimental: Option<String>,
    pub completion: Option<String>,
}

impl CmpConfig {
    pub fn new() -> Self {
        Self {
            sources: Vec::new(),
            snippet_engine: None,
            mappings: None,
            formatting: None,
            experimental: None,
            completion: None,
        }
    }
    pub fn source(mut self, s: CmpSource) -> Self {
        self.sources.push(s);
        self
    }
    pub fn snippet_engine(mut self, lua: &str) -> Self {
        self.snippet_engine = Some(lua.to_string());
        self
    }
    pub fn mappings(mut self, lua: &str) -> Self {
        self.mappings = Some(lua.to_string());
        self
    }
    pub fn formatting(mut self, lua: &str) -> Self {
        self.formatting = Some(lua.to_string());
        self
    }
    pub fn experimental(mut self, lua: &str) -> Self {
        self.experimental = Some(lua.to_string());
        self
    }
    pub fn completion(mut self, lua: &str) -> Self {
        self.completion = Some(lua.to_string());
        self
    }
}

impl Config for CmpConfig {
    fn render(&self, ctx: &RenderContext) -> String {
        let i = ctx.indent();
        let cmp_var = format!("{}local cmp = require('cmp')", i);
        let mut opts = Vec::new();
        if let Some(ref s) = self.snippet_engine {
            opts.push(format!("  snippet = {}", s));
        }
        if let Some(ref m) = self.mappings {
            opts.push(format!("  mapping = {}", m));
        }
        if let Some(ref f) = self.formatting {
            opts.push(format!("  formatting = {}", f));
        }
        if let Some(ref e) = self.experimental {
            opts.push(format!("  experimental = {}", e));
        }
        if let Some(ref c) = self.completion {
            opts.push(format!("  completion = {}", c));
        }
        if !self.sources.is_empty() {
            let srcs = self
                .sources
                .iter()
                .map(|s| format!("    {}", s.to_lua()))
                .collect::<Vec<_>>()
                .join(",\n");
            opts.push(format!(
                "  sources = cmp.config.sources({{\n{}\n  }})",
                srcs
            ));
        }
        format!(
            "{}\n{}cmp.setup({{\n{}\n{}}})",
            cmp_var,
            i,
            opts.join(",\n"),
            i
        )
    }
}
