use super::{KittyConfig, KittyRenderContext};

/// A single keyboard shortcut mapping in kitty.
///
/// Rendered as: `map <mods+key> <action>`
///
/// # Example
/// ```
/// use toconfig::kitty::KittyConfig;
/// use toconfig::kitty::keybind::KittyKeybind;
/// let k = KittyKeybind::new("ctrl+shift+c", "copy_to_clipboard");
/// assert_eq!(k.generate(), "map ctrl+shift+c copy_to_clipboard");
/// ```
pub struct KittyKeybind {
    /// The key combination, e.g. `"ctrl+shift+c"`.
    pub keys: String,
    /// The action to execute, e.g. `"copy_to_clipboard"`.
    pub action: String,
}

impl KittyKeybind {
    pub fn new(keys: impl Into<String>, action: impl Into<String>) -> Self {
        Self {
            keys: keys.into(),
            action: action.into(),
        }
    }

    // ── Common clipboard shortcuts ────────────────────────────────────────────

    /// `ctrl+shift+c → copy_to_clipboard`
    pub fn copy() -> Self {
        Self::new("ctrl+shift+c", "copy_to_clipboard")
    }

    /// `ctrl+shift+v → paste_from_clipboard`
    pub fn paste() -> Self {
        Self::new("ctrl+shift+v", "paste_from_clipboard")
    }

    // ── Common tab shortcuts ──────────────────────────────────────────────────

    /// `ctrl+shift+t → new_tab`
    pub fn new_tab() -> Self {
        Self::new("ctrl+shift+t", "new_tab")
    }

    /// `ctrl+shift+q → close_tab`
    pub fn close_tab() -> Self {
        Self::new("ctrl+shift+q", "close_tab")
    }

    /// `ctrl+shift+right → next_tab`
    pub fn next_tab() -> Self {
        Self::new("ctrl+shift+right", "next_tab")
    }

    /// `ctrl+shift+left → previous_tab`
    pub fn prev_tab() -> Self {
        Self::new("ctrl+shift+left", "previous_tab")
    }

    // ── Common window shortcuts ───────────────────────────────────────────────

    /// `ctrl+shift+enter → new_window`
    pub fn new_window() -> Self {
        Self::new("ctrl+shift+enter", "new_window")
    }

    /// `ctrl+shift+w → close_window`
    pub fn close_window() -> Self {
        Self::new("ctrl+shift+w", "close_window")
    }

    // ── Font size shortcuts ───────────────────────────────────────────────────

    /// `ctrl+shift+equal → change_font_size all +2.0`
    pub fn font_size_increase() -> Self {
        Self::new("ctrl+shift+equal", "change_font_size all +2.0")
    }

    /// `ctrl+shift+minus → change_font_size all -2.0`
    pub fn font_size_decrease() -> Self {
        Self::new("ctrl+shift+minus", "change_font_size all -2.0")
    }

    /// `ctrl+shift+backspace → change_font_size all 0`
    pub fn font_size_reset() -> Self {
        Self::new("ctrl+shift+backspace", "change_font_size all 0")
    }
}

impl KittyConfig for KittyKeybind {
    fn render(&self, ctx: &KittyRenderContext) -> String {
        format!("{}map {} {}", ctx.indent(), self.keys, self.action)
    }

    fn validate(&self) -> Result<(), String> {
        if self.keys.is_empty() {
            return Err("KittyKeybind: key combination cannot be empty".into());
        }
        if self.action.is_empty() {
            return Err("KittyKeybind: action cannot be empty".into());
        }
        Ok(())
    }
}

/// An `unmap` directive that removes a default kitty keybind.
///
/// Rendered as: `map <keys> no-op`
///
/// # Example
/// ```
/// use toconfig::kitty::KittyConfig;
/// use toconfig::kitty::keybind::KittyUnmap;
/// let u = KittyUnmap::new("ctrl+shift+z");
/// assert_eq!(u.generate(), "map ctrl+shift+z no-op");
/// ```
pub struct KittyUnmap {
    pub keys: String,
}

impl KittyUnmap {
    pub fn new(keys: impl Into<String>) -> Self {
        Self { keys: keys.into() }
    }
}

impl KittyConfig for KittyUnmap {
    fn render(&self, ctx: &KittyRenderContext) -> String {
        format!("{}map {} no-op", ctx.indent(), self.keys)
    }
}
