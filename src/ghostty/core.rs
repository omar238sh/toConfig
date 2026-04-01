/// Render context passed to every [`GhosttyConfig`] node.
#[derive(Clone, Debug)]
pub struct GhosttyRenderContext {
    /// When `true`, doc comments are emitted above key-value pairs.
    pub emit_comments: bool,
}

impl Default for GhosttyRenderContext {
    fn default() -> Self {
        Self {
            emit_comments: true,
        }
    }
}

/// Trait implemented by every Ghostty configuration node.
///
/// The rendered output is plain `key = value` text suitable for
/// `~/.config/ghostty/config`.
pub trait GhosttyConfig {
    fn render(&self, ctx: &GhosttyRenderContext) -> String;

    fn generate(&self) -> String {
        self.render(&GhosttyRenderContext::default())
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Top-level container that aggregates [`GhosttyConfig`] nodes into a
/// complete configuration file.
#[derive(Default)]
pub struct GhosttyConfigTree {
    pub nodes: Vec<Box<dyn GhosttyConfig>>,
    pub header_comment: Option<String>,
}

impl GhosttyConfigTree {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push<N: GhosttyConfig + 'static>(&mut self, node: N) -> &mut Self {
        self.nodes.push(Box::new(node));
        self
    }

    pub fn with_header(mut self, comment: impl Into<String>) -> Self {
        self.header_comment = Some(comment.into());
        self
    }
}

impl GhosttyConfig for GhosttyConfigTree {
    fn render(&self, ctx: &GhosttyRenderContext) -> String {
        let mut out = String::new();

        if let Some(ref h) = self.header_comment {
            for line in h.lines() {
                out.push_str("# ");
                out.push_str(line);
                out.push('\n');
            }
            out.push('\n');
        }

        for node in &self.nodes {
            let s = node.render(ctx);
            if !s.is_empty() {
                out.push_str(&s);
                if !s.ends_with('\n') {
                    out.push('\n');
                }
            }
        }

        out
    }

    fn validate(&self) -> Result<(), String> {
        for node in &self.nodes {
            node.validate()?;
        }
        Ok(())
    }
}
