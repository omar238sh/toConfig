use super::core::{GhosttyConfig, GhosttyRenderContext};

/// Common Ghostty keybind actions.
///
/// Use [`KeybindAction::Custom`] for actions that are not listed here.
#[derive(Clone, Debug)]
pub enum KeybindAction {
    // Clipboard
    CopyToClipboard,
    PasteFromClipboard,
    CopyUrlToClipboard,

    // Window / surface management
    NewWindow,
    CloseWindow,
    NewTab,
    CloseTab,
    NextTab,
    PreviousTab,
    /// Jump to tab by 1-based index.
    GotoTab(u8),
    NewSplitRight,
    NewSplitLeft,
    NewSplitDown,
    NewSplitUp,
    CloseSurface,
    FocusSplitRight,
    FocusSplitLeft,
    FocusSplitDown,
    FocusSplitUp,
    ResizeSplitRight(u32),
    ResizeSplitLeft(u32),
    ResizeSplitDown(u32),
    ResizeSplitUp(u32),
    EqualizeSplits,

    // View
    ToggleFullscreen,
    ToggleZoom,
    ToggleTabOverview,

    // Font
    IncreaseFontSize(u32),
    DecreaseFontSize(u32),
    ResetFontSize,

    // Scroll
    ScrollUp,
    ScrollDown,
    ScrollPageUp,
    ScrollPageDown,
    ScrollToTop,
    ScrollToBottom,

    // Input
    /// Send a literal string to the PTY.
    WriteToTty(String),
    /// Send text preceded by ESC.
    Esc(String),

    /// Clear screen.
    ClearScreen,

    // Inspector
    InspectorToggle,

    /// Quit the application.
    Quit,

    /// Reload configuration.
    ReloadConfig,

    /// Any action not covered above.
    Custom(String),

    /// Explicitly unbind a key.
    Unbind,
}

impl std::fmt::Display for KeybindAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CopyToClipboard => write!(f, "copy_to_clipboard"),
            Self::PasteFromClipboard => write!(f, "paste_from_clipboard"),
            Self::CopyUrlToClipboard => write!(f, "copy_url_to_clipboard"),
            Self::NewWindow => write!(f, "new_window"),
            Self::CloseWindow => write!(f, "close_window"),
            Self::NewTab => write!(f, "new_tab"),
            Self::CloseTab => write!(f, "close_tab"),
            Self::NextTab => write!(f, "next_tab"),
            Self::PreviousTab => write!(f, "previous_tab"),
            Self::GotoTab(n) => write!(f, "goto_tab:{}", n),
            Self::NewSplitRight => write!(f, "new_split:right"),
            Self::NewSplitLeft => write!(f, "new_split:left"),
            Self::NewSplitDown => write!(f, "new_split:down"),
            Self::NewSplitUp => write!(f, "new_split:up"),
            Self::CloseSurface => write!(f, "close_surface"),
            Self::FocusSplitRight => write!(f, "goto_split:right"),
            Self::FocusSplitLeft => write!(f, "goto_split:left"),
            Self::FocusSplitDown => write!(f, "goto_split:down"),
            Self::FocusSplitUp => write!(f, "goto_split:up"),
            Self::ResizeSplitRight(n) => write!(f, "resize_split:right,{}", n),
            Self::ResizeSplitLeft(n) => write!(f, "resize_split:left,{}", n),
            Self::ResizeSplitDown(n) => write!(f, "resize_split:down,{}", n),
            Self::ResizeSplitUp(n) => write!(f, "resize_split:up,{}", n),
            Self::EqualizeSplits => write!(f, "equalize_splits"),
            Self::ToggleFullscreen => write!(f, "toggle_fullscreen"),
            Self::ToggleZoom => write!(f, "toggle_zoom"),
            Self::ToggleTabOverview => write!(f, "toggle_tab_overview"),
            Self::IncreaseFontSize(n) => write!(f, "increase_font_size:{}", n),
            Self::DecreaseFontSize(n) => write!(f, "decrease_font_size:{}", n),
            Self::ResetFontSize => write!(f, "reset_font_size"),
            Self::ScrollUp => write!(f, "scroll_up"),
            Self::ScrollDown => write!(f, "scroll_down"),
            Self::ScrollPageUp => write!(f, "scroll_page_up"),
            Self::ScrollPageDown => write!(f, "scroll_page_down"),
            Self::ScrollToTop => write!(f, "scroll_to_top"),
            Self::ScrollToBottom => write!(f, "scroll_to_bottom"),
            Self::WriteToTty(s) => write!(f, "write_to_pty:{}", s),
            Self::Esc(s) => write!(f, "esc:{}", s),
            Self::ClearScreen => write!(f, "clear_screen"),
            Self::InspectorToggle => write!(f, "inspector:toggle"),
            Self::Quit => write!(f, "quit"),
            Self::ReloadConfig => write!(f, "reload_config"),
            Self::Custom(s) => write!(f, "{}", s),
            Self::Unbind => write!(f, "unbind"),
        }
    }
}

/// A single `keybind = <trigger>=<action>` entry.
///
/// `trigger` is a `+`-separated list of modifiers and a key, e.g.
/// `"ctrl+shift+t"`. Use `global:` prefix to set a system-wide shortcut.
#[derive(Clone, Debug)]
pub struct Keybind {
    /// Key trigger, e.g. `"ctrl+c"` or `"super+shift+enter"`.
    pub trigger: String,
    pub action: KeybindAction,
    /// Make this a global (OS-level) shortcut.
    pub global: bool,
}

impl Keybind {
    pub fn new(trigger: impl Into<String>, action: KeybindAction) -> Self {
        Self {
            trigger: trigger.into(),
            action,
            global: false,
        }
    }

    pub fn global(mut self) -> Self {
        self.global = true;
        self
    }
}

impl GhosttyConfig for Keybind {
    fn render(&self, _ctx: &GhosttyRenderContext) -> String {
        let prefix = if self.global { "global:" } else { "" };
        format!("keybind = {}{}={}\n", prefix, self.trigger, self.action)
    }
}

/// A collection of [`Keybind`] entries rendered together.
#[derive(Default, Clone, Debug)]
pub struct KeybindGroup {
    pub binds: Vec<Keybind>,
}

impl KeybindGroup {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bind(mut self, trigger: impl Into<String>, action: KeybindAction) -> Self {
        self.binds.push(Keybind::new(trigger, action));
        self
    }

    pub fn global_bind(mut self, trigger: impl Into<String>, action: KeybindAction) -> Self {
        self.binds.push(Keybind::new(trigger, action).global());
        self
    }
}

impl GhosttyConfig for KeybindGroup {
    fn render(&self, ctx: &GhosttyRenderContext) -> String {
        self.binds.iter().map(|b| b.render(ctx)).collect()
    }
}
