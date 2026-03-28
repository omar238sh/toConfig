use std::path::PathBuf;

use super::{HyprlandConfig, HyprlandRenderContext};

/// Manages writing the generated Hyprland configuration to a file.
///
/// The default target is `~/.config/hypr/hyprland.conf`.
///
/// # Example
/// ```no_run
/// use toconfig::hyprland::{HyprlandConfigTree, Variable};
/// use toconfig::hyprland::output::HyprlandOutput;
///
/// let mut tree = HyprlandConfigTree::new();
/// tree.add(Variable::new("terminal", "kitty"));
///
/// HyprlandOutput::hyprland_conf().write(&tree).unwrap();
/// ```
pub struct HyprlandOutput {
    pub target: PathBuf,
}

impl HyprlandOutput {
    /// Write to the standard Hyprland config location:
    /// `$HOME/.config/hypr/hyprland.conf`.
    pub fn hyprland_conf() -> Self {
        Self {
            target: PathBuf::from(
                std::env::var("HOME").unwrap_or_else(|_| "/root".into())
                    + "/.config/hypr/hyprland.conf",
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
    pub fn write<C: HyprlandConfig>(&self, config: &C) -> std::io::Result<bool> {
        let ctx = HyprlandRenderContext::default();
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
    pub fn preview<C: HyprlandConfig>(&self, config: &C) -> String {
        let ctx = HyprlandRenderContext::default();
        config.render(&ctx)
    }
}
