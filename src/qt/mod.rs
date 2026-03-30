//! Qt 5 and Qt 6 configuration generators.
//!
//! Targets `~/.config/qt5ct/qt5ct.conf` and `~/.config/qt6ct/qt6ct.conf`.
//!
//! # Quick start
//! ```
//! use toconfig::qt::{QtConfig, QtVersion, QtStyle};
//! use toconfig::ini::IniConfig;
//!
//! let cfg = QtConfig::new(QtVersion::Qt5)
//!     .style(QtStyle::Kvantum)
//!     .color_scheme("/usr/share/color-schemes/CatppuccinMocha.colors")
//!     .font_general("Noto Sans, 11")
//!     .font_fixed("JetBrains Mono, 11");
//!
//! let out = cfg.generate();
//! assert!(out.contains("[Appearance]"));
//! ```

use crate::ini::{IniConfig, IniFile, IniRenderContext, IniSection};

// ── Version ──────────────────────────────────────────────────────────────────

/// Qt major version selector.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QtVersion {
    Qt5,
    Qt6,
}

impl QtVersion {
    /// Name of the ct configuration tool (`"qt5ct"` / `"qt6ct"`).
    pub fn ct_name(self) -> &'static str {
        match self {
            QtVersion::Qt5 => "qt5ct",
            QtVersion::Qt6 => "qt6ct",
        }
    }
}

// ── Style ─────────────────────────────────────────────────────────────────────

/// Qt widget style.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QtStyle {
    Breeze,
    Fusion,
    Kvantum,
    /// Any style name not covered by the variants above.
    Custom(String),
}

impl std::fmt::Display for QtStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QtStyle::Breeze => write!(f, "Breeze"),
            QtStyle::Fusion => write!(f, "Fusion"),
            QtStyle::Kvantum => write!(f, "kvantum"),
            QtStyle::Custom(s) => write!(f, "{}", s),
        }
    }
}

// ── Standard dialogs ──────────────────────────────────────────────────────────

/// Standard-dialog implementation for qt5ct / qt6ct.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StandardDialogs {
    Default,
    Xdgdesktopportal,
}

impl std::fmt::Display for StandardDialogs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StandardDialogs::Default => write!(f, "default"),
            StandardDialogs::Xdgdesktopportal => write!(f, "xdgdesktopportal"),
        }
    }
}

// ── QtConfig ──────────────────────────────────────────────────────────────────

/// Qt configuration builder for qt5ct and qt6ct.
///
/// Every field is optional; only fields that are `Some` (or `true` for
/// `custom_palette`) are emitted.  Call
/// [`generate`](IniConfig::generate) to produce the INI text.
#[derive(Debug, Clone)]
pub struct QtConfig {
    /// Target Qt version.
    pub version: QtVersion,
    pub style: Option<QtStyle>,
    /// Path to a KDE `.colors` file used as the colour scheme.
    pub color_scheme_path: Option<String>,
    pub custom_palette: bool,
    pub standard_dialogs: Option<StandardDialogs>,
    /// Proportional font spec in qt5ct CSV form, e.g. `"Noto Sans, 11"`.
    pub font_general: Option<String>,
    /// Monospace font spec in qt5ct CSV form, e.g. `"JetBrains Mono, 11"`.
    pub font_fixed: Option<String>,
    pub icon_theme: Option<String>,
    pub double_click_interval: Option<u32>,
    pub wheel_scroll_lines: Option<u32>,
}

impl QtConfig {
    pub fn new(version: QtVersion) -> Self {
        Self {
            version,
            style: None,
            color_scheme_path: None,
            custom_palette: false,
            standard_dialogs: None,
            font_general: None,
            font_fixed: None,
            icon_theme: None,
            double_click_interval: None,
            wheel_scroll_lines: None,
        }
    }

    pub fn style(mut self, s: QtStyle) -> Self {
        self.style = Some(s);
        self
    }

    pub fn color_scheme(mut self, path: &str) -> Self {
        self.color_scheme_path = Some(path.to_string());
        self
    }

    pub fn custom_palette(mut self, v: bool) -> Self {
        self.custom_palette = v;
        self
    }

    pub fn standard_dialogs(mut self, d: StandardDialogs) -> Self {
        self.standard_dialogs = Some(d);
        self
    }

    pub fn font_general(mut self, spec: &str) -> Self {
        self.font_general = Some(spec.to_string());
        self
    }

    pub fn font_fixed(mut self, spec: &str) -> Self {
        self.font_fixed = Some(spec.to_string());
        self
    }

    pub fn icon_theme(mut self, name: &str) -> Self {
        self.icon_theme = Some(name.to_string());
        self
    }

    pub fn double_click_interval(mut self, ms: u32) -> Self {
        self.double_click_interval = Some(ms);
        self
    }

    pub fn wheel_scroll_lines(mut self, lines: u32) -> Self {
        self.wheel_scroll_lines = Some(lines);
        self
    }
}

impl IniConfig for QtConfig {
    fn render(&self, ctx: &IniRenderContext) -> String {
        // [Appearance] section
        let mut appearance = IniSection::new("Appearance");
        if let Some(ref s) = self.style {
            appearance = appearance.set("style", s);
        }
        if let Some(ref p) = self.color_scheme_path {
            appearance = appearance.set("color_scheme_path", p);
        }
        appearance = appearance.set("custom_palette", self.custom_palette);
        if let Some(d) = self.standard_dialogs {
            appearance = appearance.set("standard_dialogs", d);
        }
        if let Some(ref t) = self.icon_theme {
            appearance = appearance.set("icon_theme", t);
        }

        // [Fonts] section
        let mut fonts = IniSection::new("Fonts");
        if let Some(ref f) = self.font_general {
            fonts = fonts.set("general", f);
        }
        if let Some(ref f) = self.font_fixed {
            fonts = fonts.set("fixed", f);
        }

        // [Interface] section (emitted only when any field is set)
        let mut iface = IniSection::new("Interface");
        let mut has_iface = false;
        if let Some(v) = self.double_click_interval {
            iface = iface.set("double_click_interval", v);
            has_iface = true;
        }
        if let Some(v) = self.wheel_scroll_lines {
            iface = iface.set("wheel_scroll_lines", v);
            has_iface = true;
        }

        let mut file = IniFile::new().section(appearance).section(fonts);
        if has_iface {
            file = file.section(iface);
        }
        file.render(ctx)
    }
}
