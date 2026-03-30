//! Waybar `battery` module.

use super::core::{json_object, json_str, WaybarModule, WaybarRenderContext};

/// Waybar battery module.
///
/// # Example
/// ```
/// use toconfig::waybar::battery::Battery;
/// use toconfig::waybar::core::WaybarModule;
///
/// let b = Battery::new()
///     .bat("BAT0")
///     .format("{capacity}% {icon}")
///     .format_icons(["", "", "", "", ""]);
///
/// assert_eq!(b.module_id(), "battery");
/// let json = b.render_config(&Default::default());
/// assert!(json.contains("BAT0"));
/// ```
pub struct Battery {
    id: String,
    pub bat: Option<String>,
    pub adapter: Option<String>,
    pub full_at: Option<f32>,
    pub design_capacity: Option<bool>,
    pub interval: Option<u32>,
    pub format: Option<String>,
    pub format_charging: Option<String>,
    pub format_plugged: Option<String>,
    pub format_full: Option<String>,
    pub format_alt: Option<String>,
    pub format_icons: Vec<String>,
    pub states_warning: Option<u32>,
    pub states_critical: Option<u32>,
    pub tooltip: Option<bool>,
    pub tooltip_format: Option<String>,
    pub on_click: Option<String>,
}

impl Default for Battery {
    fn default() -> Self {
        Self::new()
    }
}

impl Battery {
    pub fn new() -> Self {
        Self {
            id: "battery".into(),
            bat: None,
            adapter: None,
            full_at: None,
            design_capacity: None,
            interval: None,
            format: None,
            format_charging: None,
            format_plugged: None,
            format_full: None,
            format_alt: None,
            format_icons: Vec::new(),
            states_warning: None,
            states_critical: None,
            tooltip: None,
            tooltip_format: None,
            on_click: None,
        }
    }

    /// Named instance suffix (e.g. `"laptop"` → id becomes `"battery#laptop"`).
    pub fn instance(mut self, name: impl Into<String>) -> Self {
        self.id = format!("battery#{}", name.into());
        self
    }

    /// Battery device name (e.g. `"BAT0"`).
    pub fn bat(mut self, bat: impl Into<String>) -> Self {
        self.bat = Some(bat.into());
        self
    }

    /// AC adapter device name (e.g. `"ADP1"`).
    pub fn adapter(mut self, adapter: impl Into<String>) -> Self {
        self.adapter = Some(adapter.into());
        self
    }

    /// Report full at this percentage (default: 100).
    pub fn full_at(mut self, pct: f32) -> Self {
        self.full_at = Some(pct);
        self
    }

    /// Use design capacity instead of full capacity.
    pub fn design_capacity(mut self, v: bool) -> Self {
        self.design_capacity = Some(v);
        self
    }

    /// Update interval in seconds.
    pub fn interval(mut self, secs: u32) -> Self {
        self.interval = Some(secs);
        self
    }

    pub fn format(mut self, f: impl Into<String>) -> Self {
        self.format = Some(f.into());
        self
    }
    pub fn format_charging(mut self, f: impl Into<String>) -> Self {
        self.format_charging = Some(f.into());
        self
    }
    pub fn format_plugged(mut self, f: impl Into<String>) -> Self {
        self.format_plugged = Some(f.into());
        self
    }
    pub fn format_full(mut self, f: impl Into<String>) -> Self {
        self.format_full = Some(f.into());
        self
    }
    pub fn format_alt(mut self, f: impl Into<String>) -> Self {
        self.format_alt = Some(f.into());
        self
    }

    /// Icons shown at different charge levels (low → high).
    pub fn format_icons(mut self, icons: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.format_icons = icons.into_iter().map(|s| s.into()).collect();
        self
    }

    /// Percentage below which the `warning` CSS class is applied.
    pub fn states_warning(mut self, pct: u32) -> Self {
        self.states_warning = Some(pct);
        self
    }

    /// Percentage below which the `critical` CSS class is applied.
    pub fn states_critical(mut self, pct: u32) -> Self {
        self.states_critical = Some(pct);
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
    pub fn on_click(mut self, cmd: impl Into<String>) -> Self {
        self.on_click = Some(cmd.into());
        self
    }
}

impl WaybarModule for Battery {
    fn module_id(&self) -> &str {
        &self.id
    }

    fn render_config(&self, ctx: &WaybarRenderContext) -> String {
        let mut entries: Vec<(String, String)> = Vec::new();

        if let Some(ref v) = self.bat {
            entries.push(("bat".into(), json_str(v)));
        }
        if let Some(ref v) = self.adapter {
            entries.push(("adapter".into(), json_str(v)));
        }
        if let Some(v) = self.full_at {
            entries.push(("full-at".into(), format!("{}", v)));
        }
        if let Some(v) = self.design_capacity {
            entries.push(("design-capacity".into(), v.to_string()));
        }
        if let Some(v) = self.interval {
            entries.push(("interval".into(), v.to_string()));
        }
        if let Some(ref v) = self.format {
            entries.push(("format".into(), json_str(v)));
        }
        if let Some(ref v) = self.format_charging {
            entries.push(("format-charging".into(), json_str(v)));
        }
        if let Some(ref v) = self.format_plugged {
            entries.push(("format-plugged".into(), json_str(v)));
        }
        if let Some(ref v) = self.format_full {
            entries.push(("format-full".into(), json_str(v)));
        }
        if let Some(ref v) = self.format_alt {
            entries.push(("format-alt".into(), json_str(v)));
        }
        if !self.format_icons.is_empty() {
            let quoted: Vec<String> = self.format_icons.iter().map(|s| json_str(s)).collect();
            entries.push(("format-icons".into(), format!("[{}]", quoted.join(", "))));
        }
        // states sub-object
        {
            let mut state_entries: Vec<(String, String)> = Vec::new();
            if let Some(v) = self.states_warning {
                state_entries.push(("warning".into(), v.to_string()));
            }
            if let Some(v) = self.states_critical {
                state_entries.push(("critical".into(), v.to_string()));
            }
            if !state_entries.is_empty() {
                entries.push(("states".into(), json_object(&state_entries, &ctx.deeper())));
            }
        }
        if let Some(v) = self.tooltip {
            entries.push(("tooltip".into(), v.to_string()));
        }
        if let Some(ref v) = self.tooltip_format {
            entries.push(("tooltip-format".into(), json_str(v)));
        }
        if let Some(ref v) = self.on_click {
            entries.push(("on-click".into(), json_str(v)));
        }

        json_object(&entries, ctx)
    }
}
