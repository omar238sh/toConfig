use super::core::{GhosttyConfig, GhosttyRenderContext};

/// Mouse behavior configuration for Ghostty.
///
/// Renders as `mouse-hide-while-typing`, `mouse-scroll-multiplier`,
/// and `focus-follows-mouse` entries.
#[derive(Default, Clone, Debug)]
pub struct MouseConfig {
    /// Hide the mouse cursor while the user is typing.
    pub hide_while_typing: Option<bool>,
    /// Multiplier applied to scroll wheel events (default 1.0).
    pub scroll_multiplier: Option<f64>,
    /// Give keyboard focus to whichever surface the pointer is over.
    pub focus_follows_mouse: Option<bool>,
}

impl MouseConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn hide_while_typing(mut self, v: bool) -> Self {
        self.hide_while_typing = Some(v);
        self
    }

    pub fn scroll_multiplier(mut self, v: f64) -> Self {
        self.scroll_multiplier = Some(v);
        self
    }

    pub fn focus_follows_mouse(mut self, v: bool) -> Self {
        self.focus_follows_mouse = Some(v);
        self
    }
}

impl GhosttyConfig for MouseConfig {
    fn render(&self, _ctx: &GhosttyRenderContext) -> String {
        let mut out = String::new();

        if let Some(v) = self.hide_while_typing {
            out.push_str(&format!("mouse-hide-while-typing = {}\n", v));
        }
        if let Some(v) = self.scroll_multiplier {
            out.push_str(&format!("mouse-scroll-multiplier = {}\n", v));
        }
        if let Some(v) = self.focus_follows_mouse {
            out.push_str(&format!("focus-follows-mouse = {}\n", v));
        }

        out
    }
}
