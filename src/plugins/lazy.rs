use crate::core::{Config, RenderContext};

/// Every lazy.nvim spec field is modeled here.
#[derive(Debug, Clone)]
pub struct Plugin {
    pub name: String,
    pub dir: Option<String>,
    pub url: Option<String>,
    pub lazy: Option<bool>,
    pub enabled: Option<bool>,
    pub pin: Option<bool>,
    pub build: Option<String>,
    pub branch: Option<String>,
    pub tag: Option<String>,
    pub commit: Option<String>,
    pub version: Option<String>,
    pub priority: Option<i32>,
    pub dependencies: Vec<Plugin>,
    pub event: Vec<String>,
    pub cmd: Vec<String>,
    pub ft: Vec<String>,
    pub keys: Vec<String>,
    pub init: Option<String>,
    pub config: Option<String>,
    pub opts: Option<String>,
    pub main: Option<String>,
    pub cond: Option<String>,
    pub submodules: Option<bool>,
}

impl Plugin {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            dir: None,
            url: None,
            lazy: None,
            enabled: None,
            pin: None,
            build: None,
            branch: None,
            tag: None,
            commit: None,
            version: None,
            priority: None,
            dependencies: Vec::new(),
            event: Vec::new(),
            cmd: Vec::new(),
            ft: Vec::new(),
            keys: Vec::new(),
            init: None,
            config: None,
            opts: None,
            main: None,
            cond: None,
            submodules: None,
        }
    }

    // ── Fluent setters ───────────────────────────────────────────────────────
    pub fn lazy(mut self, v: bool) -> Self {
        self.lazy = Some(v);
        self
    }
    pub fn enabled(mut self, v: bool) -> Self {
        self.enabled = Some(v);
        self
    }
    pub fn pin(mut self, v: bool) -> Self {
        self.pin = Some(v);
        self
    }
    pub fn build(mut self, s: &str) -> Self {
        self.build = Some(s.to_string());
        self
    }
    pub fn branch(mut self, s: &str) -> Self {
        self.branch = Some(s.to_string());
        self
    }
    pub fn tag(mut self, s: &str) -> Self {
        self.tag = Some(s.to_string());
        self
    }
    pub fn commit(mut self, s: &str) -> Self {
        self.commit = Some(s.to_string());
        self
    }
    pub fn version(mut self, s: &str) -> Self {
        self.version = Some(s.to_string());
        self
    }
    pub fn priority(mut self, n: i32) -> Self {
        self.priority = Some(n);
        self
    }
    pub fn dep(mut self, d: Plugin) -> Self {
        self.dependencies.push(d);
        self
    }
    pub fn event(mut self, e: &str) -> Self {
        self.event.push(e.to_string());
        self
    }
    pub fn cmd(mut self, c: &str) -> Self {
        self.cmd.push(c.to_string());
        self
    }
    pub fn ft(mut self, f: &str) -> Self {
        self.ft.push(f.to_string());
        self
    }
    pub fn keys(mut self, k: &str) -> Self {
        self.keys.push(k.to_string());
        self
    }
    pub fn init(mut self, lua: &str) -> Self {
        self.init = Some(lua.to_string());
        self
    }
    pub fn config(mut self, lua: &str) -> Self {
        self.config = Some(lua.to_string());
        self
    }
    pub fn opts(mut self, lua: &str) -> Self {
        self.opts = Some(lua.to_string());
        self
    }
    pub fn main(mut self, m: &str) -> Self {
        self.main = Some(m.to_string());
        self
    }
    pub fn cond(mut self, lua: &str) -> Self {
        self.cond = Some(lua.to_string());
        self
    }
    pub fn submodules(mut self, v: bool) -> Self {
        self.submodules = Some(v);
        self
    }
    pub fn dir(mut self, d: &str) -> Self {
        self.dir = Some(d.to_string());
        self
    }
    pub fn url(mut self, u: &str) -> Self {
        self.url = Some(u.to_string());
        self
    }

    fn str_list(items: &[String]) -> String {
        if items.len() == 1 {
            format!("'{}'", items[0])
        } else {
            format!(
                "{{ {} }}",
                items
                    .iter()
                    .map(|s| format!("'{}'", s))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }
    }

    pub fn to_lua_table(&self, indent: usize) -> String {
        let pad = "  ".repeat(indent);
        let inner = "  ".repeat(indent + 1);
        let mut props = Vec::new();

        props.push(format!("{}'{}'", inner, self.name));
        if let Some(ref d) = self.dir {
            props.push(format!("{}dir = '{}'", inner, d));
        }
        if let Some(ref u) = self.url {
            props.push(format!("{}url = '{}'", inner, u));
        }
        if let Some(v) = self.lazy {
            props.push(format!("{}lazy = {}", inner, v));
        }
        if let Some(v) = self.enabled {
            props.push(format!("{}enabled = {}", inner, v));
        }
        if let Some(v) = self.pin {
            props.push(format!("{}pin = {}", inner, v));
        }
        if let Some(ref v) = self.build {
            props.push(format!("{}build = '{}'", inner, v));
        }
        if let Some(ref v) = self.branch {
            props.push(format!("{}branch = '{}'", inner, v));
        }
        if let Some(ref v) = self.tag {
            props.push(format!("{}tag = '{}'", inner, v));
        }
        if let Some(ref v) = self.commit {
            props.push(format!("{}commit = '{}'", inner, v));
        }
        if let Some(ref v) = self.version {
            props.push(format!("{}version = '{}'", inner, v));
        }
        if let Some(v) = self.priority {
            props.push(format!("{}priority = {}", inner, v));
        }
        if let Some(ref v) = self.cond {
            props.push(format!("{}cond = {}", inner, v));
        }
        if let Some(v) = self.submodules {
            props.push(format!("{}submodules = {}", inner, v));
        }

        if !self.dependencies.is_empty() {
            let deps = self
                .dependencies
                .iter()
                .map(|d| d.to_lua_table(indent + 2))
                .collect::<Vec<_>>()
                .join(",\n");
            props.push(format!("{}dependencies = {{\n{}\n{}}}", inner, deps, inner));
        }
        if !self.event.is_empty() {
            props.push(format!("{}event = {}", inner, Self::str_list(&self.event)));
        }
        if !self.cmd.is_empty() {
            props.push(format!("{}cmd = {}", inner, Self::str_list(&self.cmd)));
        }
        if !self.ft.is_empty() {
            props.push(format!("{}ft = {}", inner, Self::str_list(&self.ft)));
        }
        if !self.keys.is_empty() {
            props.push(format!("{}keys = {{ {} }}", inner, self.keys.join(", ")));
        }
        if let Some(ref v) = self.main {
            props.push(format!("{}main = '{}'", inner, v));
        }
        if let Some(ref v) = self.init {
            props.push(format!("{}init = {}", inner, v));
        }
        if let Some(ref v) = self.config {
            props.push(format!("{}config = {}", inner, v));
        }
        if let Some(ref v) = self.opts {
            props.push(format!("{}opts = {}", inner, v));
        }

        format!("{}{{\n{}\n{}}}", pad, props.join(",\n"), pad)
    }
}

