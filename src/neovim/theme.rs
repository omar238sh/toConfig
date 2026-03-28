use crate::core::{Config, RenderContext};

/// A validated 6-digit hex colour (e.g., "#1e1e2e").
#[derive(Debug, Clone)]
pub struct HexColor(pub String);

impl HexColor {
    /// Panics at construction if the format is wrong (compile-time safety in tests).
    pub fn new(hex: &str) -> Self {
        assert!(
            hex.starts_with('#') && hex.len() == 7,
            "Invalid hex color: {}",
            hex
        );
        Self(hex.to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// An attribute within a highlight group.
#[derive(Debug, Clone, Default)]
pub struct HighlightAttrs {
    pub fg: Option<HexColor>,
    pub bg: Option<HexColor>,
    pub sp: Option<HexColor>, // special / underline colour
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub underline: Option<bool>,
    pub undercurl: Option<bool>,
    pub strikethrough: Option<bool>,
    pub reverse: Option<bool>,
    pub link: Option<String>, // link to another highlight group
}

impl HighlightAttrs {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn fg(mut self, c: HexColor) -> Self {
        self.fg = Some(c);
        self
    }
    pub fn bg(mut self, c: HexColor) -> Self {
        self.bg = Some(c);
        self
    }
    pub fn sp(mut self, c: HexColor) -> Self {
        self.sp = Some(c);
        self
    }
    pub fn bold(mut self, v: bool) -> Self {
        self.bold = Some(v);
        self
    }
    pub fn italic(mut self, v: bool) -> Self {
        self.italic = Some(v);
        self
    }
    pub fn underline(mut self, v: bool) -> Self {
        self.underline = Some(v);
        self
    }
    pub fn undercurl(mut self, v: bool) -> Self {
        self.undercurl = Some(v);
        self
    }
    pub fn strikethrough(mut self, v: bool) -> Self {
        self.strikethrough = Some(v);
        self
    }
    pub fn reverse(mut self, v: bool) -> Self {
        self.reverse = Some(v);
        self
    }
    pub fn link(mut self, group: &str) -> Self {
        self.link = Some(group.to_string());
        self
    }

    pub fn to_lua_table(&self) -> String {
        let mut props = Vec::new();
        if let Some(ref c) = self.fg {
            props.push(format!("fg = '{}'", c.as_str()));
        }
        if let Some(ref c) = self.bg {
            props.push(format!("bg = '{}'", c.as_str()));
        }
        if let Some(ref c) = self.sp {
            props.push(format!("sp = '{}'", c.as_str()));
        }
        if let Some(v) = self.bold {
            props.push(format!("bold = {}", v));
        }
        if let Some(v) = self.italic {
            props.push(format!("italic = {}", v));
        }
        if let Some(v) = self.underline {
            props.push(format!("underline = {}", v));
        }
        if let Some(v) = self.undercurl {
            props.push(format!("undercurl = {}", v));
        }
        if let Some(v) = self.strikethrough {
            props.push(format!("strikethrough = {}", v));
        }
        if let Some(v) = self.reverse {
            props.push(format!("reverse = {}", v));
        }
        if let Some(ref l) = self.link {
            props.push(format!("link = '{}'", l));
        }
        if props.is_empty() {
            "{}".to_string()
        } else {
            format!("{{ {} }}", props.join(", "))
        }
    }
}

/// A single highlight group definition.
pub struct HighlightNode {
    pub group: String,
    pub attrs: HighlightAttrs,
    pub ns_id: u32, // 0 = global
}

impl HighlightNode {
    pub fn new(group: &str, attrs: HighlightAttrs) -> Self {
        Self {
            group: group.to_string(),
            attrs,
            ns_id: 0,
        }
    }
    pub fn ns(mut self, id: u32) -> Self {
        self.ns_id = id;
        self
    }
}

impl Config for HighlightNode {
    fn render(&self, ctx: &RenderContext) -> String {
        format!(
            "{}vim.api.nvim_set_hl({}, '{}', {})",
            ctx.indent(),
            self.ns_id,
            self.group,
            self.attrs.to_lua_table()
        )
    }
}

/// Sets the active colorscheme.
pub struct ColorschemeNode {
    pub name: String,
}

impl ColorschemeNode {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl Config for ColorschemeNode {
    fn render(&self, ctx: &RenderContext) -> String {
        format!("{}vim.cmd.colorscheme('{}')", ctx.indent(), self.name)
    }
}

/// A full theme definition: a colorscheme + a set of overrides.
pub struct ThemeNode {
    pub colorscheme: String,
    pub highlights: Vec<HighlightNode>,
}

impl ThemeNode {
    pub fn new(colorscheme: &str) -> Self {
        Self {
            colorscheme: colorscheme.to_string(),
            highlights: Vec::new(),
        }
    }

    pub fn override_hl(mut self, node: HighlightNode) -> Self {
        self.highlights.push(node);
        self
    }
}

impl Config for ThemeNode {
    fn render(&self, ctx: &RenderContext) -> String {
        let mut out = vec![format!(
            "{}vim.cmd.colorscheme('{}')",
            ctx.indent(),
            self.colorscheme
        )];
        for h in &self.highlights {
            out.push(h.render(ctx));
        }
        out.join("\n")
    }
}
