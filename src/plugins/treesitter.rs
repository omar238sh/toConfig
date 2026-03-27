use crate::core::{Config, RenderContext};

#[derive(Debug, Clone, Default)]
pub struct TreesitterConfig {
    pub ensure_installed: Vec<String>,
    pub sync_install: Option<bool>,
    pub auto_install: Option<bool>,
    pub ignore_install: Vec<String>,
    pub highlight: Option<TreesitterHighlight>,
    pub indent: Option<bool>,
    pub incremental_selection: Option<String>,
    pub textobjects: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct TreesitterHighlight {
    pub enable: bool,
    pub disable: Vec<String>,
    pub additional_vim_regex_highlighting: Option<bool>,
}

impl TreesitterConfig {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn ensure_installed(mut self, langs: &[&str]) -> Self {
        self.ensure_installed = langs.iter().map(|s| s.to_string()).collect();
        self
    }
    pub fn sync_install(mut self, v: bool) -> Self {
        self.sync_install = Some(v);
        self
    }
    pub fn auto_install(mut self, v: bool) -> Self {
        self.auto_install = Some(v);
        self
    }
    pub fn ignore_install(mut self, langs: &[&str]) -> Self {
        self.ignore_install = langs.iter().map(|s| s.to_string()).collect();
        self
    }
    pub fn highlight(mut self, h: TreesitterHighlight) -> Self {
        self.highlight = Some(h);
        self
    }
    pub fn indent(mut self, v: bool) -> Self {
        self.indent = Some(v);
        self
    }
}

impl TreesitterHighlight {
    pub fn new(enable: bool) -> Self {
        Self {
            enable,
            ..Default::default()
        }
    }
    pub fn disable(mut self, langs: &[&str]) -> Self {
        self.disable = langs.iter().map(|s| s.to_string()).collect();
        self
    }
    pub fn additional_vim_regex_highlighting(mut self, v: bool) -> Self {
        self.additional_vim_regex_highlighting = Some(v);
        self
    }
}

impl Config for TreesitterConfig {
    fn render(&self, ctx: &RenderContext) -> String {
        let i = ctx.indent();
        let mut opts = Vec::new();
        if !self.ensure_installed.is_empty() {
            let langs = self
                .ensure_installed
                .iter()
                .map(|s| format!("'{}'", s))
                .collect::<Vec<_>>()
                .join(", ");
            opts.push(format!("  ensure_installed = {{ {} }}", langs));
        }
        if let Some(v) = self.sync_install {
            opts.push(format!("  sync_install = {}", v));
        }
        if let Some(v) = self.auto_install {
            opts.push(format!("  auto_install = {}", v));
        }
        if !self.ignore_install.is_empty() {
            let langs = self
                .ignore_install
                .iter()
                .map(|s| format!("'{}'", s))
                .collect::<Vec<_>>()
                .join(", ");
            opts.push(format!("  ignore_install = {{ {} }}", langs));
        }
        if let Some(ref h) = self.highlight {
            let mut hp = vec![format!("    enable = {}", h.enable)];
            if !h.disable.is_empty() {
                let d = h
                    .disable
                    .iter()
                    .map(|s| format!("'{}'", s))
                    .collect::<Vec<_>>()
                    .join(", ");
                hp.push(format!("    disable = {{ {} }}", d));
            }
            if let Some(v) = h.additional_vim_regex_highlighting {
                hp.push(format!("    additional_vim_regex_highlighting = {}", v));
            }
            opts.push(format!("  highlight = {{\n{}\n  }}", hp.join(",\n")));
        }
        if let Some(v) = self.indent {
            opts.push(format!("  indent = {{ enable = {} }}", v));
        }
        if let Some(ref io) = self.incremental_selection {
            opts.push(format!("  incremental_selection = {}", io));
        }
        if let Some(ref to) = self.textobjects {
            opts.push(format!("  textobjects = {}", to));
        }

        format!(
            "{}require('nvim-treesitter.configs').setup({{\n{}\n{}}})",
            i,
            opts.join(",\n"),
            i
        )
    }
}
