use super::{KittyConfig, KittyRenderContext};

/// URL style variants for underline decoration on hyperlinks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UrlStyle {
    None,
    Straight,
    Double,
    Curly,
    Dotted,
    Dashed,
}

impl UrlStyle {
    fn as_str(self) -> &'static str {
        match self {
            UrlStyle::None => "none",
            UrlStyle::Straight => "straight",
            UrlStyle::Double => "double",
            UrlStyle::Curly => "curly",
            UrlStyle::Dotted => "dotted",
            UrlStyle::Dashed => "dashed",
        }
    }
}

/// General / miscellaneous kitty settings that don't fit a dedicated section.
///
/// Covers shell, editor, URL handling, clipboard, mouse behavior, bell,
/// Linux IME, and OS-integration options.
///
/// # Example
/// ```
/// use toconfig::kitty::KittyConfig;
/// use toconfig::kitty::general::{GeneralConfig, UrlStyle};
/// let g = GeneralConfig::new()
///     .shell("/bin/fish")
///     .editor("nvim")
///     .url_style(UrlStyle::Curly)
///     .copy_on_select(true);
/// let out = g.generate();
/// assert!(out.contains("shell /bin/fish"));
/// assert!(out.contains("editor nvim"));
/// ```
#[derive(Default)]
pub struct GeneralConfig {
    /// Shell program to run (`"."`  = use the default system shell).
    pub shell: Option<String>,
    /// Editor program launched by kitty (`"."` = use `$EDITOR`).
    pub editor: Option<String>,
    /// Close the kitty window when the child process exits: `"window"` | `"tab"` | `"os-window"` | `"no"`.
    pub close_on_child_death: Option<String>,
    /// URL underline style.
    pub url_style: Option<UrlStyle>,
    /// Open URLs on click: `"yes"` | `"no"`.
    pub open_url_with: Option<String>,
    /// URL prefixes recognized as links.
    pub url_prefixes: Option<String>,
    /// Detect URLs even when not confirmed with a protocol prefix.
    pub detect_urls: Option<bool>,
    /// Copy selected text to clipboard automatically: `"yes"` | `"no"` | `"clipboard"`.
    pub copy_on_select: Option<bool>,
    /// Strip trailing spaces when copying: `"never"` | `"smart"` | `"always"`.
    pub strip_trailing_spaces: Option<String>,
    /// Shape of the mouse pointer when hovering text: kitty pointer name.
    pub pointer_shape_when_grabbed: Option<String>,
    /// Default pointer shape.
    pub default_pointer_shape: Option<String>,
    /// Hide mouse cursor after N seconds of inactivity (`0` = never).
    pub mouse_hide_wait: Option<f32>,
    /// Allow remote control via a socket: `"yes"` | `"no"` | `"socket"` | `"socket-only"`.
    pub allow_remote_control: Option<String>,
    /// Listen on a Unix socket for the kitty remote control protocol.
    pub listen_on: Option<String>,
    /// Environment variables to set (key=value pairs accumulated).
    pub env: Vec<(String, String)>,
    /// Additional paths to search for kitty configs.
    pub kitty_mod: Option<String>,
    /// Enable/disable the visual bell.
    pub visual_bell_duration: Option<f32>,
    /// Window alert on bell: `"yes"` | `"no"`.
    pub window_alert_on_bell: Option<bool>,
    /// Bell on tab: `"yes"` | `"no"`.
    pub bell_on_tab: Option<String>,
    /// Linux Input Method support.
    pub linux_display_server: Option<String>,
    /// Wayland titlebar color.
    pub wayland_titlebar_color: Option<String>,
    /// macOS option key as alt: `"left"` | `"right"` | `"both"` | `"no"`.
    pub macos_option_as_alt: Option<String>,
    /// macOS quit behavior: `"last-window"` | `"never"`.
    pub macos_quit_when_last_window_closed: Option<bool>,
    /// macOS window resizing: `"yes"` | `"no"`.
    pub macos_window_resizable: Option<bool>,
    /// macOS colorspace: `"srgb"` | `"default"` | `"displayp3"`.
    pub macos_colorspace: Option<String>,
}

