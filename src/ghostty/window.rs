use super::core::{GhosttyConfig, GhosttyRenderContext};

/// Whether the OS window decoration (title bar / border) is enabled.
#[derive(Clone, Debug)]
pub enum WindowDecoration {
    /// Let Ghostty decide (default).
    Auto,
    /// No window decoration.
    None,
    /// Client-side decorations (drawn by Ghostty).
    Client,
    /// Server-side decorations (drawn by the compositor/OS).
    Server,
}

impl std::fmt::Display for WindowDecoration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Auto => "auto",
            Self::None => "none",
            Self::Client => "client",
            Self::Server => "server",
        };
        write!(f, "{}", s)
    }
}

/// Controls whether Ghostty restores the previous window size on launch.
#[derive(Clone, Debug)]
pub enum WindowSaveState {
    Default,
    Never,
    Always,
}

impl std::fmt::Display for WindowSaveState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Default => "default",
            Self::Never => "never",
            Self::Always => "always",
        };
        write!(f, "{}", s)
    }
}

/// The color used to fill the padding area around the terminal grid.
#[derive(Clone, Debug)]
pub enum PaddingColor {
    /// Match the terminal background color (default).
    Background,
    /// Extend the terminal's edge colors into the padding.
    Extend,
    /// Extend, but only in the block direction (top/bottom).
    ExtendAlways,
}

impl std::fmt::Display for PaddingColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Background => "background",
            Self::Extend => "extend",
            Self::ExtendAlways => "extend-always",
        };
        write!(f, "{}", s)
    }
}

/// Color space used for rendering.
#[derive(Clone, Debug)]
pub enum Colorspace {
    Srgb,
    DisplayP3,
}

impl std::fmt::Display for Colorspace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Srgb => "srgb",
            Self::DisplayP3 => "display-p3",
        };
        write!(f, "{}", s)
    }
}

/// Window configuration for Ghostty.
///
/// Renders as `window-*` key-value entries.
#[derive(Default, Clone, Debug)]
pub struct WindowConfig {
    /// Initial window width in columns.
    pub width: Option<u32>,
    /// Initial window height in rows.
    pub height: Option<u32>,
    pub title: Option<String>,
    pub decoration: Option<WindowDecoration>,
    /// Horizontal padding in pixels.
    pub padding_x: Option<u32>,
    /// Vertical padding in pixels.
    pub padding_y: Option<u32>,
    /// Balance padding so the terminal is centered in the window.
    pub padding_balance: Option<bool>,
    pub padding_color: Option<PaddingColor>,
    pub colorspace: Option<Colorspace>,
    pub save_state: Option<WindowSaveState>,
    /// Position the new-tab button at the start of the tab bar.
    pub new_tab_position: Option<String>,
    pub inherit_working_directory: Option<bool>,
    pub inherit_font_size: Option<bool>,
    /// Initial window position: `(x, y)` in pixels from top-left.
    pub initial_position: Option<(i32, i32)>,
    /// Window background opacity (0.0 – 1.0).
    pub opacity: Option<f64>,
    /// Blur radius for transparent windows (0 = disabled).
    pub background_blur: Option<u32>,
}

impl WindowConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn width(mut self, cols: u32) -> Self {
        self.width = Some(cols);
        self
    }

    pub fn height(mut self, rows: u32) -> Self {
        self.height = Some(rows);
        self
    }

    pub fn title(mut self, t: impl Into<String>) -> Self {
        self.title = Some(t.into());
        self
    }

    pub fn decoration(mut self, d: WindowDecoration) -> Self {
        self.decoration = Some(d);
        self
    }

    pub fn padding_x(mut self, px: u32) -> Self {
        self.padding_x = Some(px);
        self
    }

    pub fn padding_y(mut self, px: u32) -> Self {
        self.padding_y = Some(px);
        self
    }

    pub fn padding_balance(mut self, v: bool) -> Self {
        self.padding_balance = Some(v);
        self
    }

    pub fn padding_color(mut self, c: PaddingColor) -> Self {
        self.padding_color = Some(c);
        self
    }

    pub fn colorspace(mut self, cs: Colorspace) -> Self {
        self.colorspace = Some(cs);
        self
    }

    pub fn save_state(mut self, s: WindowSaveState) -> Self {
        self.save_state = Some(s);
        self
    }

    pub fn new_tab_position(mut self, pos: impl Into<String>) -> Self {
        self.new_tab_position = Some(pos.into());
        self
    }

    pub fn inherit_working_directory(mut self, v: bool) -> Self {
        self.inherit_working_directory = Some(v);
        self
    }

    pub fn inherit_font_size(mut self, v: bool) -> Self {
        self.inherit_font_size = Some(v);
        self
    }

    pub fn initial_position(mut self, x: i32, y: i32) -> Self {
        self.initial_position = Some((x, y));
        self
    }

    pub fn opacity(mut self, v: f64) -> Self {
        self.opacity = Some(v);
        self
    }

    pub fn background_blur(mut self, radius: u32) -> Self {
        self.background_blur = Some(radius);
        self
    }
}

impl GhosttyConfig for WindowConfig {
    fn render(&self, _ctx: &GhosttyRenderContext) -> String {
        let mut out = String::new();

        if let Some(v) = self.width {
            out.push_str(&format!("window-width = {}\n", v));
        }
        if let Some(v) = self.height {
            out.push_str(&format!("window-height = {}\n", v));
        }
        if let Some(ref v) = self.title {
            out.push_str(&format!("title = {}\n", v));
        }
        if let Some(ref v) = self.decoration {
            out.push_str(&format!("window-decoration = {}\n", v));
        }
        if let Some(v) = self.padding_x {
            out.push_str(&format!("window-padding-x = {}\n", v));
        }
        if let Some(v) = self.padding_y {
            out.push_str(&format!("window-padding-y = {}\n", v));
        }
        if let Some(v) = self.padding_balance {
            out.push_str(&format!("window-padding-balance = {}\n", v));
        }
        if let Some(ref v) = self.padding_color {
            out.push_str(&format!("window-padding-color = {}\n", v));
        }
        if let Some(ref v) = self.colorspace {
            out.push_str(&format!("window-colorspace = {}\n", v));
        }
        if let Some(ref v) = self.save_state {
            out.push_str(&format!("window-save-state = {}\n", v));
        }
        if let Some(ref v) = self.new_tab_position {
            out.push_str(&format!("window-new-tab-position = {}\n", v));
        }
        if let Some(v) = self.inherit_working_directory {
            out.push_str(&format!("window-inherit-working-directory = {}\n", v));
        }
        if let Some(v) = self.inherit_font_size {
            out.push_str(&format!("window-inherit-font-size = {}\n", v));
        }
        if let Some((x, y)) = self.initial_position {
            out.push_str(&format!("window-initial-position = {},{}\n", x, y));
        }
        if let Some(v) = self.opacity {
            out.push_str(&format!("background-opacity = {}\n", v));
        }
        if let Some(v) = self.background_blur {
            out.push_str(&format!("background-blur-radius = {}\n", v));
        }

        out
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(v) = self.opacity
            && !(0.0..=1.0).contains(&v)
        {
            return Err(format!("background-opacity must be 0.0–1.0, got {}", v));
        }
        Ok(())
    }
}
