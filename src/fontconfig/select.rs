//! `<selectfont>` element — accept or reject specific fonts from being used.

use super::{FontconfigConfig, FontconfigRenderContext};
use super::value::xml_escape;

// ── SelectAction ─────────────────────────────────────────────────────────────

/// Whether the patterns inside a [`SelectFont`] describe fonts to accept or reject.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectAction {
    /// `<acceptfont>` — only these fonts are made available.
    Accept,
    /// `<rejectfont>` — these fonts are excluded.
    Reject,
}

// ── PatternElement ────────────────────────────────────────────────────────────

/// A single `<patelt>` inside a [`SelectPattern`].
///
/// Matches a font property by name against a value.
///
/// # Example
/// ```
/// use toconfig::fontconfig::select::PatternElement;
/// use toconfig::fontconfig::value::FontconfigValue;
/// use toconfig::fontconfig::FontconfigConfig;
///
/// let pe = PatternElement::new("scalable", FontconfigValue::boolean(false));
/// let out = pe.generate();
/// assert!(out.contains(r#"name="scalable""#));
/// assert!(out.contains("<bool>false</bool>"));
/// ```
pub struct PatternElement {
    /// Property name.
    pub name: String,
    /// Value to match.
    pub value: super::value::FontconfigValue,
}

impl PatternElement {
    pub fn new(name: impl Into<String>, value: super::value::FontconfigValue) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }
}

impl FontconfigConfig for PatternElement {
    fn render(&self, ctx: &FontconfigRenderContext) -> String {
        let indent = ctx.indent();
        let inner_ctx = ctx.deeper();
        format!(
            "{}<patelt name=\"{}\">\n{}\n{}</patelt>",
            indent,
            xml_escape(&self.name),
            self.value.render_xml(&inner_ctx),
            indent
        )
    }

    fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("PatternElement name must not be empty".into());
        }
        Ok(())
    }
}

// ── SelectPattern ─────────────────────────────────────────────────────────────

/// A `<pattern>` element inside a [`SelectFont`] accept/reject block.
///
/// A font must match *all* `<patelt>` elements in the pattern to be selected.
///
/// # Example
/// ```
/// use toconfig::fontconfig::select::{SelectPattern, PatternElement};
/// use toconfig::fontconfig::value::FontconfigValue;
/// use toconfig::fontconfig::FontconfigConfig;
///
/// let pat = SelectPattern::new()
///     .element(PatternElement::new("family", FontconfigValue::string("Arial")));
/// let out = pat.generate();
/// assert!(out.contains("<pattern>"));
/// assert!(out.contains("Arial"));
/// ```
pub struct SelectPattern {
    /// The `<patelt>` conditions that define this pattern.
    pub elements: Vec<PatternElement>,
}

impl Default for SelectPattern {
    fn default() -> Self {
        Self::new()
    }
}

impl SelectPattern {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    /// Append a `<patelt>` element (consuming builder).
    pub fn element(mut self, pe: PatternElement) -> Self {
        self.elements.push(pe);
        self
    }

    /// Append a `<patelt>` element (mutable borrow).
    pub fn add_element(&mut self, pe: PatternElement) -> &mut Self {
        self.elements.push(pe);
        self
    }
}

impl FontconfigConfig for SelectPattern {
    fn render(&self, ctx: &FontconfigRenderContext) -> String {
        let indent = ctx.indent();
        let inner_ctx = ctx.deeper();
        let mut lines = vec![format!("{}<pattern>", indent)];
        for elem in &self.elements {
            lines.push(elem.render(&inner_ctx));
        }
        lines.push(format!("{}</pattern>", indent));
        lines.join("\n")
    }
}

// ── Glob ─────────────────────────────────────────────────────────────────────

