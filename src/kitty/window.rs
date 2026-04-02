use super::{KittyConfig, KittyRenderContext};

/// Window and layout configuration for kitty terminal.
///
/// Controls the initial size, padding, border, background opacity, and other
/// windowing properties.
///
/// # Example
/// ```
/// use toconfig::kitty::KittyConfig;
/// use toconfig::kitty::window::WindowConfig;
/// let w = WindowConfig::new()
///     .remember_size(false)
///     .initial_width(1280)
///     .initial_height(800)
///     .padding_width(8)
///     .background_opacity(0.95);
/// let out = w.generate();
/// assert!(out.contains("initial_window_width"));
/// assert!(out.contains("background_opacity"));
/// ```
#[derive(Default)]
pub struct WindowConfig {
    /// Whether kitty remembers the previous window size on restart.
    pub remember_size: Option<bool>,
    /// Initial window width in pixels (used when `remember_size` is `false`).
    pub initial_width: Option<u32>,
    /// Initial window height in pixels (used when `remember_size` is `false`).
    pub initial_height: Option<u32>,
    /// Padding on each side of the window content area (pixels).
    pub padding_width: Option<u32>,
    /// Separate horizontal padding (overrides `padding_width`).
    pub padding_h: Option<u32>,
    /// Separate vertical padding (overrides `padding_width`).
    pub padding_v: Option<u32>,
    /// Whether to draw a border around windows: `"yes"` | `"no"` | `"tiled"`.
    pub draw_minimal_borders: Option<bool>,
    /// Hide window title bar and decorations: `"yes"` | `"no"` | `"titlebar-only"`.
    pub hide_window_decorations: Option<String>,
    /// Background opacity (0.0–1.0; requires compositor support).
    pub background_opacity: Option<f32>,
    /// Dynamic background opacity controlled by the shell: `"yes"` | `"no"`.
    pub dynamic_background_opacity: Option<bool>,
    /// Dim opacity for inactive windows (0.0 = fully dim, 1.0 = no dimming).
    pub inactive_text_alpha: Option<f32>,
    /// Window placement strategy: `"center"` | `"top-left"`.
    pub placement_strategy: Option<String>,
    /// Minimum padding to keep around the content (pixels).
    pub single_window_margin_width: Option<i32>,
    /// Width of the border between tiled windows (pixels).
    pub window_border_width: Option<u32>,
    /// Colour of active border (e.g. `"#00ff00"`).
    pub active_border_color: Option<String>,
    /// Colour of inactive border.
    pub inactive_border_color: Option<String>,
    /// Colour of the bell border.
    pub bell_border_color: Option<String>,
    /// Number of tab columns to leave for the OS window buttons.
    pub macos_titlebar_color: Option<String>,
    /// Resize window increments in cells.
    pub resize_in_steps: Option<bool>,
    /// Confirm closing window when there are running processes: `"yes"` | `"no"`.
    pub confirm_os_window_close: Option<i32>,
}

impl WindowConfig {
    pub fn new() -> Self {
        Self::default()
    }

    /// Whether to remember the window size between sessions.
    pub fn remember_size(mut self, v: bool) -> Self {
        self.remember_size = Some(v);
        self
    }

    /// Initial window width in pixels.
    pub fn initial_width(mut self, v: u32) -> Self {
        self.initial_width = Some(v);
        self
    }

    /// Initial window height in pixels.
    pub fn initial_height(mut self, v: u32) -> Self {
        self.initial_height = Some(v);
        self
    }

    /// Uniform padding around the content area in pixels.
    pub fn padding_width(mut self, v: u32) -> Self {
        self.padding_width = Some(v);
        self
    }

    /// Separate horizontal padding in pixels.
    pub fn padding_h(mut self, v: u32) -> Self {
        self.padding_h = Some(v);
        self
    }

    /// Separate vertical padding in pixels.
    pub fn padding_v(mut self, v: u32) -> Self {
        self.padding_v = Some(v);
        self
    }

    /// Draw minimal borders between tiled windows.
    pub fn draw_minimal_borders(mut self, v: bool) -> Self {
        self.draw_minimal_borders = Some(v);
        self
    }

    /// Hide window decorations: `"yes"` | `"no"` | `"titlebar-only"`.
    pub fn hide_window_decorations(mut self, v: impl Into<String>) -> Self {
        self.hide_window_decorations = Some(v.into());
        self
    }

    /// Background opacity (0.0–1.0).
    pub fn background_opacity(mut self, v: f32) -> Self {
        self.background_opacity = Some(v);
        self
    }

    /// Allow per-shell dynamic background opacity changes.
    pub fn dynamic_background_opacity(mut self, v: bool) -> Self {
        self.dynamic_background_opacity = Some(v);
        self
    }

    /// Dim text in inactive windows (1.0 = normal).
    pub fn inactive_text_alpha(mut self, v: f32) -> Self {
        self.inactive_text_alpha = Some(v);
        self
    }

