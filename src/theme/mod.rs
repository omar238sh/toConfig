//! Unified cross-backend theme definition.
//!
//! A [`Theme`] carries a colour palette, font settings, cursor and icon
//! preferences, and optionally the GTK / Qt theme names.  Its
//! `apply_gtk` / `apply_qt` / `apply_neovim` helpers produce ready-to-use
//! config builders so you never have to repeat the same values in multiple
//! places.
//!
//! # Quick start
//! ```
//! use toconfig::theme::{Theme, ThemePalette, FontConfig, CursorConfig, IconConfig};
//!
//! let theme = Theme::new("Catppuccin Mocha")
//!     .palette(ThemePalette::catppuccin_mocha())
//!     .fonts(FontConfig::new("Noto Sans", 11, "JetBrains Mono", 11))
//!     .cursor(CursorConfig::new("Bibata-Modern-Ice", 24))
//!     .icons(IconConfig::new("Papirus-Dark"))
//!     .gtk_theme("Catppuccin-Mocha-Standard-Blue-Dark")
//!     .qt_style("kvantum")
//!     .qt_color_scheme("/usr/share/color-schemes/CatppuccinMocha.colors")
//!     .neovim_colorscheme("catppuccin");
//! ```

use crate::gtk::{GtkSettings, GtkVersion, HintStyle, RgbaMethod};
use crate::neovim::theme::ColorschemeNode;
use crate::qt::{QtConfig, QtStyle, QtVersion};

// ── ThemeColor ────────────────────────────────────────────────────────────────

/// A hex colour value used inside a [`ThemePalette`].
///
/// The leading `#` is always stored and normalised on construction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThemeColor(String);

impl ThemeColor {
    /// Create a new colour, adding a leading `#` if absent.
    pub fn new(hex: &str) -> Self {
        let stripped = hex.strip_prefix('#').unwrap_or(hex);
        Self(format!("#{}", stripped))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Return the hex digits without the leading `#`.
    pub fn hex(&self) -> &str {
        &self.0[1..]
    }
}

impl std::fmt::Display for ThemeColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ── ThemePalette ──────────────────────────────────────────────────────────────

/// A full colour palette for a theme.
///
/// Follows a Catppuccin-inspired naming scheme but is generic enough for any
/// design system.  The built-in presets ([`catppuccin_mocha`], [`nord`],
/// [`gruvbox_dark`]) give you a quick starting point.
///
/// [`catppuccin_mocha`]: ThemePalette::catppuccin_mocha
/// [`nord`]: ThemePalette::nord
/// [`gruvbox_dark`]: ThemePalette::gruvbox_dark
#[derive(Debug, Clone)]
pub struct ThemePalette {
    /// Darkest background.
    pub base: ThemeColor,
    /// Slightly lighter background (sidebars, panels).
    pub mantle: ThemeColor,
    /// Even lighter background (status bars, tab bars).
    pub crust: ThemeColor,
    pub surface0: ThemeColor,
    pub surface1: ThemeColor,
    pub surface2: ThemeColor,
    /// Muted UI elements.
    pub overlay0: ThemeColor,
    pub overlay1: ThemeColor,
    pub overlay2: ThemeColor,
    pub subtext0: ThemeColor,
    pub subtext1: ThemeColor,
    /// Primary foreground text.
    pub text: ThemeColor,
    /// Primary accent colour.
    pub accent: ThemeColor,
    // Semantic colours
    pub red: ThemeColor,
    pub orange: ThemeColor,
    pub yellow: ThemeColor,
    pub green: ThemeColor,
    pub teal: ThemeColor,
    pub blue: ThemeColor,
    pub mauve: ThemeColor,
    pub pink: ThemeColor,
}

impl ThemePalette {
    /// [Catppuccin Mocha](https://github.com/catppuccin/catppuccin) — dark.
    pub fn catppuccin_mocha() -> Self {
        Self {
            base:     ThemeColor::new("#1e1e2e"),
            mantle:   ThemeColor::new("#181825"),
            crust:    ThemeColor::new("#11111b"),
            surface0: ThemeColor::new("#313244"),
            surface1: ThemeColor::new("#45475a"),
            surface2: ThemeColor::new("#585b70"),
            overlay0: ThemeColor::new("#6c7086"),
            overlay1: ThemeColor::new("#7f849c"),
            overlay2: ThemeColor::new("#9399b2"),
            subtext0: ThemeColor::new("#a6adc8"),
            subtext1: ThemeColor::new("#bac2de"),
            text:     ThemeColor::new("#cdd6f4"),
            accent:   ThemeColor::new("#89b4fa"),
            red:      ThemeColor::new("#f38ba8"),
            orange:   ThemeColor::new("#fab387"),
            yellow:   ThemeColor::new("#f9e2af"),
            green:    ThemeColor::new("#a6e3a1"),
            teal:     ThemeColor::new("#94e2d5"),
            blue:     ThemeColor::new("#89b4fa"),
            mauve:    ThemeColor::new("#cba6f7"),
            pink:     ThemeColor::new("#f5c2e7"),
        }
    }

