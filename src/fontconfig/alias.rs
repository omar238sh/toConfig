//! `<alias>` element — font family substitution aliases.

use super::{FontconfigConfig, FontconfigRenderContext};
use super::value::xml_escape;

/// A fontconfig `<alias>` element, which substitutes one font family for others.
///
/// An alias can specify up to three preference lists:
/// * **prefer** — families tried *before* the original family.
/// * **accept** — families tried *after* the original family.
/// * **default** — families tried when nothing else matches.
///
/// # Example
/// ```
/// use toconfig::fontconfig::FontconfigConfig;
/// use toconfig::fontconfig::alias::Alias;
///
/// let alias = Alias::new("sans-serif")
///     .prefer(["Noto Sans", "DejaVu Sans"])
///     .accept(["Liberation Sans"]);
///
/// let out = alias.generate();
/// assert!(out.contains("<family>sans-serif</family>"));
/// assert!(out.contains("<prefer>"));
/// assert!(out.contains("<family>Noto Sans</family>"));
/// assert!(out.contains("<accept>"));
/// assert!(out.contains("<family>Liberation Sans</family>"));
/// ```
pub struct Alias {
    /// The source font family name.
    pub family: String,
    /// Families to try before the source family.
    pub prefer: Vec<String>,
    /// Families to try after the source family.
    pub accept: Vec<String>,
    /// Families to use as the final fallback.
    pub default: Vec<String>,
}

impl Alias {
    /// Create a new alias for `family` with no preference lists yet.
    pub fn new(family: impl Into<String>) -> Self {
        Self {
            family: family.into(),
            prefer: Vec::new(),
            accept: Vec::new(),
            default: Vec::new(),
        }
    }

    /// Set the `<prefer>` list (consuming builder).
    pub fn prefer(mut self, families: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.prefer = families.into_iter().map(Into::into).collect();
        self
    }

    /// Set the `<accept>` list (consuming builder).
    pub fn accept(mut self, families: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.accept = families.into_iter().map(Into::into).collect();
        self
    }

    /// Set the `<default>` list (consuming builder).
    pub fn default_families(
        mut self,
        families: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.default = families.into_iter().map(Into::into).collect();
        self
    }

    /// Append a single family to the `<prefer>` list (mutable borrow).
    pub fn add_prefer(&mut self, family: impl Into<String>) -> &mut Self {
        self.prefer.push(family.into());
        self
    }

    /// Append a single family to the `<accept>` list (mutable borrow).
    pub fn add_accept(&mut self, family: impl Into<String>) -> &mut Self {
        self.accept.push(family.into());
        self
    }

    /// Append a single family to the `<default>` list (mutable borrow).
    pub fn add_default(&mut self, family: impl Into<String>) -> &mut Self {
        self.default.push(family.into());
        self
    }
}

fn render_family_list(tag: &str, families: &[String], indent: &str, inner: &str) -> String {
    let mut lines = vec![format!("{}<{}>", indent, tag)];
    for f in families {
        lines.push(format!("{}<family>{}</family>", inner, xml_escape(f)));
    }
    lines.push(format!("{}</{}>", indent, tag));
    lines.join("\n")
}

impl FontconfigConfig for Alias {
    fn render(&self, ctx: &FontconfigRenderContext) -> String {
        let indent = ctx.indent();
        let inner_ctx = ctx.deeper();
        let inner = inner_ctx.indent();
        let inner2_ctx = inner_ctx.deeper();
        let inner2 = inner2_ctx.indent();

        let mut lines = vec![
            format!("{}<alias>", indent),
            format!("{}<family>{}</family>", inner, xml_escape(&self.family)),
        ];

        if !self.prefer.is_empty() {
            lines.push(render_family_list("prefer", &self.prefer, &inner, &inner2));
        }
        if !self.accept.is_empty() {
            lines.push(render_family_list("accept", &self.accept, &inner, &inner2));
        }
        if !self.default.is_empty() {
            lines.push(render_family_list("default", &self.default, &inner, &inner2));
        }

        lines.push(format!("{}</alias>", indent));
        lines.join("\n")
    }

    fn validate(&self) -> Result<(), String> {
        if self.family.is_empty() {
            return Err("Alias family name must not be empty".into());
        }
        Ok(())
    }
}
