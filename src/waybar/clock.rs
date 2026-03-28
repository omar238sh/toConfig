//! Waybar `clock` module.

use super::core::{json_object, json_str, WaybarModule, WaybarRenderContext};

/// Waybar clock module.
///
/// Renders the current time and optional date using `strftime`-style format
/// strings.
///
/// # Example
/// ```
/// use toconfig::waybar::clock::Clock;
/// use toconfig::waybar::core::WaybarModule;
///
/// let c = Clock::new()
///     .format("{:%H:%M}")
///     .tooltip_format("<big>{:%Y %B}</big>\n<tt><small>{calendar}</small></tt>");
///
/// let json = c.render_config(&Default::default());
/// assert!(json.contains("format"));
/// ```
pub struct Clock {
    /// Full module identifier (e.g. `"clock"` or `"clock#work"`).
    id: String,
    pub format: Option<String>,
    pub format_alt: Option<String>,
    pub interval: Option<u32>,
    pub tooltip: Option<bool>,
    pub tooltip_format: Option<String>,
    pub locale: Option<String>,
    pub timezones: Vec<String>,
    pub on_click: Option<String>,
}

impl Default for Clock {
    fn default() -> Self {
        Self::new()
    }
}

impl Clock {
    pub fn new() -> Self {
        Self {
            id: "clock".into(),
            format: None,
            format_alt: None,
            interval: None,
            tooltip: None,
            tooltip_format: None,
            locale: None,
            timezones: Vec::new(),
            on_click: None,
        }
    }

    /// Named instance suffix (e.g. `"work"` → id becomes `"clock#work"`).
    pub fn instance(mut self, name: impl Into<String>) -> Self {
        self.id = format!("clock#{}", name.into());
        self
    }

    /// `strftime` format string (e.g. `"{:%H:%M}"`).
    pub fn format(mut self, f: impl Into<String>) -> Self {
        self.format = Some(f.into());
        self
    }

    /// Alternate format shown on click.
    pub fn format_alt(mut self, f: impl Into<String>) -> Self {
        self.format_alt = Some(f.into());
        self
    }

    /// Update interval in seconds (default: 60).
    pub fn interval(mut self, secs: u32) -> Self {
        self.interval = Some(secs);
        self
    }

    /// Show a tooltip.
    pub fn tooltip(mut self, v: bool) -> Self {
        self.tooltip = Some(v);
        self
    }

    /// Tooltip format string.
    pub fn tooltip_format(mut self, f: impl Into<String>) -> Self {
        self.tooltip_format = Some(f.into());
        self
    }

    /// Locale used for date/time formatting (e.g. `"en_US"`).
    pub fn locale(mut self, l: impl Into<String>) -> Self {
        self.locale = Some(l.into());
        self
    }

    /// List of timezones to cycle through on scroll.
    pub fn timezones(mut self, tzs: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.timezones = tzs.into_iter().map(|s| s.into()).collect();
        self
    }

    /// Command to execute on click.
    pub fn on_click(mut self, cmd: impl Into<String>) -> Self {
        self.on_click = Some(cmd.into());
        self
    }
}

impl WaybarModule for Clock {
    fn module_id(&self) -> &str {
        &self.id
    }

    fn render_config(&self, ctx: &WaybarRenderContext) -> String {
        let mut entries: Vec<(String, String)> = Vec::new();

        if let Some(ref v) = self.format {
            entries.push(("format".into(), json_str(v)));
        }
        if let Some(ref v) = self.format_alt {
            entries.push(("format-alt".into(), json_str(v)));
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
        if let Some(ref v) = self.locale {
            entries.push(("locale".into(), json_str(v)));
        }
        if !self.timezones.is_empty() {
            let quoted: Vec<String> = self.timezones.iter().map(|s| json_str(s)).collect();
            entries.push(("timezones".into(), format!("[{}]", quoted.join(", "))));
        }
        if let Some(ref v) = self.on_click {
            entries.push(("on-click".into(), json_str(v)));
        }

        json_object(&entries, ctx)
    }
}
