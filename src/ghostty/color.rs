use super::core::{GhosttyConfig, GhosttyRenderContext};

/// A validated 6-digit hex color, e.g. `#1e1e2e`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HexColor(pub String);

impl HexColor {
    /// Parse and validate a hex color string (`#rrggbb`).
    pub fn new(color: impl Into<String>) -> Result<Self, String> {
        let s = color.into();
        if Self::is_valid(&s) {
            Ok(Self(s))
        } else {
            Err(format!("invalid hex color '{}': expected #rrggbb", s))
        }
    }

    fn is_valid(s: &str) -> bool {
        s.len() == 7 && s.starts_with('#') && s[1..].chars().all(|c| c.is_ascii_hexdigit())
    }
}

impl std::fmt::Display for HexColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Ghostty theme specification — either a single name or a separate
/// dark/light pair.
///
/// ```text
/// theme = Catppuccin Mocha
/// theme = dark:tokyonight,light:gruvbox-light
/// ```
#[derive(Clone, Debug)]
pub enum ThemeSpec {
    Single(String),
    DarkLight { dark: String, light: String },
}

impl ThemeSpec {
    pub fn single(name: impl Into<String>) -> Self {
        Self::Single(name.into())
    }

    pub fn dark_light(dark: impl Into<String>, light: impl Into<String>) -> Self {
        Self::DarkLight {
            dark: dark.into(),
            light: light.into(),
        }
    }
}

impl std::fmt::Display for ThemeSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Single(name) => write!(f, "{}", name),
            Self::DarkLight { dark, light } => write!(f, "dark:{},light:{}", dark, light),
        }
    }
}

/// Color and theme configuration for Ghostty.
///
/// Renders as a group of `theme`, `background`, `foreground`,
/// `selection-background`, `selection-foreground`, and `palette` entries.
#[derive(Default, Clone, Debug)]
pub struct ColorConfig {
    pub theme: Option<ThemeSpec>,
    pub background: Option<HexColor>,
    pub foreground: Option<HexColor>,
    pub selection_background: Option<HexColor>,
    pub selection_foreground: Option<HexColor>,
    /// Minimum contrast ratio (1.0 – 21.0).
    pub minimum_contrast: Option<f64>,
    /// Per-index palette overrides: `(index 0–255, color)`.
    pub palette: Vec<(u8, HexColor)>,
}

impl ColorConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn theme(mut self, t: ThemeSpec) -> Self {
        self.theme = Some(t);
        self
    }

    pub fn background(mut self, c: HexColor) -> Self {
        self.background = Some(c);
        self
    }

    pub fn foreground(mut self, c: HexColor) -> Self {
        self.foreground = Some(c);
        self
    }

    pub fn selection_background(mut self, c: HexColor) -> Self {
        self.selection_background = Some(c);
        self
    }

    pub fn selection_foreground(mut self, c: HexColor) -> Self {
        self.selection_foreground = Some(c);
        self
    }

    pub fn minimum_contrast(mut self, ratio: f64) -> Self {
        self.minimum_contrast = Some(ratio);
        self
    }

    pub fn palette_entry(mut self, index: u8, color: HexColor) -> Self {
        self.palette.push((index, color));
        self
    }
}

impl GhosttyConfig for ColorConfig {
    fn render(&self, _ctx: &GhosttyRenderContext) -> String {
        let mut out = String::new();

        if let Some(ref t) = self.theme {
            out.push_str(&format!("theme = {}\n", t));
        }
        if let Some(ref c) = self.background {
            out.push_str(&format!("background = {}\n", c));
        }
        if let Some(ref c) = self.foreground {
            out.push_str(&format!("foreground = {}\n", c));
        }
        if let Some(ref c) = self.selection_background {
            out.push_str(&format!("selection-background = {}\n", c));
        }
        if let Some(ref c) = self.selection_foreground {
            out.push_str(&format!("selection-foreground = {}\n", c));
        }
        if let Some(ratio) = self.minimum_contrast {
            out.push_str(&format!("minimum-contrast = {}\n", ratio));
        }
        for (idx, color) in &self.palette {
            out.push_str(&format!("palette = {}={}\n", idx, color));
        }

        out
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(ratio) = self.minimum_contrast
            && !(1.0..=21.0).contains(&ratio)
        {
            return Err(format!(
                "minimum-contrast must be 1.0–21.0, got {}",
                ratio
            ));
        }
        Ok(())
    }
}
