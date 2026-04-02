use std::path::PathBuf;

use super::core::{GhosttyConfig, GhosttyRenderContext};

/// Manages writing a generated Ghostty configuration to a file.
///
/// The default target is `$HOME/.config/ghostty/config`.
///
/// # Example
/// ```no_run
/// use toconfig::ghostty::{GhosttyConfigTree, output::GhosttyOutput};
///
/// let mut tree = GhosttyConfigTree::new();
/// GhosttyOutput::default_config().write(&tree).unwrap();
/// ```
pub struct GhosttyOutput {
    pub target: PathBuf,
}

impl GhosttyOutput {
    /// Write to the standard Ghostty config location:
    /// `$HOME/.config/ghostty/config`.
    pub fn default_config() -> Self {
        Self {
            target: PathBuf::from(
                std::env::var("HOME").unwrap_or_else(|_| "/root".into())
                    + "/.config/ghostty/config",
            ),
        }
    }

    /// Write to an arbitrary file path.
    pub fn at_path(path: &str) -> Self {
        Self {
            target: PathBuf::from(path),
        }
    }

    /// Render the config tree and write it to the target file.
    ///
    /// Performs a diff-check: if the file already exists and the content
    /// is identical, the file is **not** rewritten.
    ///
    /// Returns `Ok(true)` when the file was written, `Ok(false)` when it was
    /// skipped because the content was unchanged.
    pub fn write<C: GhosttyConfig>(&self, config: &C) -> std::io::Result<bool> {
        let ctx = GhosttyRenderContext::default();
        let content = config.render(&ctx);

        if self.target.exists() {
            let existing = std::fs::read_to_string(&self.target)?;
            if existing == content {
                return Ok(false);
            }
        }

        if let Some(parent) = self.target.parent() {
            std::fs::create_dir_all(parent)?;
        }

        std::fs::write(&self.target, &content)?;
        Ok(true)
    }

    /// Return the generated string without performing any I/O (preview mode).
    pub fn preview<C: GhosttyConfig>(&self, config: &C) -> String {
        let ctx = GhosttyRenderContext::default();
        config.render(&ctx)
    }
}