/// A `<glob>` element inside a [`SelectFont`] accept/reject block.
///
/// Matches fonts by their file path using a glob pattern.
///
/// # Example
/// ```
/// use toconfig::fontconfig::select::Glob;
/// use toconfig::fontconfig::FontconfigConfig;
///
/// let g = Glob::new("/usr/share/fonts/noto/*");
/// assert!(g.generate().contains("<glob>/usr/share/fonts/noto/*</glob>"));
/// ```
pub struct Glob {
    pub pattern: String,
}

impl Glob {
    pub fn new(pattern: impl Into<String>) -> Self {
        Self {
            pattern: pattern.into(),
        }
    }
}

impl FontconfigConfig for Glob {
    fn render(&self, ctx: &FontconfigRenderContext) -> String {
        format!(
            "{}<glob>{}</glob>",
            ctx.indent(),
            xml_escape(&self.pattern)
        )
    }

    fn validate(&self) -> Result<(), String> {
        if self.pattern.is_empty() {
            return Err("Glob pattern must not be empty".into());
        }
        Ok(())
    }
}

// ── SelectFont ───────────────────────────────────────────────────────────────

/// A `<selectfont>` element — limits the set of usable fonts.
///
/// Contains one or more `<acceptfont>` or `<rejectfont>` blocks, each holding
/// [`SelectPattern`] or [`Glob`] matchers.
///
/// # Example
/// ```
/// use toconfig::fontconfig::FontconfigConfig;
/// use toconfig::fontconfig::select::{SelectFont, SelectAction, SelectPattern, PatternElement, Glob};
/// use toconfig::fontconfig::value::FontconfigValue;
///
/// let sf = SelectFont::new()
///     .block(
///         SelectAction::Reject,
///         vec![],
///         vec![Glob::new("/usr/share/fonts/Type1/*")],
///     );
///
/// let out = sf.generate();
/// assert!(out.contains("<selectfont>"));
/// assert!(out.contains("<rejectfont>"));
/// assert!(out.contains("/usr/share/fonts/Type1/*"));
/// ```
pub struct SelectBlock {
    pub action: SelectAction,
    pub patterns: Vec<SelectPattern>,
    pub globs: Vec<Glob>,
}

/// A complete `<selectfont>` element containing one or more accept/reject blocks.
pub struct SelectFont {
    pub blocks: Vec<SelectBlock>,
}

impl Default for SelectFont {
    fn default() -> Self {
        Self::new()
    }
}

impl SelectFont {
    pub fn new() -> Self {
        Self { blocks: Vec::new() }
    }

    /// Add a block with the given action, patterns, and globs (consuming builder).
    pub fn block(
        mut self,
        action: SelectAction,
        patterns: Vec<SelectPattern>,
        globs: Vec<Glob>,
    ) -> Self {
        self.blocks.push(SelectBlock {
            action,
            patterns,
            globs,
        });
        self
    }

    /// Add a block (mutable borrow).
    pub fn add_block(
        &mut self,
        action: SelectAction,
        patterns: Vec<SelectPattern>,
        globs: Vec<Glob>,
    ) -> &mut Self {
        self.blocks.push(SelectBlock {
            action,
            patterns,
            globs,
        });
        self
    }
}

impl FontconfigConfig for SelectFont {
    fn render(&self, ctx: &FontconfigRenderContext) -> String {
        let indent = ctx.indent();
        let inner_ctx = ctx.deeper();
        let inner = inner_ctx.indent();
        let inner2_ctx = inner_ctx.deeper();

        let mut lines = vec![format!("{}<selectfont>", indent)];
        for block in &self.blocks {
            let tag = match block.action {
                SelectAction::Accept => "acceptfont",
                SelectAction::Reject => "rejectfont",
            };
            lines.push(format!("{}<{}>", inner, tag));
            for pat in &block.patterns {
                lines.push(pat.render(&inner2_ctx));
            }
            for glob in &block.globs {
                lines.push(glob.render(&inner2_ctx));
            }
            lines.push(format!("{}</{}>", inner, tag));
        }
        lines.push(format!("{}</selectfont>", indent));
        lines.join("\n")
    }
}
