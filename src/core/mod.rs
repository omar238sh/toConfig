/// Rendering context passed during generation (controls indentation, doc comments, etc.)
#[derive(Debug, Clone)]
pub struct RenderContext {
    pub indent_level: usize,
    pub indent_width: usize,
    pub emit_doc_comments: bool,
}

impl Default for RenderContext {
    fn default() -> Self {
        Self {
            indent_level: 0,
            indent_width: 2,
            emit_doc_comments: false,
        }
    }
}

impl RenderContext {
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

/// The central trait every configuration node must implement.
pub trait Config {
    /// Render this node into a Lua string using the provided context.
    fn render(&self, ctx: &RenderContext) -> String;

    /// Convenience: render with default context.
    fn generate(&self) -> String {
        self.render(&RenderContext::default())
    }

    /// Optional doc-comment header for this node.
    fn doc_comment(&self) -> Option<&str> {
        None
    }

    /// Optional validation that runs before rendering.
    /// Returns Ok(()) or a descriptive error string.
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// A composite node that owns an ordered list of child config nodes.
/// Used to represent an entire init.lua or a logical section of it.
pub struct ConfigTree {
    pub nodes: Vec<Box<dyn Config>>,
    pub section_comment: Option<String>,
}

impl ConfigTree {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            section_comment: None,
        }
    }

    pub fn with_comment(mut self, comment: &str) -> Self {
        self.section_comment = Some(comment.to_string());
        self
    }

    pub fn add<C: Config + 'static>(&mut self, node: C) -> &mut Self {
        self.nodes.push(Box::new(node));
        self
    }

    pub fn push<C: Config + 'static>(mut self, node: C) -> Self {
        self.nodes.push(Box::new(node));
        self
    }

    /// Validate all children before rendering.
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

impl Config for ConfigTree {
    fn render(&self, ctx: &RenderContext) -> String {
        let mut parts = Vec::new();
        if let Some(ref c) = self.section_comment {
            parts.push(format!("{}-- {}", ctx.indent(), c));
        }
        for node in &self.nodes {
            if ctx.emit_doc_comments {
                if let Some(doc) = node.doc_comment() {
                    parts.push(format!("{}--- {}", ctx.indent(), doc));
                }
            }
            parts.push(node.render(ctx));
        }
        parts.join("\n")
    }
}
