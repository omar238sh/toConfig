//! Waybar `cpu` module.

use super::core::{json_object, json_str, WaybarModule, WaybarRenderContext};

/// Waybar CPU-usage module.
///
/// # Example
/// ```
/// use toconfig::waybar::cpu::Cpu;
/// use toconfig::waybar::core::WaybarModule;
///
/// let c = Cpu::new().format("{usage}% ").interval(5);
/// assert_eq!(c.module_id(), "cpu");
/// ```
pub struct Cpu {
    id: String,
    pub format: Option<String>,
    pub format_alt: Option<String>,
    pub interval: Option<u32>,
    pub tooltip: Option<bool>,
    pub on_click: Option<String>,
    pub states_warning: Option<u32>,
    pub states_critical: Option<u32>,
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            id: "cpu".into(),
            format: None,
            format_alt: None,
            interval: None,
            tooltip: None,
            on_click: None,
            states_warning: None,
            states_critical: None,
        }
    }

    pub fn instance(mut self, name: impl Into<String>) -> Self {
        self.id = format!("cpu#{}", name.into());
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

    pub fn interval(mut self, secs: u32) -> Self {
        self.interval = Some(secs);
        self
    }

    pub fn tooltip(mut self, v: bool) -> Self {
        self.tooltip = Some(v);
        self
    }

    pub fn on_click(mut self, cmd: impl Into<String>) -> Self {
        self.on_click = Some(cmd.into());
        self
    }

    /// CPU usage percentage above which the `warning` CSS class is applied.
    pub fn states_warning(mut self, pct: u32) -> Self {
        self.states_warning = Some(pct);
        self
    }

    /// CPU usage percentage above which the `critical` CSS class is applied.
    pub fn states_critical(mut self, pct: u32) -> Self {
        self.states_critical = Some(pct);
        self
    }
}

impl WaybarModule for Cpu {
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
        if let Some(ref v) = self.on_click {
            entries.push(("on-click".into(), json_str(v)));
        }
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

        json_object(&entries, ctx)
    }
}
