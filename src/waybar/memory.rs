//! Waybar `memory` module.

use super::core::{json_object, json_str, WaybarModule, WaybarRenderContext};

/// Waybar memory-usage module.
///
/// # Example
/// ```
/// use toconfig::waybar::memory::Memory;
/// use toconfig::waybar::core::WaybarModule;
///
/// let m = Memory::new().format("{used:0.1f}G/{total:0.1f}G ");
/// assert_eq!(m.module_id(), "memory");
/// ```
pub struct Memory {
    id: String,
    pub format: Option<String>,
    pub format_alt: Option<String>,
    pub interval: Option<u32>,
    pub tooltip: Option<bool>,
    pub tooltip_format: Option<String>,
    pub on_click: Option<String>,
    pub states_warning: Option<u32>,
    pub states_critical: Option<u32>,
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory {
    pub fn new() -> Self {
        Self {
            id: "memory".into(),
            format: None,
            format_alt: None,
            interval: None,
            tooltip: None,
            tooltip_format: None,
            on_click: None,
            states_warning: None,
            states_critical: None,
        }
    }

    pub fn instance(mut self, name: impl Into<String>) -> Self {
        self.id = format!("memory#{}", name.into());
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

    pub fn tooltip_format(mut self, f: impl Into<String>) -> Self {
        self.tooltip_format = Some(f.into());
        self
    }

    pub fn on_click(mut self, cmd: impl Into<String>) -> Self {
        self.on_click = Some(cmd.into());
        self
    }

    pub fn states_warning(mut self, pct: u32) -> Self {
        self.states_warning = Some(pct);
        self
    }

    pub fn states_critical(mut self, pct: u32) -> Self {
        self.states_critical = Some(pct);
        self
    }
}

impl WaybarModule for Memory {
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
