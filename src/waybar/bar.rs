//! Top-level [`Bar`] configuration struct for Waybar.

use super::core::{
    json_array_str, json_object, json_str, WaybarConfig, WaybarModule, WaybarRenderContext,
};

/// A complete Waybar bar configuration.
///
/// Modules are added to the left, centre, or right slots via
/// [`Bar::add_left`], [`Bar::add_center`], and [`Bar::add_right`].
/// Each call records the module identifier in the appropriate slot *and*
/// stores the module's JSON config so it is included in the rendered output.
///
/// # Example
/// ```
/// use toconfig::waybar::{Bar, WaybarConfig, clock::Clock, battery::Battery};
///
/// let bar = Bar::new()
///     .position("top")
///     .height(30)
///     .add_center(Clock::new().format("{:%H:%M}"))
///     .add_right(Battery::new().format("{capacity}% {icon}"));
///
/// let json = bar.generate();
/// assert!(json.contains("\"position\": \"top\""));
/// assert!(json.contains("\"clock\""));
/// assert!(json.contains("\"battery\""));
/// ```
pub struct Bar {
    // ── Bar-level settings ────────────────────────────────────────────────
    pub name: Option<String>,
    pub output: Option<String>,
    pub layer: Option<String>,
    pub position: Option<String>,
    pub height: Option<u32>,
    pub width: Option<u32>,
    pub margin: Option<String>,
    pub margin_top: Option<u32>,
    pub margin_right: Option<u32>,
    pub margin_bottom: Option<u32>,
    pub margin_left: Option<u32>,
    pub spacing: Option<u32>,
    pub exclusive: Option<bool>,
    pub passthrough: Option<bool>,
    pub start_hidden: Option<bool>,
    pub reload_style_on_change: Option<bool>,
    // ── Module slots ──────────────────────────────────────────────────────
    pub modules_left: Vec<String>,
    pub modules_center: Vec<String>,
    pub modules_right: Vec<String>,
    /// Stored module configurations (in insertion order).
    pub modules: Vec<Box<dyn WaybarModule>>,
}

impl Default for Bar {
    fn default() -> Self {
        Self::new()
    }
}

impl Bar {
    pub fn new() -> Self {
        Self {
            name: None,
            output: None,
            layer: None,
            position: None,
            height: None,
            width: None,
            margin: None,
            margin_top: None,
            margin_right: None,
            margin_bottom: None,
            margin_left: None,
            spacing: None,
            exclusive: None,
            passthrough: None,
            start_hidden: None,
            reload_style_on_change: None,
            modules_left: Vec::new(),
            modules_center: Vec::new(),
            modules_right: Vec::new(),
            modules: Vec::new(),
        }
    }

    // ── Setters ───────────────────────────────────────────────────────────

    /// A unique name for this bar (used with `waybar -b <name>`).
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Restrict the bar to a specific output/monitor (e.g. `"eDP-1"`).
    pub fn output(mut self, output: impl Into<String>) -> Self {
        self.output = Some(output.into());
        self
    }

    /// Rendering layer: `"top"` (default) or `"bottom"`.
    pub fn layer(mut self, layer: impl Into<String>) -> Self {
        self.layer = Some(layer.into());
        self
    }

    /// Screen edge: `"top"`, `"bottom"`, `"left"`, or `"right"`.
    pub fn position(mut self, position: impl Into<String>) -> Self {
        self.position = Some(position.into());
        self
    }

    /// Bar height in pixels.
    pub fn height(mut self, h: u32) -> Self {
        self.height = Some(h);
        self
    }

    /// Bar width in pixels (omit to span the full monitor width/height).
    pub fn width(mut self, w: u32) -> Self {
        self.width = Some(w);
        self
    }

    /// Uniform margin on all sides (CSS shorthand string, e.g. `"5px 10px"`).
    pub fn margin(mut self, m: impl Into<String>) -> Self {
        self.margin = Some(m.into());
        self
    }

    pub fn margin_top(mut self, v: u32) -> Self {
        self.margin_top = Some(v);
        self
    }
    pub fn margin_right(mut self, v: u32) -> Self {
        self.margin_right = Some(v);
        self
    }
    pub fn margin_bottom(mut self, v: u32) -> Self {
        self.margin_bottom = Some(v);
        self
    }
    pub fn margin_left(mut self, v: u32) -> Self {
        self.margin_left = Some(v);
        self
    }

