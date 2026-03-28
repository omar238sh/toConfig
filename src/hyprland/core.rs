//! Core traits and rendering context for Hyprland configuration.

/// Rendering context for Hyprland configuration (tracks indentation for nested sections).
#[derive(Debug, Clone)]
pub struct HyprlandRenderContext {
    pub indent_level: usize,
    pub indent_width: usize,
}

impl Default for HyprlandRenderContext {
    fn default() -> Self {
        Self {
            indent_level: 0,
            indent_width: 4,
        }
    }
}

impl HyprlandRenderContext {
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

/// Central trait for all Hyprland configuration nodes.
///
/// Intentionally **distinct** from [`crate::core::Config`] (the Neovim trait) so that
/// Hyprland and Neovim nodes cannot be added to the same configuration tree.
pub trait HyprlandConfig {
    /// Render this node into a Hyprland config string using the provided context.
    fn render(&self, ctx: &HyprlandRenderContext) -> String;

    /// Convenience: render with default context.
    fn generate(&self) -> String {
        self.render(&HyprlandRenderContext::default())
    }

    /// Optional pre-render validation hook.
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Top-level Hyprland configuration tree.
///
/// Accepts only nodes that implement [`HyprlandConfig`].  Neovim
/// [`crate::core::Config`] nodes are rejected at compile time because they
/// implement a different trait — the two cannot be mixed in the same tree.
pub struct HyprlandConfigTree {
    pub nodes: Vec<Box<dyn HyprlandConfig>>,
    pub header_comment: Option<String>,
}

impl Default for HyprlandConfigTree {
    fn default() -> Self {
        Self::new()
    }
}

impl HyprlandConfigTree {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            header_comment: None,
        }
    }

    /// Attach a header comment to the top of the generated file.
    pub fn with_comment(mut self, comment: &str) -> Self {
        self.header_comment = Some(comment.to_string());
        self
    }

    /// Append a node (mutable borrow, returns `&mut Self` for chaining).
    pub fn add<C: HyprlandConfig + 'static>(&mut self, node: C) -> &mut Self {
        self.nodes.push(Box::new(node));
        self
    }

    /// Append a node (consuming, returns `Self` for builder chains).
    pub fn push<C: HyprlandConfig + 'static>(mut self, node: C) -> Self {
        self.nodes.push(Box::new(node));
        self
    }

    /// Run [`HyprlandConfig::validate`] on every child node.
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

impl HyprlandConfig for HyprlandConfigTree {
    fn render(&self, ctx: &HyprlandRenderContext) -> String {
        let mut parts: Vec<String> = Vec::new();
        if let Some(ref c) = self.header_comment {
            for line in c.lines() {
                parts.push(format!("{}# {}", ctx.indent(), line));
            }
        }
        for node in &self.nodes {
            let rendered = node.render(ctx);
            if !rendered.is_empty() {
                parts.push(rendered);
            }
        }
        parts.join("\n")
    }
}