    /// [Nord](https://www.nordtheme.com/) palette.
    pub fn nord() -> Self {
        Self {
            base:     ThemeColor::new("#2e3440"),
            mantle:   ThemeColor::new("#272c36"),
            crust:    ThemeColor::new("#1e222a"),
            surface0: ThemeColor::new("#3b4252"),
            surface1: ThemeColor::new("#434c5e"),
            surface2: ThemeColor::new("#4c566a"),
            overlay0: ThemeColor::new("#5e6779"),
            overlay1: ThemeColor::new("#6e7c8e"),
            overlay2: ThemeColor::new("#808fa3"),
            subtext0: ThemeColor::new("#8fbcbb"),
            subtext1: ThemeColor::new("#88c0d0"),
            text:     ThemeColor::new("#eceff4"),
            accent:   ThemeColor::new("#88c0d0"),
            red:      ThemeColor::new("#bf616a"),
            orange:   ThemeColor::new("#d08770"),
            yellow:   ThemeColor::new("#ebcb8b"),
            green:    ThemeColor::new("#a3be8c"),
            teal:     ThemeColor::new("#8fbcbb"),
            blue:     ThemeColor::new("#81a1c1"),
            mauve:    ThemeColor::new("#b48ead"),
            pink:     ThemeColor::new("#b48ead"),
        }
    }

    /// [Gruvbox Dark](https://github.com/morhetz/gruvbox) palette.
    pub fn gruvbox_dark() -> Self {
        Self {
            base:     ThemeColor::new("#282828"),
            mantle:   ThemeColor::new("#1d2021"),
            crust:    ThemeColor::new("#141617"),
            surface0: ThemeColor::new("#3c3836"),
            surface1: ThemeColor::new("#504945"),
            surface2: ThemeColor::new("#665c54"),
            overlay0: ThemeColor::new("#7c6f64"),
            overlay1: ThemeColor::new("#928374"),
            overlay2: ThemeColor::new("#a89984"),
            subtext0: ThemeColor::new("#bdae93"),
            subtext1: ThemeColor::new("#d5c4a1"),
            text:     ThemeColor::new("#ebdbb2"),
            accent:   ThemeColor::new("#83a598"),
            red:      ThemeColor::new("#cc241d"),
            orange:   ThemeColor::new("#d65d0e"),
            yellow:   ThemeColor::new("#d79921"),
            green:    ThemeColor::new("#98971a"),
            teal:     ThemeColor::new("#689d6a"),
            blue:     ThemeColor::new("#458588"),
            mauve:    ThemeColor::new("#b16286"),
            pink:     ThemeColor::new("#d3869b"),
        }
    }
}

// ── FontConfig ────────────────────────────────────────────────────────────────

/// Font settings used across all backends.
#[derive(Debug, Clone)]
pub struct FontConfig {
    /// Proportional (UI / sans-serif) font family, e.g. `"Noto Sans"`.
    pub family: String,
    /// Proportional font size in points.
    pub size: u32,
    /// Monospace font family, e.g. `"JetBrains Mono"`.
    pub mono_family: String,
    /// Monospace font size in points.
    pub mono_size: u32,
}

impl FontConfig {
    pub fn new(family: &str, size: u32, mono_family: &str, mono_size: u32) -> Self {
        Self {
            family: family.to_string(),
            size,
            mono_family: mono_family.to_string(),
            mono_size,
        }
    }

    /// GTK font spec, e.g. `"Noto Sans 11"`.
    pub fn gtk_spec(&self) -> String {
        format!("{} {}", self.family, self.size)
    }

    /// GTK monospace font spec, e.g. `"JetBrains Mono 11"`.
    pub fn gtk_mono_spec(&self) -> String {
        format!("{} {}", self.mono_family, self.mono_size)
    }

    /// Qt (qt5ct CSV) general font spec, e.g. `"Noto Sans, 11"`.
    pub fn qt_spec(&self) -> String {
        format!("{}, {}", self.family, self.size)
    }

