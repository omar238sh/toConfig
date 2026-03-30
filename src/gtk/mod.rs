//! GTK 3 and GTK 4 configuration generators.
//!
//! Renders to `~/.config/gtk-{3,4}.0/settings.ini` (INI format) and
//! optionally a `gtk.css` snippet for per-application overrides.
//!
//! # Quick start
//! ```
//! use toconfig::gtk::{GtkSettings, GtkVersion, HintStyle, RgbaMethod};
//! use toconfig::ini::IniConfig;
//!
//! let cfg = GtkSettings::new(GtkVersion::Gtk3)
//!     .theme("Catppuccin-Mocha-Standard-Blue-Dark")
//!     .icon_theme("Papirus-Dark")
//!     .cursor_theme("Bibata-Modern-Ice")
//!     .cursor_size(24)
//!     .font("JetBrains Mono 11")
//!     .prefer_dark(true)
//!     .antialias(true)
//!     .hint_style(HintStyle::Full)
//!     .rgba(RgbaMethod::Rgb);
//!
//! let out = cfg.generate();
//! assert!(out.contains("[Settings]"));
//! ```

use crate::ini::{IniConfig, IniFile, IniRenderContext, IniSection};

// ── Version ──────────────────────────────────────────────────────────────────

/// GTK major version selector.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GtkVersion {
    Gtk3,
    Gtk4,
}

impl GtkVersion {
    /// Config directory name for this GTK version (e.g. `"gtk-3.0"`).
    pub fn config_dir(self) -> &'static str {
        match self {
            GtkVersion::Gtk3 => "gtk-3.0",
            GtkVersion::Gtk4 => "gtk-4.0",
        }
    }
}

// ── Font rendering options ────────────────────────────────────────────────────

/// Font hinting style for GTK Xft settings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HintStyle {
    None,
    Slight,
    Medium,
    Full,
}

impl std::fmt::Display for HintStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HintStyle::None => write!(f, "hintnone"),
            HintStyle::Slight => write!(f, "hintslight"),
            HintStyle::Medium => write!(f, "hintmedium"),
            HintStyle::Full => write!(f, "hintfull"),
        }
    }
}

/// Subpixel rendering method for GTK Xft settings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RgbaMethod {
    None,
    Rgb,
    Bgr,
    Vrgb,
    Vbgr,
}

impl std::fmt::Display for RgbaMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RgbaMethod::None => write!(f, "none"),
            RgbaMethod::Rgb => write!(f, "rgb"),
            RgbaMethod::Bgr => write!(f, "bgr"),
            RgbaMethod::Vrgb => write!(f, "vrgb"),
            RgbaMethod::Vbgr => write!(f, "vbgr"),
        }
    }
}

// ── GtkSettings ──────────────────────────────────────────────────────────────

/// GTK `settings.ini` configuration builder.
///
/// Every field is optional; only fields that are `Some` are emitted.
/// Call [`generate`](IniConfig::generate) or
/// [`render`](IniConfig::render) to produce the INI text.
#[derive(Debug, Clone, Default)]
pub struct GtkSettings {
    /// Target GTK version (required for [`validate`](IniConfig::validate)).
    pub version: Option<GtkVersion>,
    pub theme_name: Option<String>,
    pub icon_theme: Option<String>,
    pub cursor_theme: Option<String>,
    pub cursor_size: Option<u32>,
    /// Full GTK font spec, e.g. `"Noto Sans 11"`.
    pub font: Option<String>,
    pub prefer_dark: Option<bool>,
    pub antialias: Option<bool>,
    pub hinting: Option<bool>,
    pub hint_style: Option<HintStyle>,
    pub rgba: Option<RgbaMethod>,
    pub toolbar_style: Option<String>,
    pub button_images: Option<bool>,
    pub menu_images: Option<bool>,
}

impl GtkSettings {
    pub fn new(version: GtkVersion) -> Self {
        Self {
            version: Some(version),
            ..Default::default()
        }
    }

    pub fn theme(mut self, name: &str) -> Self {
        self.theme_name = Some(name.to_string());
        self
    }

    pub fn icon_theme(mut self, name: &str) -> Self {
        self.icon_theme = Some(name.to_string());
        self
    }

