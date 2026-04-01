use super::{KittyConfig, KittyRenderContext};

/// Complete color scheme for kitty terminal.
///
/// Covers foreground/background, the 16 standard ANSI colors (color0–color15),
/// selection colors, URL color, and the mark highlight colors.
///
/// Colors must be CSS-style hex strings (`"#rrggbb"`) or named X11 colors.
///
/// # Example
/// ```
/// use toconfig::kitty::KittyConfig;
/// use toconfig::kitty::colors::ColorScheme;
/// let cs = ColorScheme::new()
///     .foreground("#dddddd")
///     .background("#1e1e2e")
///     .color0("#45475a");
/// let out = cs.generate();
/// assert!(out.contains("foreground"));
/// assert!(out.contains("background"));
/// ```
#[derive(Default)]
pub struct ColorScheme {
    pub foreground: Option<String>,
    pub background: Option<String>,
    pub selection_foreground: Option<String>,
    pub selection_background: Option<String>,
    pub url_color: Option<String>,
    pub visual_bell_color: Option<String>,
    /// The 16 standard ANSI palette entries (color0 – color15).
    pub ansi: [Option<String>; 16],
}

impl ColorScheme {
    pub fn new() -> Self {
        Self::default()
    }

    /// Main text color.
    pub fn foreground(mut self, v: impl Into<String>) -> Self {
        self.foreground = Some(v.into());
        self
    }

    /// Terminal background color.
    pub fn background(mut self, v: impl Into<String>) -> Self {
        self.background = Some(v.into());
        self
    }

    /// Foreground color for selected text (`"none"` = reverse video).
    pub fn selection_foreground(mut self, v: impl Into<String>) -> Self {
        self.selection_foreground = Some(v.into());
        self
    }

    /// Background color for selected text.
    pub fn selection_background(mut self, v: impl Into<String>) -> Self {
        self.selection_background = Some(v.into());
        self
    }

    /// Color used to draw URLs when hovering.
    pub fn url_color(mut self, v: impl Into<String>) -> Self {
        self.url_color = Some(v.into());
        self
    }

    /// Color for the visual bell flash.
    pub fn visual_bell_color(mut self, v: impl Into<String>) -> Self {
        self.visual_bell_color = Some(v.into());
        self
    }

    /// Set a single ANSI palette entry (index 0–15).
    ///
    /// # Panics
    /// Panics if `index > 15`.
    pub fn ansi_color(mut self, index: usize, v: impl Into<String>) -> Self {
        assert!(index < 16, "ANSI color index must be 0–15");
        self.ansi[index] = Some(v.into());
        self
    }

    /// Convenience setter for color0 (black).
    pub fn color0(mut self, v: impl Into<String>) -> Self {
        self.ansi[0] = Some(v.into());
        self
    }
    /// Convenience setter for color1 (red).
    pub fn color1(mut self, v: impl Into<String>) -> Self {
        self.ansi[1] = Some(v.into());
        self
    }
    /// Convenience setter for color2 (green).
    pub fn color2(mut self, v: impl Into<String>) -> Self {
        self.ansi[2] = Some(v.into());
        self
    }
    /// Convenience setter for color3 (yellow).
    pub fn color3(mut self, v: impl Into<String>) -> Self {
        self.ansi[3] = Some(v.into());
        self
    }
    /// Convenience setter for color4 (blue).
    pub fn color4(mut self, v: impl Into<String>) -> Self {
        self.ansi[4] = Some(v.into());
        self
    }
    /// Convenience setter for color5 (magenta).
    pub fn color5(mut self, v: impl Into<String>) -> Self {
        self.ansi[5] = Some(v.into());
        self
    }
    /// Convenience setter for color6 (cyan).
    pub fn color6(mut self, v: impl Into<String>) -> Self {
        self.ansi[6] = Some(v.into());
        self
    }
    /// Convenience setter for color7 (white).
    pub fn color7(mut self, v: impl Into<String>) -> Self {
        self.ansi[7] = Some(v.into());
        self
    }
    /// Convenience setter for color8 (bright black).
    pub fn color8(mut self, v: impl Into<String>) -> Self {
        self.ansi[8] = Some(v.into());
        self
    }
    /// Convenience setter for color9 (bright red).
    pub fn color9(mut self, v: impl Into<String>) -> Self {
        self.ansi[9] = Some(v.into());
        self
    }
    /// Convenience setter for color10 (bright green).
    pub fn color10(mut self, v: impl Into<String>) -> Self {
        self.ansi[10] = Some(v.into());
        self
    }
    /// Convenience setter for color11 (bright yellow).
    pub fn color11(mut self, v: impl Into<String>) -> Self {
        self.ansi[11] = Some(v.into());
        self
    }
    /// Convenience setter for color12 (bright blue).
    pub fn color12(mut self, v: impl Into<String>) -> Self {
        self.ansi[12] = Some(v.into());
        self
    }
    /// Convenience setter for color13 (bright magenta).
    pub fn color13(mut self, v: impl Into<String>) -> Self {
        self.ansi[13] = Some(v.into());
        self
    }
    /// Convenience setter for color14 (bright cyan).
    pub fn color14(mut self, v: impl Into<String>) -> Self {
        self.ansi[14] = Some(v.into());
        self
    }
    /// Convenience setter for color15 (bright white).
    pub fn color15(mut self, v: impl Into<String>) -> Self {
        self.ansi[15] = Some(v.into());
        self
    }
}

impl KittyConfig for ColorScheme {
    fn render(&self, ctx: &KittyRenderContext) -> String {
        let indent = ctx.indent();
        let mut lines: Vec<String> = Vec::new();
        if let Some(ref v) = self.foreground {
            lines.push(format!("{}foreground {}", indent, v));
        }
        if let Some(ref v) = self.background {
            lines.push(format!("{}background {}", indent, v));
        }
        if let Some(ref v) = self.selection_foreground {
            lines.push(format!("{}selection_foreground {}", indent, v));
        }
        if let Some(ref v) = self.selection_background {
            lines.push(format!("{}selection_background {}", indent, v));
        }
        if let Some(ref v) = self.url_color {
            lines.push(format!("{}url_color {}", indent, v));
        }
        if let Some(ref v) = self.visual_bell_color {
            lines.push(format!("{}visual_bell_color {}", indent, v));
        }
        for (i, color) in self.ansi.iter().enumerate() {
            if let Some(v) = color {
                lines.push(format!("{}color{} {}", indent, i, v));
            }
        }
        lines.join("\n")
    }
}
