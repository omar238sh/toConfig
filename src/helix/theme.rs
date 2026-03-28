use super::core::{HelixConfig, HelixRenderContext, toml_str};

/// Sets the active Helix theme via the top-level `theme` key.
///
/// Rendered as: `theme = "name"`
///
/// # Example
/// ```
/// # use toconfig::helix::HelixConfig;
/// use toconfig::helix::theme::ThemeSetting;
/// let t = ThemeSetting::new("gruvbox_dark");
/// assert_eq!(t.generate(), "theme = \"gruvbox_dark\"");
/// ```
pub struct ThemeSetting {
    pub name: String,
}

impl ThemeSetting {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

impl HelixConfig for ThemeSetting {
    fn render(&self, _ctx: &HelixRenderContext) -> String {
        format!("theme = {}", toml_str(&self.name))
    }

    fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("ThemeSetting: theme name cannot be empty".into());
        }
        Ok(())
    }
}
