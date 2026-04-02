//! Waybar `backlight` module.

use super::core::{json_object, json_str, WaybarModule, WaybarRenderContext};

/// Waybar screen-backlight brightness module.
///
/// # Example
/// ```
/// use toconfig::waybar::backlight::Backlight;
/// use toconfig::waybar::core::WaybarModule;
///
/// let b = Backlight::new()
///     .device("intel_backlight")
///     .format("{percent}% {icon}")
///     .format_icons(["", "", ""]);
///
/// assert_eq!(b.module_id(), "backlight");
/// ```
pub struct Backlight {
    id: String,
    pub device: Option<String>,
    pub format: Option<String>,
    pub format_alt: Option<String>,
    pub format_icons: Vec<String>,
    pub interval: Option<u32>,
    pub tooltip: Option<bool>,
    pub tooltip_format: Option<String>,
    pub on_scroll_up: Option<String>,
    pub on_scroll_down: Option<String>,
    pub on_click: Option<String>,
    pub scroll_step: Option<f32>,
    pub smooth_scrolling_threshold: Option<f64>,
}

impl Default for Backlight {
    fn default() -> Self {
        Self::new()
    }
}

impl Backlight {
    pub fn new() -> Self {
        Self {
            id: "backlight".into(),
            device: None,
            format: None,
            format_alt: None,
            format_icons: Vec::new(),
            interval: None,
            tooltip: None,
            tooltip_format: None,
            on_scroll_up: None,
            on_scroll_down: None,
            on_click: None,
            scroll_step: None,
            smooth_scrolling_threshold: None,
        }
    }

    pub fn instance(mut self, name: impl Into<String>) -> Self {
        self.id = format!("backlight#{}", name.into());
        self
    }

    /// Backlight device name (e.g. `"intel_backlight"`, `"amdgpu_bl0"`).
    pub fn device(mut self, d: impl Into<String>) -> Self {
        self.device = Some(d.into());
        self
    }

    pub fn format(mut self, f: impl Into<String>) -> Self {
        self.format = Some(f.into());
        self
    }

    pub fn format_alt(mut self, f: impl Into<String>) -> Self {
        self.format_alt = Some(f.into());
        self
    }

    /// Icons shown at different brightness levels (low → high).
    pub fn format_icons(mut self, icons: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.format_icons = icons.into_iter().map(|s| s.into()).collect();
        self
    }

    pub fn interval(mut self, secs: u32) -> Self {
        self.interval = Some(secs);
        self
    }

    pub fn tooltip(mut self, v: bool) -> Self {
        self.tooltip = Some(v);
        self
    }

    pub fn tooltip_format(mut self, f: impl Into<String>) -> Self {
        self.tooltip_format = Some(f.into());
        self
    }

    pub fn on_scroll_up(mut self, cmd: impl Into<String>) -> Self {
        self.on_scroll_up = Some(cmd.into());
        self
    }

    pub fn on_scroll_down(mut self, cmd: impl Into<String>) -> Self {
        self.on_scroll_down = Some(cmd.into());
        self
    }

    pub fn on_click(mut self, cmd: impl Into<String>) -> Self {
        self.on_click = Some(cmd.into());
        self
    }

    /// Brightness step per scroll tick (percent).
    pub fn scroll_step(mut self, step: f32) -> Self {
        self.scroll_step = Some(step);
        self
    }

    pub fn smooth_scrolling_threshold(mut self, t: f64) -> Self {
        self.smooth_scrolling_threshold = Some(t);
        self
    }
}

impl WaybarModule for Backlight {
    fn module_id(&self) -> &str {
        &self.id
    }

    fn render_config(&self, ctx: &WaybarRenderContext) -> String {
        let mut entries: Vec<(String, String)> = Vec::new();

        if let Some(ref v) = self.device {
            entries.push(("device".into(), json_str(v)));
        }
        if let Some(ref v) = self.format {
            entries.push(("format".into(), json_str(v)));
        }
        if let Some(ref v) = self.format_alt {
            entries.push(("format-alt".into(), json_str(v)));
        }
        if !self.format_icons.is_empty() {
            let quoted: Vec<String> = self.format_icons.iter().map(|s| json_str(s)).collect();
            entries.push(("format-icons".into(), format!("[{}]", quoted.join(", "))));
        }
        if let Some(v) = self.interval {
            entries.push(("interval".into(), v.to_string()));
        }
        if let Some(v) = self.tooltip {
            entries.push(("tooltip".into(), v.to_string()));
        }
        if let Some(ref v) = self.tooltip_format {
            entries.push(("tooltip-format".into(), json_str(v)));
        }
        if let Some(ref v) = self.on_scroll_up {
            entries.push(("on-scroll-up".into(), json_str(v)));
        }
        if let Some(ref v) = self.on_scroll_down {
            entries.push(("on-scroll-down".into(), json_str(v)));
        }
        if let Some(ref v) = self.on_click {
            entries.push(("on-click".into(), json_str(v)));
        }
        if let Some(v) = self.scroll_step {
            entries.push(("scroll-step".into(), format!("{}", v)));
        }
        if let Some(v) = self.smooth_scrolling_threshold {
            entries.push(("smooth-scrolling-threshold".into(), format!("{}", v)));
        }

        json_object(&entries, ctx)
    }
}
