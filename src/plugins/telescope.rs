use crate::core::{Config, RenderContext};

#[derive(Debug, Clone, Default)]
pub struct TelescopeDefaults {
    pub prompt_prefix: Option<String>,
    pub selection_caret: Option<String>,
    pub path_display: Option<Vec<String>>,
    pub border: Option<bool>,
    pub winblend: Option<i32>,
    pub layout_strategy: Option<String>,
    pub sorting_strategy: Option<String>,
    pub mappings: Option<String>,
}

impl TelescopeDefaults {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn prompt_prefix(mut self, s: &str) -> Self {
        self.prompt_prefix = Some(s.to_string());
        self
    }
    pub fn selection_caret(mut self, s: &str) -> Self {
        self.selection_caret = Some(s.to_string());
        self
    }
    pub fn path_display(mut self, v: &[&str]) -> Self {
        self.path_display = Some(v.iter().map(|s| s.to_string()).collect());
        self
    }
    pub fn border(mut self, v: bool) -> Self {
        self.border = Some(v);
        self
    }
    pub fn winblend(mut self, n: i32) -> Self {
        self.winblend = Some(n);
        self
    }
    pub fn layout_strategy(mut self, s: &str) -> Self {
        self.layout_strategy = Some(s.to_string());
        self
    }
    pub fn sorting_strategy(mut self, s: &str) -> Self {
        self.sorting_strategy = Some(s.to_string());
        self
    }
    pub fn mappings(mut self, lua: &str) -> Self {
        self.mappings = Some(lua.to_string());
        self
    }

    fn to_lua(&self, indent: usize) -> String {
        let pad = "  ".repeat(indent);
        let inner = "  ".repeat(indent + 1);
        let mut props = Vec::new();
        if let Some(ref s) = self.prompt_prefix {
            props.push(format!("{}prompt_prefix = '{}'", inner, s));
        }
        if let Some(ref s) = self.selection_caret {
            props.push(format!("{}selection_caret = '{}'", inner, s));
        }
        if let Some(ref p) = self.path_display {
            let v = p
                .iter()
                .map(|s| format!("'{}'", s))
                .collect::<Vec<_>>()
                .join(", ");
            props.push(format!("{}path_display = {{ {} }}", inner, v));
        }
        if let Some(v) = self.border {
            props.push(format!("{}border = {}", inner, v));
        }
        if let Some(v) = self.winblend {
            props.push(format!("{}winblend = {}", inner, v));
        }
        if let Some(ref s) = self.layout_strategy {
            props.push(format!("{}layout_strategy = '{}'", inner, s));
        }
        if let Some(ref s) = self.sorting_strategy {
            props.push(format!("{}sorting_strategy = '{}'", inner, s));
        }
        if let Some(ref m) = self.mappings {
            props.push(format!("{}mappings = {}", inner, m));
        }
        if props.is_empty() {
            "{}".to_string()
        } else {
            format!("{{\n{}\n{}}}", props.join(",\n"), pad)
        }
    }
}

pub struct TelescopeConfigNode {
    pub defaults: TelescopeDefaults,
    pub pickers: Option<String>,
    pub extensions: Option<String>,
    pub load_extensions: Vec<String>,
}

impl TelescopeConfigNode {
    pub fn new() -> Self {
        Self {
            defaults: TelescopeDefaults::new(),
            pickers: None,
            extensions: None,
            load_extensions: Vec::new(),
        }
    }
    pub fn defaults(mut self, d: TelescopeDefaults) -> Self {
        self.defaults = d;
        self
    }
    pub fn pickers(mut self, lua: &str) -> Self {
        self.pickers = Some(lua.to_string());
        self
    }
    pub fn extensions(mut self, lua: &str) -> Self {
        self.extensions = Some(lua.to_string());
        self
    }
    pub fn load_extension(mut self, name: &str) -> Self {
        self.load_extensions.push(name.to_string());
        self
    }
}

impl Config for TelescopeConfigNode {
    fn render(&self, ctx: &RenderContext) -> String {
        let i = ctx.indent();
        let defs = self.defaults.to_lua(ctx.indent_level + 1);
        let mut main_opts = format!("  defaults = {}", defs);
        if let Some(ref p) = self.pickers {
            main_opts.push_str(&format!(",\n  pickers = {}", p));
        }
        if let Some(ref e) = self.extensions {
            main_opts.push_str(&format!(",\n  extensions = {}", e));
        }

        let mut out = vec![format!(
            "{}require('telescope').setup({{\n{}\n{}}})",
            i, main_opts, i
        )];
        for ext in &self.load_extensions {
            out.push(format!(
                "{}require('telescope').load_extension('{}')",
                i, ext
            ));
        }
        out.join("\n")
    }
}
