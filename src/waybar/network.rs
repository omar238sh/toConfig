//! Waybar `network` module.

use super::core::{json_object, json_str, WaybarModule, WaybarRenderContext};

/// Waybar network module.
///
/// # Example
/// ```
/// use toconfig::waybar::network::Network;
/// use toconfig::waybar::core::WaybarModule;
///
/// let n = Network::new()
///     .format_wifi("{essid} ({signalStrength}%) ")
///     .format_ethernet("{ifname} ")
///     .format_disconnected("Disconnected ⚠");
///
/// assert_eq!(n.module_id(), "network");
/// ```
pub struct Network {
    id: String,
    pub interface: Option<String>,
    pub interval: Option<u32>,
    pub format: Option<String>,
    pub format_wifi: Option<String>,
    pub format_ethernet: Option<String>,
    pub format_linked: Option<String>,
    pub format_disconnected: Option<String>,
    pub format_alt: Option<String>,
    pub tooltip: Option<bool>,
    pub tooltip_format: Option<String>,
    pub tooltip_format_wifi: Option<String>,
    pub tooltip_format_ethernet: Option<String>,
    pub tooltip_format_disconnected: Option<String>,
    pub on_click: Option<String>,
    pub on_click_right: Option<String>,
    pub max_length: Option<u32>,
}

impl Default for Network {
    fn default() -> Self {
        Self::new()
    }
}

impl Network {
    pub fn new() -> Self {
        Self {
            id: "network".into(),
            interface: None,
            interval: None,
            format: None,
            format_wifi: None,
            format_ethernet: None,
            format_linked: None,
            format_disconnected: None,
            format_alt: None,
            tooltip: None,
            tooltip_format: None,
            tooltip_format_wifi: None,
            tooltip_format_ethernet: None,
            tooltip_format_disconnected: None,
            on_click: None,
            on_click_right: None,
            max_length: None,
        }
    }

    pub fn instance(mut self, name: impl Into<String>) -> Self {
        self.id = format!("network#{}", name.into());
        self
    }

    /// Restrict to a specific network interface (e.g. `"wlan0"`).
    pub fn interface(mut self, iface: impl Into<String>) -> Self {
        self.interface = Some(iface.into());
        self
    }

    pub fn interval(mut self, secs: u32) -> Self {
        self.interval = Some(secs);
        self
    }

    /// Fallback format when no more-specific format matches.
    pub fn format(mut self, f: impl Into<String>) -> Self {
        self.format = Some(f.into());
        self
    }

    pub fn format_wifi(mut self, f: impl Into<String>) -> Self {
        self.format_wifi = Some(f.into());
        self
    }

    pub fn format_ethernet(mut self, f: impl Into<String>) -> Self {
        self.format_ethernet = Some(f.into());
        self
    }

    pub fn format_linked(mut self, f: impl Into<String>) -> Self {
        self.format_linked = Some(f.into());
        self
    }

    pub fn format_disconnected(mut self, f: impl Into<String>) -> Self {
        self.format_disconnected = Some(f.into());
        self
    }

    pub fn format_alt(mut self, f: impl Into<String>) -> Self {
        self.format_alt = Some(f.into());
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

    pub fn tooltip_format_wifi(mut self, f: impl Into<String>) -> Self {
        self.tooltip_format_wifi = Some(f.into());
        self
    }

    pub fn tooltip_format_ethernet(mut self, f: impl Into<String>) -> Self {
        self.tooltip_format_ethernet = Some(f.into());
        self
    }

    pub fn tooltip_format_disconnected(mut self, f: impl Into<String>) -> Self {
        self.tooltip_format_disconnected = Some(f.into());
        self
    }

    pub fn on_click(mut self, cmd: impl Into<String>) -> Self {
        self.on_click = Some(cmd.into());
        self
    }

    pub fn on_click_right(mut self, cmd: impl Into<String>) -> Self {
        self.on_click_right = Some(cmd.into());
        self
    }

    /// Truncate the displayed text to at most this many characters.
    pub fn max_length(mut self, n: u32) -> Self {
        self.max_length = Some(n);
        self
    }
}

impl WaybarModule for Network {
    fn module_id(&self) -> &str {
        &self.id
    }

    fn render_config(&self, ctx: &WaybarRenderContext) -> String {
        let mut entries: Vec<(String, String)> = Vec::new();

        if let Some(ref v) = self.interface {
            entries.push(("interface".into(), json_str(v)));
        }
        if let Some(v) = self.interval {
            entries.push(("interval".into(), v.to_string()));
        }
        if let Some(ref v) = self.format {
            entries.push(("format".into(), json_str(v)));
        }
        if let Some(ref v) = self.format_wifi {
            entries.push(("format-wifi".into(), json_str(v)));
        }
        if let Some(ref v) = self.format_ethernet {
            entries.push(("format-ethernet".into(), json_str(v)));
        }
        if let Some(ref v) = self.format_linked {
            entries.push(("format-linked".into(), json_str(v)));
        }
        if let Some(ref v) = self.format_disconnected {
            entries.push(("format-disconnected".into(), json_str(v)));
        }
        if let Some(ref v) = self.format_alt {
            entries.push(("format-alt".into(), json_str(v)));
        }
        if let Some(v) = self.tooltip {
            entries.push(("tooltip".into(), v.to_string()));
        }
        if let Some(ref v) = self.tooltip_format {
            entries.push(("tooltip-format".into(), json_str(v)));
        }
        if let Some(ref v) = self.tooltip_format_wifi {
            entries.push(("tooltip-format-wifi".into(), json_str(v)));
        }
        if let Some(ref v) = self.tooltip_format_ethernet {
            entries.push(("tooltip-format-ethernet".into(), json_str(v)));
        }
        if let Some(ref v) = self.tooltip_format_disconnected {
            entries.push(("tooltip-format-disconnected".into(), json_str(v)));
        }
        if let Some(ref v) = self.on_click {
            entries.push(("on-click".into(), json_str(v)));
        }
        if let Some(ref v) = self.on_click_right {
            entries.push(("on-click-right".into(), json_str(v)));
        }
        if let Some(v) = self.max_length {
            entries.push(("max-length".into(), v.to_string()));
        }

        json_object(&entries, ctx)
    }
}
