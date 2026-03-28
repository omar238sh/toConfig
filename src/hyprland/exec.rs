use super::{HyprlandConfig, HyprlandRenderContext};

/// Startup command executed **once** when Hyprland starts.
///
/// Rendered as: `exec-once = command`
///
/// Use this to launch status bars, notification daemons, polkit agents, etc.
/// For commands that should restart on every config reload use [`Exec`] instead.
///
/// # Example
/// ```
/// # use toconfig::hyprland::HyprlandConfig;
/// use toconfig::hyprland::exec::ExecOnce;
/// assert_eq!(ExecOnce::new("waybar").generate(), "exec-once = waybar");
/// ```
pub struct ExecOnce {
    pub command: String,
}

impl ExecOnce {
    pub fn new(command: impl Into<String>) -> Self {
        Self {
            command: command.into(),
        }
    }

    /// Convenience: run a `hyprctl dispatch` command on startup.
    ///
    /// # Example
    /// ```
    /// # use toconfig::hyprland::HyprlandConfig;
    /// use toconfig::hyprland::exec::ExecOnce;
    /// let e = ExecOnce::hyprctl_dispatch("workspace 1");
    /// assert_eq!(e.generate(), "exec-once = hyprctl dispatch workspace 1");
    /// ```
    pub fn hyprctl_dispatch(dispatch: impl Into<String>) -> Self {
        Self::new(format!("hyprctl dispatch {}", dispatch.into()))
    }
}

impl HyprlandConfig for ExecOnce {
    fn render(&self, ctx: &HyprlandRenderContext) -> String {
        format!("{}exec-once = {}", ctx.indent(), self.command)
    }
}

/// Command executed every time the Hyprland config is reloaded.
///
/// Rendered as: `exec = command`
///
/// Suitable for things like setting the wallpaper, which should refresh on reload.
///
/// # Example
/// ```
/// # use toconfig::hyprland::HyprlandConfig;
/// use toconfig::hyprland::exec::Exec;
/// assert_eq!(
///     Exec::new("swww img ~/wallpaper.png").generate(),
///     "exec = swww img ~/wallpaper.png"
/// );
/// ```
pub struct Exec {
    pub command: String,
}

impl Exec {
    pub fn new(command: impl Into<String>) -> Self {
        Self {
            command: command.into(),
        }
    }
}

impl HyprlandConfig for Exec {
    fn render(&self, ctx: &HyprlandRenderContext) -> String {
        format!("{}exec = {}", ctx.indent(), self.command)
    }
}

/// Load a Hyprland plugin (`*.so` shared library).
///
/// Rendered as: `plugin = /path/to/plugin.so`
///
/// Plugins extend Hyprland with custom layouts, dispatchers, and more.
/// Examples include `hyprscroller` (scrolling layout), `hyprexpo` (overview),
/// and `hycov` (monocle/overview).
///
/// # Example
/// ```
/// # use toconfig::hyprland::HyprlandConfig;
/// use toconfig::hyprland::exec::PluginLoad;
/// let p = PluginLoad::new("/usr/lib/hyprland/hyprscroller.so");
/// assert_eq!(p.generate(), "plugin = /usr/lib/hyprland/hyprscroller.so");
/// ```
pub struct PluginLoad {
    pub path: String,
}

impl PluginLoad {
    pub fn new(path: impl Into<String>) -> Self {
        Self { path: path.into() }
    }
}

impl HyprlandConfig for PluginLoad {
    fn render(&self, ctx: &HyprlandRenderContext) -> String {
        format!("{}plugin = {}", ctx.indent(), self.path)
    }

    fn validate(&self) -> Result<(), String> {
        if self.path.is_empty() {
            return Err("PluginLoad path cannot be empty".into());
        }
        Ok(())
    }
}

/// Include (source) another config file.
///
/// Rendered as: `source = path`
///
/// Useful for splitting a large config into multiple files, e.g. separating
/// keybinds, window rules, or per-machine overrides.
///
/// # Example
/// ```
/// # use toconfig::hyprland::HyprlandConfig;
/// use toconfig::hyprland::exec::Source;
/// assert_eq!(
///     Source::new("~/.config/hypr/keybinds.conf").generate(),
///     "source = ~/.config/hypr/keybinds.conf"
/// );
/// ```
pub struct Source {
    pub path: String,
}

impl Source {
    pub fn new(path: impl Into<String>) -> Self {
        Self { path: path.into() }
    }
}

impl HyprlandConfig for Source {
    fn render(&self, ctx: &HyprlandRenderContext) -> String {
        format!("{}source = {}", ctx.indent(), self.path)
    }

    fn validate(&self) -> Result<(), String> {
        if self.path.is_empty() {
            return Err("Source path cannot be empty".into());
        }
        Ok(())
    }
}
