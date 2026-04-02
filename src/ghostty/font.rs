use super::core::{GhosttyConfig, GhosttyRenderContext};

/// Controls how synthetic bold/italic faces are generated when a font
/// family does not ship the requested variant.
#[derive(Clone, Debug)]
pub enum FontSyntheticStyle {
    /// Allow all synthetic styles (default).
    All,
    NoBold,
    NoItalic,
    NoBoldItalic,
    /// Disable all synthetic styles.
    False,
}

impl std::fmt::Display for FontSyntheticStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::All => "true",
            Self::NoBold => "no-bold",
            Self::NoItalic => "no-italic",
            Self::NoBoldItalic => "no-bold-italic",
            Self::False => "false",
        };
        write!(f, "{}", s)
    }
}

/// Font configuration for Ghostty.
///
/// Renders as `font-family`, `font-size`, `font-style`, `font-feature`,
/// and related key-value entries.
#[derive(Default, Clone, Debug)]
pub struct FontConfig {
    pub family: Option<String>,
    pub family_bold: Option<String>,
    pub family_italic: Option<String>,
    pub family_bold_italic: Option<String>,
    /// Point size (e.g. `13.0`).
    pub size: Option<f64>,
    pub style: Option<String>,
    pub style_bold: Option<String>,
    pub style_italic: Option<String>,
    pub style_bold_italic: Option<String>,
    /// OpenType feature tags, e.g. `"calt"`, `"-liga"`.
    pub features: Vec<String>,
    /// Variation axis overrides: `(axis-tag, value)`, e.g. `("wght", 450.0)`.
    pub variations: Vec<(String, f64)>,
    /// macOS only — render fonts with a slightly heavier weight.
    pub thicken: Option<bool>,
    pub synthetic_style: Option<FontSyntheticStyle>,
}

impl FontConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn family(mut self, v: impl Into<String>) -> Self {
        self.family = Some(v.into());
        self
    }

    pub fn family_bold(mut self, v: impl Into<String>) -> Self {
        self.family_bold = Some(v.into());
        self
    }

    pub fn family_italic(mut self, v: impl Into<String>) -> Self {
        self.family_italic = Some(v.into());
        self
    }

    pub fn family_bold_italic(mut self, v: impl Into<String>) -> Self {
        self.family_bold_italic = Some(v.into());
        self
    }

    pub fn size(mut self, pt: f64) -> Self {
        self.size = Some(pt);
        self
    }

    pub fn style(mut self, v: impl Into<String>) -> Self {
        self.style = Some(v.into());
        self
    }

    pub fn style_bold(mut self, v: impl Into<String>) -> Self {
        self.style_bold = Some(v.into());
        self
    }

    pub fn style_italic(mut self, v: impl Into<String>) -> Self {
        self.style_italic = Some(v.into());
        self
    }

    pub fn style_bold_italic(mut self, v: impl Into<String>) -> Self {
        self.style_bold_italic = Some(v.into());
        self
    }

    /// Add an OpenType feature tag, e.g. `"calt"` (enable) or `"-liga"` (disable).
    pub fn add_feature(mut self, feat: impl Into<String>) -> Self {
        self.features.push(feat.into());
        self
    }

    /// Add a font variation axis override, e.g. `("wght", 450.0)`.
    pub fn add_variation(mut self, axis: impl Into<String>, value: f64) -> Self {
        self.variations.push((axis.into(), value));
        self
    }

    pub fn thicken(mut self, v: bool) -> Self {
        self.thicken = Some(v);
        self
    }

    pub fn synthetic_style(mut self, v: FontSyntheticStyle) -> Self {
        self.synthetic_style = Some(v);
        self
    }
}

impl GhosttyConfig for FontConfig {
    fn render(&self, _ctx: &GhosttyRenderContext) -> String {
        let mut out = String::new();

        if let Some(ref v) = self.family {
            out.push_str(&format!("font-family = {}\n", v));
        }
        if let Some(ref v) = self.family_bold {
            out.push_str(&format!("font-family-bold = {}\n", v));
        }
        if let Some(ref v) = self.family_italic {
            out.push_str(&format!("font-family-italic = {}\n", v));
        }
        if let Some(ref v) = self.family_bold_italic {
            out.push_str(&format!("font-family-bold-italic = {}\n", v));
        }
        if let Some(pt) = self.size {
            out.push_str(&format!("font-size = {}\n", pt));
        }
        if let Some(ref v) = self.style {
            out.push_str(&format!("font-style = {}\n", v));
        }
        if let Some(ref v) = self.style_bold {
            out.push_str(&format!("font-style-bold = {}\n", v));
        }
        if let Some(ref v) = self.style_italic {
            out.push_str(&format!("font-style-italic = {}\n", v));
        }
        if let Some(ref v) = self.style_bold_italic {
            out.push_str(&format!("font-style-bold-italic = {}\n", v));
        }
        for feat in &self.features {
            out.push_str(&format!("font-feature = {}\n", feat));
        }
        for (axis, value) in &self.variations {
            out.push_str(&format!("font-variation = {}={}\n", axis, value));
        }
        if let Some(v) = self.thicken {
            out.push_str(&format!("font-thicken = {}\n", v));
        }
        if let Some(ref v) = self.synthetic_style {
            out.push_str(&format!("font-synthetic-style = {}\n", v));
        }

        out
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(pt) = self.size
            && pt <= 0.0
        {
            return Err(format!("font-size must be > 0, got {}", pt));
        }
        Ok(())
    }
}