    /// Gap (px) between modules.
    pub fn spacing(mut self, s: u32) -> Self {
        self.spacing = Some(s);
        self
    }

    /// Whether the bar reserves space on screen (exclusive zone).
    pub fn exclusive(mut self, v: bool) -> Self {
        self.exclusive = Some(v);
        self
    }

    /// Whether input events pass through the bar to the layer below.
    pub fn passthrough(mut self, v: bool) -> Self {
        self.passthrough = Some(v);
        self
    }

    /// Start the bar hidden (toggle with `waybar -t`).
    pub fn start_hidden(mut self, v: bool) -> Self {
        self.start_hidden = Some(v);
        self
    }

    /// Automatically reload the CSS style when the file changes.
    pub fn reload_style_on_change(mut self, v: bool) -> Self {
        self.reload_style_on_change = Some(v);
        self
    }

    // ── Module slot builders ──────────────────────────────────────────────

    /// Add a module to the **left** slot.
    pub fn add_left<M: WaybarModule + 'static>(mut self, module: M) -> Self {
        self.modules_left.push(module.module_id().to_string());
        self.modules.push(Box::new(module));
        self
    }

    /// Add a module to the **center** slot.
    pub fn add_center<M: WaybarModule + 'static>(mut self, module: M) -> Self {
        self.modules_center.push(module.module_id().to_string());
        self.modules.push(Box::new(module));
        self
    }

    /// Add a module to the **right** slot.
    pub fn add_right<M: WaybarModule + 'static>(mut self, module: M) -> Self {
        self.modules_right.push(module.module_id().to_string());
        self.modules.push(Box::new(module));
        self
    }
}

impl WaybarConfig for Bar {
    fn render(&self, ctx: &WaybarRenderContext) -> String {
        let mut entries: Vec<(String, String)> = Vec::new();

        // ── Bar-level settings ────────────────────────────────────────────
        if let Some(ref v) = self.name {
            entries.push(("name".into(), json_str(v)));
        }
        if let Some(ref v) = self.output {
            entries.push(("output".into(), json_str(v)));
        }
        if let Some(ref v) = self.layer {
            entries.push(("layer".into(), json_str(v)));
        }
        if let Some(ref v) = self.position {
            entries.push(("position".into(), json_str(v)));
        }
        if let Some(v) = self.height {
            entries.push(("height".into(), v.to_string()));
        }
        if let Some(v) = self.width {
            entries.push(("width".into(), v.to_string()));
        }
        if let Some(ref v) = self.margin {
            entries.push(("margin".into(), json_str(v)));
        }
        if let Some(v) = self.margin_top {
            entries.push(("margin-top".into(), v.to_string()));
        }
        if let Some(v) = self.margin_right {
            entries.push(("margin-right".into(), v.to_string()));
        }
        if let Some(v) = self.margin_bottom {
            entries.push(("margin-bottom".into(), v.to_string()));
        }
        if let Some(v) = self.margin_left {
            entries.push(("margin-left".into(), v.to_string()));
        }
        if let Some(v) = self.spacing {
            entries.push(("spacing".into(), v.to_string()));
        }
        if let Some(v) = self.exclusive {
            entries.push(("exclusive".into(), v.to_string()));
        }
        if let Some(v) = self.passthrough {
            entries.push(("passthrough".into(), v.to_string()));
        }
        if let Some(v) = self.start_hidden {
            entries.push(("start_hidden".into(), v.to_string()));
        }
        if let Some(v) = self.reload_style_on_change {
            entries.push(("reload_style_on_change".into(), v.to_string()));
        }

        // ── Module slot arrays ────────────────────────────────────────────
        if !self.modules_left.is_empty() {
            entries.push((
                "modules-left".into(),
                json_array_str(&self.modules_left),
            ));
        }
        if !self.modules_center.is_empty() {
            entries.push((
                "modules-center".into(),
                json_array_str(&self.modules_center),
            ));
        }
        if !self.modules_right.is_empty() {
            entries.push((
                "modules-right".into(),
                json_array_str(&self.modules_right),
            ));
        }

        // ── Per-module config objects ─────────────────────────────────────
        for module in &self.modules {
            let config = module.render_config(ctx);
            if config != "{}" {
                entries.push((module.module_id().to_string(), config));
            }
        }

        json_object(&entries, ctx)
    }

    fn validate(&self) -> Result<(), String> {
        for module in &self.modules {
            module.validate()?;
        }
        Ok(())
    }
}
