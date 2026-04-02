use super::{KittyConfig, KittyRenderContext};

/// Tab bar style variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabBarStyle {
    /// Fade the tab bar in/out.
    Fade,
    /// Slant-separator tabs (powerline style).
    Slant,
    /// Separator character between tabs.
    Separator,
    /// Powerline chevron style.
    Powerline,
    /// Custom Python function.
    Custom,
    /// Hidden tab bar.
    Hidden,
}

impl TabBarStyle {
    fn as_str(self) -> &'static str {
        match self {
            TabBarStyle::Fade => "fade",
            TabBarStyle::Slant => "slant",
            TabBarStyle::Separator => "separator",
            TabBarStyle::Powerline => "powerline",
            TabBarStyle::Custom => "custom",
            TabBarStyle::Hidden => "hidden",
        }
    }
}

/// Tab bar edge: top or bottom of the window.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabBarEdge {
    Top,
    Bottom,
}

impl TabBarEdge {
    fn as_str(self) -> &'static str {
        match self {
            TabBarEdge::Top => "top",
            TabBarEdge::Bottom => "bottom",
        }
    }
}

/// Tab bar configuration for kitty terminal.
///
/// # Example
/// ```
/// use toconfig::kitty::KittyConfig;
/// use toconfig::kitty::tab_bar::{TabBarConfig, TabBarStyle, TabBarEdge};
/// let t = TabBarConfig::new()
///     .style(TabBarStyle::Powerline)
///     .edge(TabBarEdge::Bottom)
///     .active_foreground("#cdd6f4")
///     .active_background("#1e1e2e");
/// let out = t.generate();
/// assert!(out.contains("tab_bar_style powerline"));
/// ```
#[derive(Default)]
pub struct TabBarConfig {
    /// Visual style of the tab bar.
    pub style: Option<TabBarStyle>,
    /// Screen edge where the tab bar is rendered.
    pub edge: Option<TabBarEdge>,
    /// External margin around the tab bar (pixels).
    pub margin_width: Option<f32>,
    /// Character used as separator in `Separator` style.
    pub separator: Option<String>,
    /// Powerline separator character.
    pub powerline_style: Option<String>,
    /// Fade amount for inactive tabs in `Fade` style (0.0–1.0).
    pub fade_colors: Option<bool>,
    /// Minimum number of tabs required before the bar is shown.
    pub min_tabs: Option<u32>,
    /// Font style for the active tab: `"bold"`, `"italic"`, `"bold_italic"`, `"normal"`.
    pub active_font_style: Option<String>,
    /// Font style for inactive tabs.
    pub inactive_font_style: Option<String>,
    /// Foreground color of the active tab.
    pub active_foreground: Option<String>,
    /// Background color of the active tab.
    pub active_background: Option<String>,
    /// Foreground color of inactive tabs.
    pub inactive_foreground: Option<String>,
    /// Background color of inactive tabs.
    pub inactive_background: Option<String>,
    /// Background color of the tab bar itself.
    pub bar_background: Option<String>,
    /// Tab title template string.
    pub title_template: Option<String>,
    /// Active tab title template string.
    pub active_title_template: Option<String>,
}

impl TabBarConfig {
    pub fn new() -> Self {
        Self::default()
    }

    /// Visual style of the tab bar.
    pub fn style(mut self, v: TabBarStyle) -> Self {
        self.style = Some(v);
        self
    }

    /// Edge where the tab bar is placed.
    pub fn edge(mut self, v: TabBarEdge) -> Self {
        self.edge = Some(v);
        self
    }

    /// Margin around the tab bar in pixels.
    pub fn margin_width(mut self, v: f32) -> Self {
        self.margin_width = Some(v);
        self
    }

    /// Separator character (for `Separator` style).
    pub fn separator(mut self, v: impl Into<String>) -> Self {
        self.separator = Some(v.into());
        self
    }

    /// Powerline separator style: `"angled"` | `"slanted"` | `"round"`.
    pub fn powerline_style(mut self, v: impl Into<String>) -> Self {
        self.powerline_style = Some(v.into());
        self
    }

