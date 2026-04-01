use super::{KittyConfig, KittyRenderContext};

/// Cursor shape variants for kitty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorShape {
    /// A solid block cursor (█).
    Block,
    /// An underline cursor (_).
    Underline,
    /// A vertical bar cursor (|).
    Beam,
}

impl CursorShape {
    fn as_str(self) -> &'static str {
        match self {
            CursorShape::Block => "block",
            CursorShape::Underline => "underline",
            CursorShape::Beam => "beam",
        }
    }
}

/// Cursor configuration for kitty terminal.
///
/// Controls the cursor color, shape, blinking behavior, and related settings.
///
/// # Example
/// ```
/// use toconfig::kitty::KittyConfig;
/// use toconfig::kitty::cursor::{CursorConfig, CursorShape};
/// let c = CursorConfig::new()
///     .color("#cdd6f4")
///     .text_color("#1e1e2e")
///     .shape(CursorShape::Beam)
///     .blink_interval(0.5)
///     .stop_blinking_after(15.0);
/// let out = c.generate();
/// assert!(out.contains("cursor_shape beam"));
/// ```
#[derive(Default)]
pub struct CursorConfig {
    /// Cursor fill color (hex or `"none"` to use foreground color).
    pub color: Option<String>,
    /// Color drawn for the character under the cursor.
    pub text_color: Option<String>,
    /// Cursor shape.
    pub shape: Option<CursorShape>,
    /// Cursor blink interval in seconds (`0` = no blinking).
    pub blink_interval: Option<f32>,
    /// Stop blinking after this many seconds of inactivity (`0` = never stop).
    pub stop_blinking_after: Option<f32>,
    /// Cursor beam thickness in pixels.
    pub beam_thickness: Option<f32>,
    /// Cursor underline thickness in pixels.
    pub underline_thickness: Option<f32>,
}

impl CursorConfig {
    pub fn new() -> Self {
        Self::default()
    }

    /// Cursor fill color (hex string or `"none"`).
    pub fn color(mut self, v: impl Into<String>) -> Self {
        self.color = Some(v.into());
        self
    }

    /// Color of the character rendered under the cursor.
    pub fn text_color(mut self, v: impl Into<String>) -> Self {
        self.text_color = Some(v.into());
        self
    }

    /// Cursor shape variant.
    pub fn shape(mut self, v: CursorShape) -> Self {
        self.shape = Some(v);
        self
    }

    /// Blink interval in seconds (`0.0` disables blinking).
    pub fn blink_interval(mut self, v: f32) -> Self {
        self.blink_interval = Some(v);
        self
    }

    /// Stop blinking after N seconds of keyboard inactivity.
    pub fn stop_blinking_after(mut self, v: f32) -> Self {
        self.stop_blinking_after = Some(v);
        self
    }

    /// Thickness of the beam cursor in pixels.
    pub fn beam_thickness(mut self, v: f32) -> Self {
        self.beam_thickness = Some(v);
        self
    }

    /// Thickness of the underline cursor in pixels.
    pub fn underline_thickness(mut self, v: f32) -> Self {
        self.underline_thickness = Some(v);
        self
    }
}

impl KittyConfig for CursorConfig {
    fn render(&self, ctx: &KittyRenderContext) -> String {
        let indent = ctx.indent();
        let mut lines: Vec<String> = Vec::new();
        if let Some(ref v) = self.color {
            lines.push(format!("{}cursor {}", indent, v));
        }
        if let Some(ref v) = self.text_color {
            lines.push(format!("{}cursor_text_color {}", indent, v));
        }
        if let Some(v) = self.shape {
            lines.push(format!("{}cursor_shape {}", indent, v.as_str()));
        }
        if let Some(v) = self.blink_interval {
            lines.push(format!("{}cursor_blink_interval {}", indent, v));
        }
        if let Some(v) = self.stop_blinking_after {
            lines.push(format!("{}cursor_stop_blinking_after {}", indent, v));
        }
        if let Some(v) = self.beam_thickness {
            lines.push(format!("{}cursor_beam_thickness {}", indent, v));
        }
        if let Some(v) = self.underline_thickness {
            lines.push(format!("{}cursor_underline_thickness {}", indent, v));
        }
        lines.join("\n")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(v) = self.blink_interval {
            if v < 0.0 {
                return Err(format!(
                    "CursorConfig: blink_interval must be ≥ 0.0, got {}",
                    v
                ));
            }
        }
        Ok(())
    }
}