    /// Qt (qt5ct CSV) monospace font spec, e.g. `"JetBrains Mono, 11"`.
    pub fn qt_mono_spec(&self) -> String {
        format!("{}, {}", self.mono_family, self.mono_size)
    }
}

// ── CursorConfig & IconConfig ─────────────────────────────────────────────────

/// Cursor theme settings.
#[derive(Debug, Clone)]
pub struct CursorConfig {
    pub name: String,
    pub size: u32,
}

impl CursorConfig {
    pub fn new(name: &str, size: u32) -> Self {
        Self {
            name: name.to_string(),
            size,
        }
    }
}

/// Icon theme settings.
#[derive(Debug, Clone)]
pub struct IconConfig {
    pub name: String,
}

impl IconConfig {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

// ── Theme ─────────────────────────────────────────────────────────────────────

/// Unified theme definition spanning every supported backend.
///
/// Use the `apply_*` helpers to produce pre-filled backend-specific builders
/// that you can further customise before rendering.
///
/// # Example
/// ```
/// use toconfig::theme::{Theme, ThemePalette, FontConfig, CursorConfig, IconConfig};
/// use toconfig::gtk::GtkVersion;
/// use toconfig::qt::QtVersion;
///
/// let theme = Theme::new("Catppuccin Mocha")
///     .palette(ThemePalette::catppuccin_mocha())
///     .fonts(FontConfig::new("Noto Sans", 11, "JetBrains Mono", 11))
///     .cursor(CursorConfig::new("Bibata-Modern-Ice", 24))
///     .icons(IconConfig::new("Papirus-Dark"))
///     .gtk_theme("Catppuccin-Mocha-Standard-Blue-Dark")
///     .qt_style("kvantum")
///     .qt_color_scheme("/usr/share/color-schemes/CatppuccinMocha.colors")
///     .neovim_colorscheme("catppuccin");
///
/// // Produce backend builders
/// let gtk3 = theme.apply_gtk(GtkVersion::Gtk3);
/// let qt5  = theme.apply_qt(QtVersion::Qt5);
/// let nvim = theme.apply_neovim();
/// ```
#[derive(Debug, Clone)]
pub struct Theme {
    /// Human-readable theme name.
    pub name: String,
    pub palette: Option<ThemePalette>,
    pub fonts: Option<FontConfig>,
    pub cursor: Option<CursorConfig>,
    pub icons: Option<IconConfig>,
    /// GTK theme name passed to `gtk-theme-name`.
    pub gtk_theme_name: Option<String>,
    /// Qt widget style name (e.g. `"kvantum"`, `"Breeze"`).
    pub qt_style_name: Option<String>,
    /// Path to a KDE / Qt `.colors` file.
    pub qt_color_scheme: Option<String>,
    /// Neovim colorscheme name (passed to `vim.cmd.colorscheme`).
    pub nvim_colorscheme: Option<String>,
}

impl Theme {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            palette: None,
            fonts: None,
            cursor: None,
            icons: None,
            gtk_theme_name: None,
            qt_style_name: None,
            qt_color_scheme: None,
            nvim_colorscheme: None,
        }
    }

    pub fn palette(mut self, p: ThemePalette) -> Self {
        self.palette = Some(p);
        self
    }

    pub fn fonts(mut self, f: FontConfig) -> Self {
        self.fonts = Some(f);
        self
    }

    pub fn cursor(mut self, c: CursorConfig) -> Self {
        self.cursor = Some(c);
        self
    }

    pub fn icons(mut self, i: IconConfig) -> Self {
        self.icons = Some(i);
        self
    }

    pub fn gtk_theme(mut self, name: &str) -> Self {
        self.gtk_theme_name = Some(name.to_string());
        self
    }

    pub fn qt_style(mut self, style: &str) -> Self {
        self.qt_style_name = Some(style.to_string());
        self
    }

    pub fn qt_color_scheme(mut self, path: &str) -> Self {
        self.qt_color_scheme = Some(path.to_string());
        self
    }

    pub fn neovim_colorscheme(mut self, name: &str) -> Self {
        self.nvim_colorscheme = Some(name.to_string());
        self
    }

    // ── Backend helpers ───────────────────────────────────────────────────────

    /// Produce a [`GtkSettings`] builder pre-filled from this theme.
    ///
    /// All returned fields can be further overridden with the builder's own
    /// methods before calling `generate()`.
    pub fn apply_gtk(&self, version: GtkVersion) -> GtkSettings {
        let mut s = GtkSettings::new(version);

        if let Some(ref n) = self.gtk_theme_name {
            s = s.theme(n);
        }
        if let Some(ref icons) = self.icons {
            s = s.icon_theme(&icons.name);
        }
        if let Some(ref cursor) = self.cursor {
            s = s.cursor_theme(&cursor.name).cursor_size(cursor.size);
        }
        if let Some(ref fonts) = self.fonts {
            s = s.font(&fonts.gtk_spec());
        }
        s.prefer_dark(true)
            .antialias(true)
            .hinting(true)
            .hint_style(HintStyle::Full)
            .rgba(RgbaMethod::Rgb)
    }

    /// Produce a [`QtConfig`] builder pre-filled from this theme.
    pub fn apply_qt(&self, version: QtVersion) -> QtConfig {
        let mut q = QtConfig::new(version);

        if let Some(ref style) = self.qt_style_name {
            q = q.style(QtStyle::Custom(style.clone()));
        }
        if let Some(ref path) = self.qt_color_scheme {
            q = q.color_scheme(path);
        }
        if let Some(ref fonts) = self.fonts {
            q = q
                .font_general(&fonts.qt_spec())
                .font_fixed(&fonts.qt_mono_spec());
        }
        if let Some(ref icons) = self.icons {
            q = q.icon_theme(&icons.name);
        }
        q
    }

    /// Produce a [`ColorschemeNode`] for Neovim from the theme's
    /// `neovim_colorscheme` field.
    ///
    /// Returns `None` when no Neovim colorscheme name has been set.
    pub fn apply_neovim(&self) -> Option<ColorschemeNode> {
        self.nvim_colorscheme
            .as_deref()
            .map(ColorschemeNode::new)
    }
}