    /// Enable color fading for inactive tabs.
    pub fn fade_colors(mut self, v: bool) -> Self {
        self.fade_colors = Some(v);
        self
    }

    /// Minimum number of tabs before the bar appears.
    pub fn min_tabs(mut self, v: u32) -> Self {
        self.min_tabs = Some(v);
        self
    }

    /// Font style for the active tab.
    pub fn active_font_style(mut self, v: impl Into<String>) -> Self {
        self.active_font_style = Some(v.into());
        self
    }

    /// Font style for inactive tabs.
    pub fn inactive_font_style(mut self, v: impl Into<String>) -> Self {
        self.inactive_font_style = Some(v.into());
        self
    }

    /// Foreground color of the active tab.
    pub fn active_foreground(mut self, v: impl Into<String>) -> Self {
        self.active_foreground = Some(v.into());
        self
    }

    /// Background color of the active tab.
    pub fn active_background(mut self, v: impl Into<String>) -> Self {
        self.active_background = Some(v.into());
        self
    }

    /// Foreground color of inactive tabs.
    pub fn inactive_foreground(mut self, v: impl Into<String>) -> Self {
        self.inactive_foreground = Some(v.into());
        self
    }

    /// Background color of inactive tabs.
    pub fn inactive_background(mut self, v: impl Into<String>) -> Self {
        self.inactive_background = Some(v.into());
        self
    }

    /// Background color of the tab bar area itself.
    pub fn bar_background(mut self, v: impl Into<String>) -> Self {
        self.bar_background = Some(v.into());
        self
    }

    /// Tab title template (kitty template syntax).
    pub fn title_template(mut self, v: impl Into<String>) -> Self {
        self.title_template = Some(v.into());
        self
    }

    /// Title template for the active tab.
    pub fn active_title_template(mut self, v: impl Into<String>) -> Self {
        self.active_title_template = Some(v.into());
        self
    }
}

impl KittyConfig for TabBarConfig {
    fn render(&self, ctx: &KittyRenderContext) -> String {
        let indent = ctx.indent();
        let mut lines: Vec<String> = Vec::new();
        if let Some(v) = self.style {
            lines.push(format!("{}tab_bar_style {}", indent, v.as_str()));
        }
        if let Some(v) = self.edge {
            lines.push(format!("{}tab_bar_edge {}", indent, v.as_str()));
        }
        if let Some(v) = self.margin_width {
            lines.push(format!("{}tab_bar_margin_width {}", indent, v));
        }
        if let Some(ref v) = self.separator {
            lines.push(format!("{}tab_separator \"{}\"", indent, v));
        }
        if let Some(ref v) = self.powerline_style {
            lines.push(format!("{}tab_powerline_style {}", indent, v));
        }
        if let Some(v) = self.fade_colors {
            lines.push(format!(
                "{}tab_bar_background {}",
                indent,
                if v { "yes" } else { "no" }
            ));
        }
        if let Some(v) = self.min_tabs {
            lines.push(format!("{}tab_bar_min_tabs {}", indent, v));
        }
        if let Some(ref v) = self.active_font_style {
            lines.push(format!("{}active_tab_font_style {}", indent, v));
        }
        if let Some(ref v) = self.inactive_font_style {
            lines.push(format!("{}inactive_tab_font_style {}", indent, v));
        }
        if let Some(ref v) = self.active_foreground {
            lines.push(format!("{}active_tab_foreground {}", indent, v));
        }
        if let Some(ref v) = self.active_background {
            lines.push(format!("{}active_tab_background {}", indent, v));
        }
        if let Some(ref v) = self.inactive_foreground {
            lines.push(format!("{}inactive_tab_foreground {}", indent, v));
        }
        if let Some(ref v) = self.inactive_background {
            lines.push(format!("{}inactive_tab_background {}", indent, v));
        }
        if let Some(ref v) = self.bar_background {
            lines.push(format!("{}tab_bar_background {}", indent, v));
        }
        if let Some(ref v) = self.title_template {
            lines.push(format!("{}tab_title_template \"{}\"", indent, v));
        }
        if let Some(ref v) = self.active_title_template {
            lines.push(format!("{}active_tab_title_template \"{}\"", indent, v));
        }
        lines.join("\n")
    }
}
