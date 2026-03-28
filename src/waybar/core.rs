//! Core traits, rendering context, and JSON helpers for Waybar configuration.

// ── Rendering context ─────────────────────────────────────────────────────────

/// Rendering context for Waybar JSON output (tracks indentation depth).
#[derive(Debug, Clone)]
pub struct WaybarRenderContext {
    pub indent_level: usize,
    pub indent_width: usize,
}

impl Default for WaybarRenderContext {
    fn default() -> Self {
        Self {
            indent_level: 0,
            indent_width: 4,
        }
    }
}

impl WaybarRenderContext {
    pub fn indent(&self) -> String {
        " ".repeat(self.indent_level * self.indent_width)
    }

    pub fn deeper(&self) -> Self {
        Self {
            indent_level: self.indent_level + 1,
            ..self.clone()
        }
    }
}

// ── JSON helpers (crate-private) ──────────────────────────────────────────────

/// Encode a Rust `&str` as a JSON string literal (with quotes and escaping).
pub(crate) fn json_str(s: &str) -> String {
    let escaped = s
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t");
    format!("\"{}\"", escaped)
}

/// Render a list of strings as a JSON array of string literals: `["a", "b"]`.
pub(crate) fn json_array_str(items: &[String]) -> String {
    let quoted: Vec<String> = items.iter().map(|s| json_str(s)).collect();
    format!("[{}]", quoted.join(", "))
}

/// Render a list of already-serialised JSON values as a JSON array: `[1, true, "x"]`.
pub(crate) fn json_array_raw(items: &[String]) -> String {
    format!("[{}]", items.join(", "))
}

/// Render key-value pairs as an indented JSON object.
///
/// Each value must already be a valid JSON literal (number, bool, string, array, object …).
pub(crate) fn json_object(entries: &[(String, String)], ctx: &WaybarRenderContext) -> String {
    if entries.is_empty() {
        return "{}".to_string();
    }
    let deeper = ctx.deeper();
    let inner: Vec<String> = entries
        .iter()
        .map(|(k, v)| format!("{}{}: {}", deeper.indent(), json_str(k), v))
        .collect();
    format!("{{\n{}\n{}}}", inner.join(",\n"), ctx.indent())
}

// ── Traits ────────────────────────────────────────────────────────────────────

/// Central trait for top-level Waybar configuration nodes (bars, trees).
pub trait WaybarConfig {
    /// Render this node as a JSON string using the given context.
    fn render(&self, ctx: &WaybarRenderContext) -> String;

    /// Convenience: render with the default (zero-indent) context.
    fn generate(&self) -> String {
        self.render(&WaybarRenderContext::default())
    }

    /// Optional pre-render validation hook.
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Trait for individual Waybar modules (clock, battery, network, …).
///
/// Each module provides:
/// - [`WaybarModule::module_id`] — the identifier placed in `modules-left/center/right`.
/// - [`WaybarModule::render_config`] — the module's configuration as a JSON object body.
pub trait WaybarModule {
    /// Full module identifier, including optional instance suffix
    /// (e.g. `"clock"`, `"battery#laptop"`, `"hyprland/workspaces"`).
    fn module_id(&self) -> &str;

    /// Render this module's settings as a JSON object `{ … }`.
    /// Return `"{}"` (empty object) when the module has no custom settings.
    fn render_config(&self, ctx: &WaybarRenderContext) -> String;

    /// Optional pre-render validation hook.
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

// ── WaybarConfigTree ──────────────────────────────────────────────────────────

/// Collection of one or more bar configurations.
///
/// A single bar is rendered as a JSON object; multiple bars are rendered as a
/// JSON array (waybar supports that format for multi-monitor setups).
pub struct WaybarConfigTree {
    pub bars: Vec<Box<dyn WaybarConfig>>,
}

impl Default for WaybarConfigTree {
    fn default() -> Self {
        Self::new()
    }
}

impl WaybarConfigTree {
    pub fn new() -> Self {
        Self { bars: Vec::new() }
    }

    /// Append a bar (mutable borrow, returns `&mut Self` for chaining).
    pub fn add<C: WaybarConfig + 'static>(&mut self, bar: C) -> &mut Self {
        self.bars.push(Box::new(bar));
        self
    }

    /// Append a bar (consuming builder style).
    pub fn push<C: WaybarConfig + 'static>(mut self, bar: C) -> Self {
        self.bars.push(Box::new(bar));
        self
    }

    /// Run [`WaybarConfig::validate`] on every bar and collect all errors.
    pub fn validate_all(&self) -> Result<(), Vec<String>> {
        let errors: Vec<String> = self
            .bars
            .iter()
            .filter_map(|b| b.validate().err())
            .collect();
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl WaybarConfig for WaybarConfigTree {
    fn render(&self, ctx: &WaybarRenderContext) -> String {
        match self.bars.len() {
            0 => "[]".to_string(),
            1 => self.bars[0].render(ctx),
            _ => {
                let deeper = ctx.deeper();
                let rendered: Vec<String> =
                    self.bars.iter().map(|b| b.render(&deeper)).collect();
                format!("[\n{}\n{}]", rendered.join(",\n"), ctx.indent())
            }
        }
    }
}