impl Config for Plugin {
    fn render(&self, ctx: &RenderContext) -> String {
        self.to_lua_table(ctx.indent_level)
    }
}

/// The lazy.nvim bootstrap + setup node.
pub struct LazyManager {
    pub plugins: Vec<Plugin>,
    pub opts: Option<String>,
}

impl LazyManager {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
            opts: None,
        }
    }
    pub fn plugin(mut self, p: Plugin) -> Self {
        self.plugins.push(p);
        self
    }
    pub fn opts(mut self, lua: &str) -> Self {
        self.opts = Some(lua.to_string());
        self
    }
}

impl Config for LazyManager {
    fn render(&self, ctx: &RenderContext) -> String {
        let indent = ctx.indent();
        let bootstrap = format!(
            "{0}local lazypath = vim.fn.stdpath('data') .. '/lazy/lazy.nvim'\n\
             {0}if not vim.loop.fs_stat(lazypath) then\n\
             {0}  vim.fn.system({{ 'git', 'clone', '--filter=blob:none',\n\
             {0}    'https://github.com/folke/lazy.nvim.git', '--branch=stable', lazypath }})\n\
             {0}end\n\
             {0}vim.opt.rtp:prepend(lazypath)",
            indent
        );

        let plugins_lua = self
            .plugins
            .iter()
            .map(|p| p.to_lua_table(ctx.indent_level + 1))
            .collect::<Vec<_>>()
            .join(",\n");

        let opts_str = self.opts.clone().unwrap_or_else(|| "{}".into());

        format!(
            "{}\n{}require('lazy').setup({{\n{}\n{}}}, {})",
            bootstrap, indent, plugins_lua, indent, opts_str
        )
    }
}
