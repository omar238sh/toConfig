use crate::core::{Config, RenderContext};

/// A composite node that holds an ordered list of fish config nodes.
pub struct FishConfigTree {
    pub nodes: Vec<Box<dyn Config>>,
    pub comment: Option<String>,
}

impl Default for FishConfigTree {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            comment: None,
        }
    }
}

impl FishConfigTree {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_comment(mut self, c: &str) -> Self {
        self.comment = Some(c.to_string());
        self
    }

    pub fn add<C: Config + 'static>(mut self, node: C) -> Self {
        self.nodes.push(Box::new(node));
        self
    }

    pub fn push_node<C: Config + 'static>(&mut self, node: C) -> &mut Self {
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

impl Config for FishConfigTree {
    fn render(&self, ctx: &RenderContext) -> String {
        let mut parts = Vec::new();
        if let Some(ref c) = self.comment {
            parts.push(format!("{}# {}", ctx.indent(), c));
        }
        for node in &self.nodes {
            if ctx.emit_doc_comments {
                if let Some(doc) = node.doc_comment() {
                    parts.push(format!("{}# {}", ctx.indent(), doc));
                }
            }
            parts.push(node.render(ctx));
        }
        parts.join("\n")
    }
}
