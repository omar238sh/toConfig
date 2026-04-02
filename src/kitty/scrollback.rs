use super::{KittyConfig, KittyRenderContext};

/// Scrollback buffer configuration for kitty terminal.
///
/// # Example
/// ```
/// use toconfig::kitty::KittyConfig;
/// use toconfig::kitty::scrollback::ScrollbackConfig;
/// let s = ScrollbackConfig::new()
///     .lines(10000)
///     .pager("less --chop-long-lines --RAW-CONTROL-CHARS +INPUT_LINE_NUMBER");
/// let out = s.generate();
/// assert!(out.contains("scrollback_lines 10000"));
/// ```
#[derive(Default)]
pub struct ScrollbackConfig {
    /// Number of lines to keep in the scrollback buffer (`-1` for unlimited; use carefully).
    pub lines: Option<i64>,
    /// Pager program used to view scrollback output.
    pub pager: Option<String>,
    /// Fill new space with empty cells when `scrollback_pager_history_size` is set.
    pub pager_history_size: Option<u32>,
    /// Multiplier for wheel scroll events.
    pub wheel_scroll_multiplier: Option<f32>,
    /// Minimum number of lines to scroll per touch event.
    pub touch_scroll_multiplier: Option<f32>,
}

impl ScrollbackConfig {
    pub fn new() -> Self {
        Self::default()
    }

    /// Number of scrollback lines (`-1` = unlimited).
    pub fn lines(mut self, v: i64) -> Self {
        self.lines = Some(v);
        self
    }

    /// Pager command string used to view the scrollback buffer.
    pub fn pager(mut self, v: impl Into<String>) -> Self {
        self.pager = Some(v.into());
        self
    }

    /// History size (MB) passed to the pager (`0` disables the feature).
    pub fn pager_history_size(mut self, v: u32) -> Self {
        self.pager_history_size = Some(v);
        self
    }

    /// Mouse wheel scroll speed multiplier.
    pub fn wheel_scroll_multiplier(mut self, v: f32) -> Self {
        self.wheel_scroll_multiplier = Some(v);
        self
    }

    /// Touch scroll speed multiplier.
    pub fn touch_scroll_multiplier(mut self, v: f32) -> Self {
        self.touch_scroll_multiplier = Some(v);
        self
    }
}

impl KittyConfig for ScrollbackConfig {
    fn render(&self, ctx: &KittyRenderContext) -> String {
        let indent = ctx.indent();
        let mut lines: Vec<String> = Vec::new();
        if let Some(v) = self.lines {
            lines.push(format!("{}scrollback_lines {}", indent, v));
        }
        if let Some(ref v) = self.pager {
            lines.push(format!("{}scrollback_pager {}", indent, v));
        }
        if let Some(v) = self.pager_history_size {
            lines.push(format!(
                "{}scrollback_pager_history_size {}",
                indent, v
            ));
        }
        if let Some(v) = self.wheel_scroll_multiplier {
            lines.push(format!("{}wheel_scroll_multiplier {}", indent, v));
        }
        if let Some(v) = self.touch_scroll_multiplier {
            lines.push(format!("{}touch_scroll_multiplier {}", indent, v));
        }
        lines.join("\n")
    }
}