    /// Placement strategy: `"center"` | `"top-left"`.
    pub fn placement_strategy(mut self, v: impl Into<String>) -> Self {
        self.placement_strategy = Some(v.into());
        self
    }

    /// Margin to leave around a single maximized window.
    pub fn single_window_margin_width(mut self, v: i32) -> Self {
        self.single_window_margin_width = Some(v);
        self
    }

    /// Width of the border between tiled windows in pixels.
    pub fn window_border_width(mut self, v: u32) -> Self {
        self.window_border_width = Some(v);
        self
    }

    /// Active border color.
    pub fn active_border_color(mut self, v: impl Into<String>) -> Self {
        self.active_border_color = Some(v.into());
        self
    }

    /// Inactive border color.
    pub fn inactive_border_color(mut self, v: impl Into<String>) -> Self {
        self.inactive_border_color = Some(v.into());
        self
    }

    /// Bell border color.
    pub fn bell_border_color(mut self, v: impl Into<String>) -> Self {
        self.bell_border_color = Some(v.into());
        self
    }

    /// Resize in whole cell increments.
    pub fn resize_in_steps(mut self, v: bool) -> Self {
        self.resize_in_steps = Some(v);
        self
    }

    /// Number of processes that must be running before kitty asks for
    /// confirmation when closing (negative to disable the check).
    pub fn confirm_os_window_close(mut self, v: i32) -> Self {
        self.confirm_os_window_close = Some(v);
        self
    }
}

impl KittyConfig for WindowConfig {
    fn render(&self, ctx: &KittyRenderContext) -> String {
        let indent = ctx.indent();
        let mut lines: Vec<String> = Vec::new();

        if let Some(v) = self.remember_size {
            lines.push(format!(
                "{}remember_window_size {}",
                indent,
                if v { "yes" } else { "no" }
            ));
        }
        if let Some(v) = self.initial_width {
            lines.push(format!("{}initial_window_width  {}", indent, v));
        }
        if let Some(v) = self.initial_height {
            lines.push(format!("{}initial_window_height {}", indent, v));
        }
        if let Some(v) = self.padding_width {
            lines.push(format!("{}window_padding_width {}", indent, v));
        }
        if let Some(v) = self.padding_h {
            lines.push(format!("{}window_padding_width {}h", indent, v));
        }
        if let Some(v) = self.padding_v {
            lines.push(format!("{}window_padding_width {}v", indent, v));
        }
        if let Some(v) = self.draw_minimal_borders {
            lines.push(format!(
                "{}draw_minimal_borders {}",
                indent,
                if v { "yes" } else { "no" }
            ));
        }
        if let Some(ref v) = self.hide_window_decorations {
            lines.push(format!("{}hide_window_decorations {}", indent, v));
        }
        if let Some(v) = self.background_opacity {
            lines.push(format!("{}background_opacity {}", indent, v));
        }
        if let Some(v) = self.dynamic_background_opacity {
            lines.push(format!(
                "{}dynamic_background_opacity {}",
                indent,
                if v { "yes" } else { "no" }
            ));
        }
        if let Some(v) = self.inactive_text_alpha {
            lines.push(format!("{}inactive_text_alpha {}", indent, v));
        }
        if let Some(ref v) = self.placement_strategy {
            lines.push(format!("{}placement_strategy {}", indent, v));
        }
        if let Some(v) = self.single_window_margin_width {
            lines.push(format!("{}single_window_margin_width {}", indent, v));
        }
        if let Some(v) = self.window_border_width {
            lines.push(format!("{}window_border_width {}", indent, v));
        }
        if let Some(ref v) = self.active_border_color {
            lines.push(format!("{}active_border_color {}", indent, v));
        }
        if let Some(ref v) = self.inactive_border_color {
            lines.push(format!("{}inactive_border_color {}", indent, v));
        }
        if let Some(ref v) = self.bell_border_color {
            lines.push(format!("{}bell_border_color {}", indent, v));
        }
        if let Some(v) = self.resize_in_steps {
            lines.push(format!(
                "{}resize_in_steps {}",
                indent,
                if v { "yes" } else { "no" }
            ));
        }
        if let Some(v) = self.confirm_os_window_close {
            lines.push(format!("{}confirm_os_window_close {}", indent, v));
        }
        lines.join("\n")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(v) = self.background_opacity {
            if !(0.0..=1.0).contains(&v) {
                return Err(format!(
                    "WindowConfig: background_opacity must be 0.0–1.0, got {}",
                    v
                ));
            }
        }
        if let Some(v) = self.inactive_text_alpha {
            if !(0.0..=1.0).contains(&v) {
                return Err(format!(
                    "WindowConfig: inactive_text_alpha must be 0.0–1.0, got {}",
                    v
                ));
            }
        }
        Ok(())
    }
}
