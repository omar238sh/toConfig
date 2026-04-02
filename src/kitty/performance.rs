use super::{KittyConfig, KittyRenderContext};

/// Performance and rendering tuning for kitty terminal.
///
/// # Example
/// ```
/// use toconfig::kitty::KittyConfig;
/// use toconfig::kitty::performance::PerformanceConfig;
/// let p = PerformanceConfig::new()
///     .repaint_delay(10)
///     .input_delay(3)
///     .sync_to_monitor(true);
/// let out = p.generate();
/// assert!(out.contains("repaint_delay 10"));
/// ```
#[derive(Default)]
pub struct PerformanceConfig {
    /// Delay (ms) between successive screen redraws.
    pub repaint_delay: Option<u32>,
    /// Delay (ms) between reading input and updating the display.
    pub input_delay: Option<u32>,
    /// Sync redraws to the monitor's refresh rate (`yes`/`no`).
    pub sync_to_monitor: Option<bool>,
    /// Enable OpenGL vsync.
    pub enable_audio_bell: Option<bool>,
    /// Use a separate thread for rendering.
    pub render_factor: Option<f32>,
}

impl PerformanceConfig {
    pub fn new() -> Self {
        Self::default()
    }

    /// Milliseconds between repaints (default: 10).
    pub fn repaint_delay(mut self, v: u32) -> Self {
        self.repaint_delay = Some(v);
        self
    }

    /// Milliseconds of input latency before a repaint (default: 3).
    pub fn input_delay(mut self, v: u32) -> Self {
        self.input_delay = Some(v);
        self
    }

    /// Sync redraws to the monitor refresh cycle.
    pub fn sync_to_monitor(mut self, v: bool) -> Self {
        self.sync_to_monitor = Some(v);
        self
    }

    /// Enable the audible terminal bell.
    pub fn enable_audio_bell(mut self, v: bool) -> Self {
        self.enable_audio_bell = Some(v);
        self
    }

    /// Scale factor for the rendering thread (1.0 = default).
    pub fn render_factor(mut self, v: f32) -> Self {
        self.render_factor = Some(v);
        self
    }
}

impl KittyConfig for PerformanceConfig {
    fn render(&self, ctx: &KittyRenderContext) -> String {
        let indent = ctx.indent();
        let mut lines: Vec<String> = Vec::new();
        if let Some(v) = self.repaint_delay {
            lines.push(format!("{}repaint_delay {}", indent, v));
        }
        if let Some(v) = self.input_delay {
            lines.push(format!("{}input_delay {}", indent, v));
        }
        if let Some(v) = self.sync_to_monitor {
            lines.push(format!(
                "{}sync_to_monitor {}",
                indent,
                if v { "yes" } else { "no" }
            ));
        }
        if let Some(v) = self.enable_audio_bell {
            lines.push(format!(
                "{}enable_audio_bell {}",
                indent,
                if v { "yes" } else { "no" }
            ));
        }
        if let Some(v) = self.render_factor {
            lines.push(format!("{}render_factor {}", indent, v));
        }
        lines.join("\n")
    }
}