    pub fn cursor_theme(mut self, name: &str) -> Self {
        self.cursor_theme = Some(name.to_string());
        self
    }

    pub fn cursor_size(mut self, size: u32) -> Self {
        self.cursor_size = Some(size);
        self
    }

    pub fn font(mut self, spec: &str) -> Self {
        self.font = Some(spec.to_string());
        self
    }

    pub fn prefer_dark(mut self, v: bool) -> Self {
        self.prefer_dark = Some(v);
        self
    }

    pub fn antialias(mut self, v: bool) -> Self {
        self.antialias = Some(v);
        self
    }

    pub fn hinting(mut self, v: bool) -> Self {
        self.hinting = Some(v);
        self
    }

    pub fn hint_style(mut self, s: HintStyle) -> Self {
        self.hint_style = Some(s);
        self
    }

    pub fn rgba(mut self, m: RgbaMethod) -> Self {
        self.rgba = Some(m);
        self
    }

    pub fn toolbar_style(mut self, s: &str) -> Self {
        self.toolbar_style = Some(s.to_string());
        self
    }

    pub fn button_images(mut self, v: bool) -> Self {
        self.button_images = Some(v);
        self
    }

    pub fn menu_images(mut self, v: bool) -> Self {
        self.menu_images = Some(v);
        self
    }
}

impl IniConfig for GtkSettings {
    fn render(&self, ctx: &IniRenderContext) -> String {
        let mut section = IniSection::new("Settings");

        if let Some(ref v) = self.theme_name {
            section = section.set("gtk-theme-name", v);
        }
        if let Some(ref v) = self.icon_theme {
            section = section.set("gtk-icon-theme-name", v);
        }
        if let Some(ref v) = self.cursor_theme {
            section = section.set("gtk-cursor-theme-name", v);
        }
        if let Some(v) = self.cursor_size {
            section = section.set("gtk-cursor-theme-size", v);
        }
        if let Some(ref v) = self.font {
            section = section.set("gtk-font-name", v);
        }
        if let Some(v) = self.prefer_dark {
            section = section.set("gtk-application-prefer-dark-theme", if v { 1 } else { 0 });
        }
        if let Some(v) = self.button_images {
            section = section.set("gtk-button-images", if v { 1 } else { 0 });
        }
        if let Some(v) = self.menu_images {
            section = section.set("gtk-menu-images", if v { 1 } else { 0 });
        }
        if let Some(ref v) = self.toolbar_style {
            section = section.set("gtk-toolbar-style", v);
        }
        if let Some(v) = self.antialias {
            section = section.set("gtk-xft-antialias", if v { 1 } else { 0 });
        }
        if let Some(v) = self.hinting {
            section = section.set("gtk-xft-hinting", if v { 1 } else { 0 });
        }
        if let Some(v) = self.hint_style {
            section = section.set("gtk-xft-hintstyle", v);
        }
        if let Some(v) = self.rgba {
            section = section.set("gtk-xft-rgba", v);
        }

        IniFile::new().section(section).render(ctx)
    }

    fn validate(&self) -> Result<(), String> {
        if self.version.is_none() {
            return Err("GtkSettings: version must be set".to_string());
        }
        Ok(())
    }
}

// ── GtkCss ───────────────────────────────────────────────────────────────────

/// Raw CSS to write as `gtk.css` alongside the settings file.
///
/// Useful for per-application colour overrides, border-radius adjustments, etc.
///
/// # Example
/// ```
/// use toconfig::gtk::GtkCss;
///
/// let css = GtkCss::new()
///     .rule("window { border-radius: 8px; }")
///     .rule("button { padding: 4px 8px; }");
///
/// assert!(css.generate().contains("border-radius"));
/// ```
#[derive(Debug, Clone, Default)]
pub struct GtkCss {
    pub rules: Vec<String>,
}

impl GtkCss {
    pub fn new() -> Self {
        Self::default()
    }

    /// Append a raw CSS rule block.
    pub fn rule(mut self, css: &str) -> Self {
        self.rules.push(css.to_string());
        self
    }

    /// Render the complete CSS file contents (not INI — just plain CSS text).
    pub fn generate(&self) -> String {
        self.rules.join("\n\n")
    }
}
