//! Core trait, rendering context and root document for fontconfig XML generation.

/// Rendering context passed during fontconfig XML generation.
///
/// Controls indentation so that nested elements are properly formatted.
#[derive(Debug, Clone)]
pub struct FontconfigRenderContext {
    pub indent_level: usize,
    pub indent_width: usize,
}

impl Default for FontconfigRenderContext {
    fn default() -> Self {
        Self {
            indent_level: 0,
            indent_width: 2,
        }
    }
}

impl FontconfigRenderContext {
    pub fn indent(&self) -> String {
        " ".repeat(self.indent_level * self.indent_width)
    }

    pub fn deeper(&self) -> Self {
        Self {
            indent_level: self.indent_level + 1,
            ..self.clone()
        }
    }
}

/// Central trait for all fontconfig configuration nodes.
///
/// Each node renders itself as an XML fragment.  Intentionally **distinct** from
/// [`crate::core::Config`] and [`crate::hyprland::HyprlandConfig`] so that nodes
/// from different backends cannot be mixed in the same tree.
pub trait FontconfigConfig {
    /// Render this node into an XML string using the provided context.
    fn render(&self, ctx: &FontconfigRenderContext) -> String;

    /// Convenience: render with the default context (no indentation).
    fn generate(&self) -> String {
        self.render(&FontconfigRenderContext::default())
    }

    /// Optional pre-render validation hook.
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Root fontconfig document.
///
/// Wraps all child nodes inside a full XML document with the standard fontconfig
/// DOCTYPE header and the `<fontconfig>` root element.
///
/// # Example
/// ```
/// use toconfig::fontconfig::{FontconfigConfig, FontconfigDocument};
/// use toconfig::fontconfig::dir::Dir;
/// use toconfig::fontconfig::alias::Alias;
///
/// let doc = FontconfigDocument::new()
///     .push(Dir::new("~/.local/share/fonts"))
///     .push(Alias::new("sans-serif").prefer(["Noto Sans", "DejaVu Sans"]));
///
/// let out = doc.generate();
/// assert!(out.starts_with("<?xml"));
/// assert!(out.contains("<fontconfig>"));
/// assert!(out.contains("<dir>~/.local/share/fonts</dir>"));
/// assert!(out.contains("<family>sans-serif</family>"));
/// ```
pub struct FontconfigDocument {
    pub nodes: Vec<Box<dyn FontconfigConfig>>,
}

impl Default for FontconfigDocument {
    fn default() -> Self {
        Self::new()
    }
}

impl FontconfigDocument {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    /// Append a node (consuming builder).
    pub fn push<C: FontconfigConfig + 'static>(mut self, node: C) -> Self {
        self.nodes.push(Box::new(node));
        self
    }

    /// Append a node (mutable borrow, returns `&mut Self` for chaining).
    pub fn add<C: FontconfigConfig + 'static>(&mut self, node: C) -> &mut Self {
        self.nodes.push(Box::new(node));
        self
    }

    /// Run [`FontconfigConfig::validate`] on every child node.
    pub fn validate_all(&self) -> Result<(), Vec<String>> {
        let errors: Vec<String> = self
            .nodes
            .iter()
            .filter_map(|n| n.validate().err())
            .collect();
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl FontconfigConfig for FontconfigDocument {
    fn render(&self, ctx: &FontconfigRenderContext) -> String {
        let inner_ctx = ctx.deeper();
        let mut parts = vec![
            r#"<?xml version="1.0"?>"#.to_string(),
            r#"<!DOCTYPE fontconfig SYSTEM "fonts.dtd">"#.to_string(),
            format!("{}<fontconfig>", ctx.indent()),
        ];
        for node in &self.nodes {
            let rendered = node.render(&inner_ctx);
            if !rendered.is_empty() {
                parts.push(rendered);
            }
        }
        parts.push(format!("{}</fontconfig>", ctx.indent()));
        parts.join("\n")
    }
}
