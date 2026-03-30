//! Waybar `pulseaudio` module.

use super::core::{json_object, json_str, WaybarModule, WaybarRenderContext};

/// Waybar PulseAudio (or PipeWire-pulse) volume module.
///
/// # Example
/// ```
/// use toconfig::waybar::pulseaudio::Pulseaudio;
/// use toconfig::waybar::core::WaybarModule;
///
/// let p = Pulseaudio::new()
///     .format("{volume}% {icon}  {format_source}")
///     .format_muted(" Muted")
///     .on_click("pavucontrol");
///
/// assert_eq!(p.module_id(), "pulseaudio");
/// ```
pub struct Pulseaudio {
    id: String,
    pub format: Option<String>,
    pub format_bluetooth: Option<String>,
    pub format_bluetooth_muted: Option<String>,
    pub format_muted: Option<String>,
    pub format_source: Option<String>,
    pub format_source_muted: Option<String>,
    pub format_alt: Option<String>,
    /// Icons keyed by sink type: `"headphone"`, `"hands-free"`, `"headset"`, `"phone"`,
    /// `"portable"`, `"car"`, `"default"`.  Each value can be a single string or an
    /// array of strings (stored pre-serialised as JSON).
    pub format_icons: Vec<(String, String)>,
    pub scroll_step: Option<f32>,
    pub tooltip: Option<bool>,
    pub tooltip_format: Option<String>,
    pub on_click: Option<String>,
    pub on_click_right: Option<String>,
    pub on_scroll_up: Option<String>,
    pub on_scroll_down: Option<String>,
}

impl Default for Pulseaudio {
    fn default() -> Self {
        Self::new()
    }
}

impl Pulseaudio {
    pub fn new() -> Self {
        Self {
            id: "pulseaudio".into(),
            format: None,
            format_bluetooth: None,
            format_bluetooth_muted: None,
            format_muted: None,
            format_source: None,
            format_source_muted: None,
            format_alt: None,
            format_icons: Vec::new(),
            scroll_step: None,
            tooltip: None,
            tooltip_format: None,
            on_click: None,
            on_click_right: None,
            on_scroll_up: None,
            on_scroll_down: None,
        }
    }

    pub fn instance(mut self, name: impl Into<String>) -> Self {
        self.id = format!("pulseaudio#{}", name.into());
        self
    }

    pub fn format(mut self, f: impl Into<String>) -> Self {
        self.format = Some(f.into());
        self
    }
    pub fn format_bluetooth(mut self, f: impl Into<String>) -> Self {
        self.format_bluetooth = Some(f.into());
        self
    }
    pub fn format_bluetooth_muted(mut self, f: impl Into<String>) -> Self {
        self.format_bluetooth_muted = Some(f.into());
        self
    }
    pub fn format_muted(mut self, f: impl Into<String>) -> Self {
        self.format_muted = Some(f.into());
        self
    }
    pub fn format_source(mut self, f: impl Into<String>) -> Self {
        self.format_source = Some(f.into());
        self
    }
    pub fn format_source_muted(mut self, f: impl Into<String>) -> Self {
        self.format_source_muted = Some(f.into());
        self
    }
    pub fn format_alt(mut self, f: impl Into<String>) -> Self {
        self.format_alt = Some(f.into());
        self
    }

    /// Add a single icon for a named sink type (e.g. `"default"`, `"headphone"`).
    pub fn format_icon(mut self, kind: impl Into<String>, icon: impl Into<String>) -> Self {
        self.format_icons
            .push((kind.into(), json_str(&icon.into())));
        self
    }

    /// Add an array of icons for a named sink type.
    pub fn format_icon_list(
        mut self,
        kind: impl Into<String>,
        icons: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        let quoted: Vec<String> = icons.into_iter().map(|s| json_str(&s.into())).collect();
        self.format_icons
            .push((kind.into(), format!("[{}]", quoted.join(", "))));
        self
    }

    /// Volume step for mouse scroll (percent).
    pub fn scroll_step(mut self, step: f32) -> Self {
        self.scroll_step = Some(step);
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
    pub fn on_click_right(mut self, cmd: impl Into<String>) -> Self {
        self.on_click_right = Some(cmd.into());
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
}

impl WaybarModule for Pulseaudio {
    fn module_id(&self) -> &str {
        &self.id
    }

    fn render_config(&self, ctx: &WaybarRenderContext) -> String {
        let mut entries: Vec<(String, String)> = Vec::new();

        if let Some(ref v) = self.format {
            entries.push(("format".into(), json_str(v)));
        }
        if let Some(ref v) = self.format_bluetooth {
            entries.push(("format-bluetooth".into(), json_str(v)));
        }
        if let Some(ref v) = self.format_bluetooth_muted {
            entries.push(("format-bluetooth-muted".into(), json_str(v)));
        }
        if let Some(ref v) = self.format_muted {
            entries.push(("format-muted".into(), json_str(v)));
        }
        if let Some(ref v) = self.format_source {
            entries.push(("format-source".into(), json_str(v)));
        }
        if let Some(ref v) = self.format_source_muted {
            entries.push(("format-source-muted".into(), json_str(v)));
        }
        if let Some(ref v) = self.format_alt {
            entries.push(("format-alt".into(), json_str(v)));
        }
        if !self.format_icons.is_empty() {
            let icon_entries: Vec<(String, String)> = self
                .format_icons
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();
            entries.push(("format-icons".into(), json_object(&icon_entries, &ctx.deeper())));
        }
        if let Some(v) = self.scroll_step {
            entries.push(("scroll-step".into(), format!("{}", v)));
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
        if let Some(ref v) = self.on_click_right {
            entries.push(("on-click-right".into(), json_str(v)));
        }
        if let Some(ref v) = self.on_scroll_up {
            entries.push(("on-scroll-up".into(), json_str(v)));
        }
        if let Some(ref v) = self.on_scroll_down {
            entries.push(("on-scroll-down".into(), json_str(v)));
        }

        json_object(&entries, ctx)
    }
}
