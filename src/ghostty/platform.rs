use super::core::{GhosttyConfig, GhosttyRenderContext};

// ── GTK (Linux / BSD) ────────────────────────────────────────────────────────

/// Where the tab bar is rendered in the GTK frontend.
#[derive(Clone, Debug)]
pub enum GtkTabsLocation {
    Top,
    Bottom,
    Left,
    Right,
    Hidden,
}

impl std::fmt::Display for GtkTabsLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Top => "top",
            Self::Bottom => "bottom",
            Self::Left => "left",
            Self::Right => "right",
            Self::Hidden => "hidden",
        };
        write!(f, "{}", s)
    }
}

/// GTK-specific configuration options.
///
/// These settings are silently ignored on non-GTK builds.
#[derive(Default, Clone, Debug)]
pub struct GtkConfig {
    /// Run only a single Ghostty process; new windows open in the existing instance.
    pub single_instance: Option<bool>,
    pub tabs_location: Option<GtkTabsLocation>,
    /// Use wider tab labels.
    pub wide_tabs: Option<bool>,
    /// Apply Adwaita theming to the window chrome.
    pub adwaita: Option<bool>,
}

impl GtkConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn single_instance(mut self, v: bool) -> Self {
        self.single_instance = Some(v);
        self
    }

    pub fn tabs_location(mut self, v: GtkTabsLocation) -> Self {
        self.tabs_location = Some(v);
        self
    }

    pub fn wide_tabs(mut self, v: bool) -> Self {
        self.wide_tabs = Some(v);
        self
    }

    pub fn adwaita(mut self, v: bool) -> Self {
        self.adwaita = Some(v);
        self
    }
}

impl GhosttyConfig for GtkConfig {
    fn render(&self, _ctx: &GhosttyRenderContext) -> String {
        let mut out = String::new();

        if let Some(v) = self.single_instance {
            out.push_str(&format!("gtk-single-instance = {}\n", v));
        }
        if let Some(ref v) = self.tabs_location {
            out.push_str(&format!("gtk-tabs-location = {}\n", v));
        }
        if let Some(v) = self.wide_tabs {
            out.push_str(&format!("gtk-wide-tabs = {}\n", v));
        }
        if let Some(v) = self.adwaita {
            out.push_str(&format!("gtk-adwaita = {}\n", v));
        }

        out
    }
}

// ── macOS ────────────────────────────────────────────────────────────────────

/// macOS title-bar style.
#[derive(Clone, Debug)]
pub enum MacosTitlebarStyle {
    Native,
    Transparent,
    Tabs,
    Hidden,
}

impl std::fmt::Display for MacosTitlebarStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Native => "native",
            Self::Transparent => "transparent",
            Self::Tabs => "tabs",
            Self::Hidden => "hidden",
        };
        write!(f, "{}", s)
    }
}

/// Which modifier the Option key acts as on macOS.
#[derive(Clone, Debug)]
pub enum MacosOptionAsAlt {
    /// Do not remap Option.
    False,
    /// Left Option → Alt.
    Left,
    /// Right Option → Alt.
    Right,
    /// Both Option keys → Alt.
    True,
}

impl std::fmt::Display for MacosOptionAsAlt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::False => "false",
            Self::Left => "left",
            Self::Right => "right",
            Self::True => "true",
        };
        write!(f, "{}", s)
    }
}

/// macOS-specific configuration options.
///
/// These settings are silently ignored on non-macOS builds.
#[derive(Default, Clone, Debug)]
pub struct MacosConfig {
    pub option_as_alt: Option<MacosOptionAsAlt>,
    pub window_shadow: Option<bool>,
    pub titlebar_style: Option<MacosTitlebarStyle>,
    /// Show the proxy icon in the title bar.
    pub titlebar_proxy_icon: Option<bool>,
    /// Display a visual indicator when Secure Input is active.
    pub secure_input_indication: Option<bool>,
    /// Use a non-native fullscreen mode (faster transition).
    pub non_native_fullscreen: Option<bool>,
    /// Auto-hide the menu bar in fullscreen.
    pub hide_mouse_while_typing: Option<bool>,
}

impl MacosConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn option_as_alt(mut self, v: MacosOptionAsAlt) -> Self {
        self.option_as_alt = Some(v);
        self
    }

    pub fn window_shadow(mut self, v: bool) -> Self {
        self.window_shadow = Some(v);
        self
    }

    pub fn titlebar_style(mut self, v: MacosTitlebarStyle) -> Self {
        self.titlebar_style = Some(v);
        self
    }

    pub fn titlebar_proxy_icon(mut self, v: bool) -> Self {
        self.titlebar_proxy_icon = Some(v);
        self
    }

    pub fn secure_input_indication(mut self, v: bool) -> Self {
        self.secure_input_indication = Some(v);
        self
    }

    pub fn non_native_fullscreen(mut self, v: bool) -> Self {
        self.non_native_fullscreen = Some(v);
        self
    }

    pub fn hide_mouse_while_typing(mut self, v: bool) -> Self {
        self.hide_mouse_while_typing = Some(v);
        self
    }
}

impl GhosttyConfig for MacosConfig {
    fn render(&self, _ctx: &GhosttyRenderContext) -> String {
        let mut out = String::new();

        if let Some(ref v) = self.option_as_alt {
            out.push_str(&format!("macos-option-as-alt = {}\n", v));
        }
        if let Some(v) = self.window_shadow {
            out.push_str(&format!("macos-window-shadow = {}\n", v));
        }
        if let Some(ref v) = self.titlebar_style {
            out.push_str(&format!("macos-titlebar-style = {}\n", v));
        }
        if let Some(v) = self.titlebar_proxy_icon {
            out.push_str(&format!("macos-titlebar-proxy-icon = {}\n", v));
        }
        if let Some(v) = self.secure_input_indication {
            out.push_str(&format!("macos-secure-input-indication = {}\n", v));
        }
        if let Some(v) = self.non_native_fullscreen {
            out.push_str(&format!("macos-non-native-fullscreen = {}\n", v));
        }
        if let Some(v) = self.hide_mouse_while_typing {
            out.push_str(&format!("macos-hide-mouse-while-typing = {}\n", v));
        }

        out
    }
}
