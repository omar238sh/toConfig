use super::color::HexColor;
use super::core::{GhosttyConfig, GhosttyRenderContext};

/// Shape of the terminal cursor.
#[derive(Clone, Debug)]
pub enum CursorStyle {
    Block,
    BlockHollow,
    Bar,
    Underline,
    UnderlineHollow,
}

impl std::fmt::Display for CursorStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Block => "block",
            Self::BlockHollow => "block_hollow",
            Self::Bar => "bar",
            Self::Underline => "underline",
            Self::UnderlineHollow => "underline_hollow",
        };
        write!(f, "{}", s)
    }
}

/// Cursor configuration for Ghostty.
///
/// Renders as `cursor-*` key-value entries.
#[derive(Default, Clone, Debug)]
pub struct CursorConfig {
    pub style: Option<CursorStyle>,
    /// `true` = blink, `false` = steady, `None` = follow system.
    pub style_blink: Option<bool>,
    /// Override the cursor fill color.
    pub color: Option<HexColor>,
    /// Override the character-under-cursor text color.
    pub text: Option<HexColor>,
    /// Allow placing the cursor by clicking.
    pub click_to_move: Option<bool>,
    /// Cursor opacity (0.0 – 1.0).
    pub opacity: Option<f64>,
}

impl CursorConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn style(mut self, s: CursorStyle) -> Self {
        self.style = Some(s);
        self
    }

    pub fn blink(mut self, v: bool) -> Self {
        self.style_blink = Some(v);
        self
    }

    pub fn color(mut self, c: HexColor) -> Self {
        self.color = Some(c);
        self
    }

    pub fn text_color(mut self, c: HexColor) -> Self {
        self.text = Some(c);
        self
    }

    pub fn click_to_move(mut self, v: bool) -> Self {
        self.click_to_move = Some(v);
        self
    }

    pub fn opacity(mut self, v: f64) -> Self {
        self.opacity = Some(v);
        self
    }
}

impl GhosttyConfig for CursorConfig {
    fn render(&self, _ctx: &GhosttyRenderContext) -> String {
        let mut out = String::new();

        if let Some(ref v) = self.style {
            out.push_str(&format!("cursor-style = {}\n", v));
        }
        if let Some(v) = self.style_blink {
            out.push_str(&format!("cursor-style-blink = {}\n", v));
        }
        if let Some(ref c) = self.color {
            out.push_str(&format!("cursor-color = {}\n", c));
        }
        if let Some(ref c) = self.text {
            out.push_str(&format!("cursor-text = {}\n", c));
        }
        if let Some(v) = self.click_to_move {
            out.push_str(&format!("cursor-click-to-move = {}\n", v));
        }
        if let Some(v) = self.opacity {
            out.push_str(&format!("cursor-opacity = {}\n", v));
        }

        out
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(v) = self.opacity
            && !(0.0..=1.0).contains(&v)
        {
            return Err(format!("cursor-opacity must be 0.0–1.0, got {}", v));
        }
        Ok(())
    }
}