impl GeneralConfig {
    pub fn new() -> Self {
        Self::default()
    }

    /// Shell program to launch (`.` = system default).
    pub fn shell(mut self, v: impl Into<String>) -> Self {
        self.shell = Some(v.into());
        self
    }

    /// Editor program (`.` = use `$EDITOR`).
    pub fn editor(mut self, v: impl Into<String>) -> Self {
        self.editor = Some(v.into());
        self
    }

    /// Close behavior when the child exits.
    pub fn close_on_child_death(mut self, v: impl Into<String>) -> Self {
        self.close_on_child_death = Some(v.into());
        self
    }

    /// URL underline style.
    pub fn url_style(mut self, v: UrlStyle) -> Self {
        self.url_style = Some(v);
        self
    }

    /// Application used to open URLs.
    pub fn open_url_with(mut self, v: impl Into<String>) -> Self {
        self.open_url_with = Some(v.into());
        self
    }

    /// URL prefixes (space-separated), e.g. `"http https ftp"`.
    pub fn url_prefixes(mut self, v: impl Into<String>) -> Self {
        self.url_prefixes = Some(v.into());
        self
    }

    /// Automatically detect URLs in the terminal output.
    pub fn detect_urls(mut self, v: bool) -> Self {
        self.detect_urls = Some(v);
        self
    }

    /// Copy selected text to primary selection automatically.
    pub fn copy_on_select(mut self, v: bool) -> Self {
        self.copy_on_select = Some(v);
        self
    }

    /// Strip trailing spaces: `"never"` | `"smart"` | `"always"`.
    pub fn strip_trailing_spaces(mut self, v: impl Into<String>) -> Self {
        self.strip_trailing_spaces = Some(v.into());
        self
    }

    /// Hide mouse pointer after N seconds of inactivity.
    pub fn mouse_hide_wait(mut self, v: f32) -> Self {
        self.mouse_hide_wait = Some(v);
        self
    }

    /// Allow remote control connections.
    pub fn allow_remote_control(mut self, v: impl Into<String>) -> Self {
        self.allow_remote_control = Some(v.into());
        self
    }

    /// Listen socket path for remote control (e.g. `"unix:/tmp/kitty.sock"`).
    pub fn listen_on(mut self, v: impl Into<String>) -> Self {
        self.listen_on = Some(v.into());
        self
    }

    /// Add an environment variable to inject into child processes.
    pub fn env(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.env.push((key.into(), value.into()));
        self
    }

    /// Modifier key used as kitty's "super" modifier in default shortcuts.
    pub fn kitty_mod(mut self, v: impl Into<String>) -> Self {
        self.kitty_mod = Some(v.into());
        self
    }

    /// Duration of the visual bell flash in seconds (`0` = disabled).
    pub fn visual_bell_duration(mut self, v: f32) -> Self {
        self.visual_bell_duration = Some(v);
        self
    }

    /// Flash the OS window title bar on bell.
    pub fn window_alert_on_bell(mut self, v: bool) -> Self {
        self.window_alert_on_bell = Some(v);
        self
    }

    /// Show a bell indicator in the tab title when a bell fires.
    pub fn bell_on_tab(mut self, v: impl Into<String>) -> Self {
        self.bell_on_tab = Some(v.into());
        self
    }

    /// Linux display server: `"auto"` | `"x11"` | `"wayland"`.
    pub fn linux_display_server(mut self, v: impl Into<String>) -> Self {
        self.linux_display_server = Some(v.into());
        self
    }

    /// Wayland decoration title bar color.
    pub fn wayland_titlebar_color(mut self, v: impl Into<String>) -> Self {
        self.wayland_titlebar_color = Some(v.into());
        self
    }

    /// macOS: treat the option key as Alt.
    pub fn macos_option_as_alt(mut self, v: impl Into<String>) -> Self {
        self.macos_option_as_alt = Some(v.into());
        self
    }

