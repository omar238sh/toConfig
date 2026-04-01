use super::{KittyConfig, KittyRenderContext};

/// Font configuration for kitty terminal.
///
/// Renders as a block of `key value` lines for font-related kitty settings.
///
/// # Example
/// ```
/// use toconfig::kitty::KittyConfig;
/// use toconfig::kitty::font::FontConfig;
/// let f = FontConfig::new()
///     .family("Hack Nerd Font Mono")
///     .size(13.0)
///     .bold("auto")
///     .italic("auto")
///     .bold_italic("auto");
/// let out = f.generate();
/// assert!(out.contains("font_family"));
/// assert!(out.contains("font_size"));
/// ```
#[derive(Default)]
pub struct FontConfig {
    /// Primary font family name.
    pub family: Option<String>,
    /// Bold font variant; `"auto"` to derive from regular.
    pub bold: Option<String>,
    /// Italic font variant; `"auto"` to derive from regular.
    pub italic: Option<String>,
    /// Bold-italic font variant; `"auto"` to derive from regular.
    pub bold_italic: Option<String>,
    /// Font size in points (e.g. `13.0`).
    pub size: Option<f32>,
    /// Disable ligatures: `"never"`, `"always"`, `"cursor"`.
    pub disable_ligatures: Option<String>,
    /// Extra font features as a raw string (forwarded to freetype/harfbuzz).
    pub features: Option<String>,
    /// Adjust glyph cell width (percentage or pixels, e.g. `"100"` or `"+2"`).
    pub cell_width: Option<String>,
    /// Adjust glyph cell height (percentage or pixels).
    pub cell_height: Option<String>,
    /// Modify baseline position of text (`"0"` = default).
    pub baseline: Option<String>,
    /// Underline thickness in pixels.
    pub underline_thickness: Option<u32>,
    /// Underline position relative to baseline.
    pub underline_position: Option<i32>,
    /// Strikethrough position relative to baseline.
    pub strikethrough_position: Option<i32>,
    /// Strikethrough thickness in pixels.
    pub strikethrough_thickness: Option<u32>,
}

impl FontConfig {
    pub fn new() -> Self {
        Self::default()
    }

    /// Primary font family name.
    pub fn family(mut self, v: impl Into<String>) -> Self {
        self.family = Some(v.into());
        self
    }

    /// Bold font variant (`"auto"` to derive automatically).
    pub fn bold(mut self, v: impl Into<String>) -> Self {
        self.bold = Some(v.into());
        self
    }

    /// Italic font variant (`"auto"` to derive automatically).
    pub fn italic(mut self, v: impl Into<String>) -> Self {
        self.italic = Some(v.into());
        self
    }

    /// Bold-italic font variant (`"auto"` to derive automatically).
    pub fn bold_italic(mut self, v: impl Into<String>) -> Self {
        self.bold_italic = Some(v.into());
        self
    }

    /// Font size in points.
    pub fn size(mut self, v: f32) -> Self {
        self.size = Some(v);
        self
    }

    /// Disable ligatures: `"never"` | `"always"` | `"cursor"`.
    pub fn disable_ligatures(mut self, v: impl Into<String>) -> Self {
        self.disable_ligatures = Some(v.into());
        self
    }

    /// Additional font features string (harfbuzz feature tags).
    pub fn features(mut self, v: impl Into<String>) -> Self {
        self.features = Some(v.into());
        self
    }

    /// Adjust glyph cell width.
    pub fn cell_width(mut self, v: impl Into<String>) -> Self {
        self.cell_width = Some(v.into());
        self
    }

    /// Adjust glyph cell height.
    pub fn cell_height(mut self, v: impl Into<String>) -> Self {
        self.cell_height = Some(v.into());
        self
    }

    /// Modify baseline position (`"0"` = default).
    pub fn baseline(mut self, v: impl Into<String>) -> Self {
        self.baseline = Some(v.into());
        self
    }

    /// Underline thickness in pixels.
    pub fn underline_thickness(mut self, v: u32) -> Self {
        self.underline_thickness = Some(v);
        self
    }

    /// Underline position relative to the baseline.
    pub fn underline_position(mut self, v: i32) -> Self {
        self.underline_position = Some(v);
        self
    }

    /// Strikethrough position relative to the baseline.
    pub fn strikethrough_position(mut self, v: i32) -> Self {
        self.strikethrough_position = Some(v);
        self
    }

    /// Strikethrough thickness in pixels.
    pub fn strikethrough_thickness(mut self, v: u32) -> Self {
        self.strikethrough_thickness = Some(v);
        self
    }
}

impl KittyConfig for FontConfig {
    fn render(&self, ctx: &KittyRenderContext) -> String {
        let indent = ctx.indent();
        let mut lines: Vec<String> = Vec::new();
        if let Some(ref v) = self.family {
            lines.push(format!("{}font_family      {}", indent, v));
        }
        if let Some(ref v) = self.bold {
            lines.push(format!("{}bold_font        {}", indent, v));
        }
        if let Some(ref v) = self.italic {
            lines.push(format!("{}italic_font      {}", indent, v));
        }
        if let Some(ref v) = self.bold_italic {
            lines.push(format!("{}bold_italic_font {}", indent, v));
        }
        if let Some(v) = self.size {
            lines.push(format!("{}font_size        {}", indent, v));
        }
        if let Some(ref v) = self.disable_ligatures {
            lines.push(format!("{}disable_ligatures {}", indent, v));
        }
        if let Some(ref v) = self.features {
            lines.push(format!("{}font_features {}", indent, v));
        }
        if let Some(ref v) = self.cell_width {
            lines.push(format!("{}adjust_cell_width {}", indent, v));
        }
        if let Some(ref v) = self.cell_height {
            lines.push(format!("{}adjust_cell_height {}", indent, v));
        }
        if let Some(ref v) = self.baseline {
            lines.push(format!("{}adjust_baseline {}", indent, v));
        }
        if let Some(v) = self.underline_thickness {
            lines.push(format!("{}underline_thickness {}", indent, v));
        }
        if let Some(v) = self.underline_position {
            lines.push(format!("{}underline_position {}", indent, v));
        }
        if let Some(v) = self.strikethrough_position {
            lines.push(format!("{}strikethrough_position {}", indent, v));
        }
        if let Some(v) = self.strikethrough_thickness {
            lines.push(format!("{}strikethrough_thickness {}", indent, v));
        }
        lines.join("\n")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(s) = self.size {
            if s <= 0.0 {
                return Err(format!("FontConfig: font_size must be positive, got {}", s));
            }
        }
        Ok(())
    }
}
