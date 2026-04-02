use super::core::{GhosttyConfig, GhosttyRenderContext};

/// Auto-update channel selection.
#[derive(Clone, Debug)]
pub enum AutoUpdateChannel {
    Stable,
    Tip,
}

impl std::fmt::Display for AutoUpdateChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Stable => "stable",
                Self::Tip => "tip",
            }
        )
    }
}

/// Bell feature flags.
///
/// Pass as individual strings, e.g. `"audio"`, `"visual"`, `"system"`.
/// Use `"false"` to disable the bell entirely.
#[derive(Clone, Debug)]
pub enum BellFeature {
    Audio,
    Visual,
    System,
    /// Disable the bell.
    False,
    /// Any feature string not listed above.
    Custom(String),
}

impl std::fmt::Display for BellFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Audio => write!(f, "audio"),
            Self::Visual => write!(f, "visual"),
            Self::System => write!(f, "system"),
            Self::False => write!(f, "false"),
            Self::Custom(s) => write!(f, "{}", s),
        }
    }
}

/// Miscellaneous application-level settings that don't belong to a more
/// specific group.
#[derive(Default, Clone, Debug)]
pub struct MiscConfig {
    /// Close the application after the last window closes.
    pub quit_after_last_window_closed: Option<bool>,
    /// Prompt before closing a surface that has a running process.
    pub confirm_close_surface: Option<bool>,
    /// Enable automatic update checks.
    pub auto_update: Option<bool>,
    pub auto_update_channel: Option<AutoUpdateChannel>,
    /// Bell feature flags rendered as individual `bell-features` entries.
    pub bell_features: Vec<BellFeature>,
    /// Path to an additional config file to `@include`.
    pub include: Option<String>,
}

impl MiscConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn quit_after_last_window_closed(mut self, v: bool) -> Self {
        self.quit_after_last_window_closed = Some(v);
        self
    }

    pub fn confirm_close_surface(mut self, v: bool) -> Self {
        self.confirm_close_surface = Some(v);
        self
    }

    pub fn auto_update(mut self, v: bool) -> Self {
        self.auto_update = Some(v);
        self
    }

    pub fn auto_update_channel(mut self, c: AutoUpdateChannel) -> Self {
        self.auto_update_channel = Some(c);
        self
    }

    pub fn add_bell_feature(mut self, f: BellFeature) -> Self {
        self.bell_features.push(f);
        self
    }

    pub fn include(mut self, path: impl Into<String>) -> Self {
        self.include = Some(path.into());
        self
    }
}

impl GhosttyConfig for MiscConfig {
    fn render(&self, _ctx: &GhosttyRenderContext) -> String {
        let mut out = String::new();

        if let Some(v) = self.quit_after_last_window_closed {
            out.push_str(&format!("quit-after-last-window-closed = {}\n", v));
        }
        if let Some(v) = self.confirm_close_surface {
            out.push_str(&format!("confirm-close-surface = {}\n", v));
        }
        if let Some(v) = self.auto_update {
            let s = if v { "check" } else { "off" };
            out.push_str(&format!("auto-update = {}\n", s));
        }
        if let Some(ref c) = self.auto_update_channel {
            out.push_str(&format!("auto-update-channel = {}\n", c));
        }
        for feat in &self.bell_features {
            out.push_str(&format!("bell-features = {}\n", feat));
        }
        if let Some(ref path) = self.include {
            out.push_str(&format!("@include = {}\n", path));
        }

        out
    }
}