    /// macOS: quit kitty when the last window is closed.
    pub fn macos_quit_when_last_window_closed(mut self, v: bool) -> Self {
        self.macos_quit_when_last_window_closed = Some(v);
        self
    }

    /// macOS: allow the OS window to be resized.
    pub fn macos_window_resizable(mut self, v: bool) -> Self {
        self.macos_window_resizable = Some(v);
        self
    }

    /// macOS colorspace: `"srgb"` | `"default"` | `"displayp3"`.
    pub fn macos_colorspace(mut self, v: impl Into<String>) -> Self {
        self.macos_colorspace = Some(v.into());
        self
    }
}

impl KittyConfig for GeneralConfig {
    fn render(&self, ctx: &KittyRenderContext) -> String {
        let indent = ctx.indent();
        let mut lines: Vec<String> = Vec::new();
        if let Some(ref v) = self.shell {
            lines.push(format!("{}shell {}", indent, v));
        }
        if let Some(ref v) = self.editor {
            lines.push(format!("{}editor {}", indent, v));
        }
        if let Some(ref v) = self.close_on_child_death {
            lines.push(format!("{}close_on_child_death {}", indent, v));
        }
        if let Some(v) = self.url_style {
            lines.push(format!("{}url_style {}", indent, v.as_str()));
        }
        if let Some(ref v) = self.open_url_with {
            lines.push(format!("{}open_url_with {}", indent, v));
        }
        if let Some(ref v) = self.url_prefixes {
            lines.push(format!("{}url_prefixes {}", indent, v));
        }
        if let Some(v) = self.detect_urls {
            lines.push(format!(
                "{}detect_urls {}",
                indent,
                if v { "yes" } else { "no" }
            ));
        }
        if let Some(v) = self.copy_on_select {
            lines.push(format!(
                "{}copy_on_select {}",
                indent,
                if v { "yes" } else { "no" }
            ));
        }
        if let Some(ref v) = self.strip_trailing_spaces {
            lines.push(format!("{}strip_trailing_spaces {}", indent, v));
        }
        if let Some(v) = self.mouse_hide_wait {
            lines.push(format!("{}mouse_hide_wait {}", indent, v));
        }
        if let Some(ref v) = self.allow_remote_control {
            lines.push(format!("{}allow_remote_control {}", indent, v));
        }
        if let Some(ref v) = self.listen_on {
            lines.push(format!("{}listen_on {}", indent, v));
        }
        for (k, v) in &self.env {
            lines.push(format!("{}env {}={}", indent, k, v));
        }
        if let Some(ref v) = self.kitty_mod {
            lines.push(format!("{}kitty_mod {}", indent, v));
        }
        if let Some(v) = self.visual_bell_duration {
            lines.push(format!("{}visual_bell_duration {}", indent, v));
        }
        if let Some(v) = self.window_alert_on_bell {
            lines.push(format!(
                "{}window_alert_on_bell {}",
                indent,
                if v { "yes" } else { "no" }
            ));
        }
        if let Some(ref v) = self.bell_on_tab {
            lines.push(format!("{}bell_on_tab {}", indent, v));
        }
        if let Some(ref v) = self.linux_display_server {
            lines.push(format!("{}linux_display_server {}", indent, v));
        }
        if let Some(ref v) = self.wayland_titlebar_color {
            lines.push(format!("{}wayland_titlebar_color {}", indent, v));
        }
        if let Some(ref v) = self.macos_option_as_alt {
            lines.push(format!("{}macos_option_as_alt {}", indent, v));
        }
        if let Some(v) = self.macos_quit_when_last_window_closed {
            lines.push(format!(
                "{}macos_quit_when_last_window_closed {}",
                indent,
                if v { "yes" } else { "no" }
            ));
        }
        if let Some(v) = self.macos_window_resizable {
            lines.push(format!(
                "{}macos_window_resizable {}",
                indent,
                if v { "yes" } else { "no" }
            ));
        }
        if let Some(ref v) = self.macos_colorspace {
            lines.push(format!("{}macos_colorspace {}", indent, v));
        }
        lines.join("\n")
    }
}
