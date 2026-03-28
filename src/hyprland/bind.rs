use super::{HyprlandConfig, HyprlandRenderContext};

/// Hyprland dispatcher commands used as the action in [`Bind`] rules.
///
/// The dispatcher string is the third field in a bind line:
/// `bind = MOD, key, <dispatcher>[, params]`
///
/// See the Hyprland wiki "Dispatchers" page for the full reference.
#[derive(Debug, Clone)]
pub enum Dispatcher {
    /// Run a shell command: `exec, $terminal`
    Exec(String),
    /// Kill the focused window gracefully.
    KillActive,
    /// Force-close the focused window.
    ForceCloseActive,
    /// Toggle floating mode for the focused window.
    ToggleFloating,
    /// Toggle fullscreen. 0 = real fullscreen, 1 = maximised, 2 = fullscreen (no bar).
    FullScreen(u8),
    /// Toggle fake-fullscreen (window thinks it is fullscreen but is not).
    FakeFullScreen,
    /// Toggle pseudo-tiling (dwindle layout).
    TogglePseudo,
    /// Toggle the split direction (dwindle layout).
    ToggleSplit,
    /// Move keyboard focus: `l`, `r`, `u`, `d`.
    MoveFocus(String),
    /// Swap the focused window with a neighbour: `l`, `r`, `u`, `d`.
    SwapWindow(String),
    /// Move the focused window in a direction: `l`, `r`, `u`, `d`.
    MoveWindow(String),
    /// Resize the active window. Argument examples: `exact 800 600`, `100 0`.
    ResizeActive(String),
    /// Switch to a workspace by id or name, e.g. `"1"`, `"name:web"`, `"+1"`.
    Workspace(String),
    /// Move the focused window to a workspace (stays on current workspace).
    MoveToWorkspace(String),
    /// Move the focused window to a workspace silently (no focus switch).
    MoveToWorkspaceSilent(String),
    /// Toggle the named special workspace, or the default one if `None`.
    ToggleSpecialWorkspace(Option<String>),
    /// Move the active window to a special workspace.
    MoveToSpecialWorkspace(Option<String>),
    /// Move the current workspace to a monitor by name.
    MoveCurrentWorkspaceToMonitor(String),
    /// Change the dwindle split ratio by a floating delta, e.g. `"+0.1"`.
    SplitRatio(String),
    /// Focus the next window on the current workspace.
    CycleNext,
    /// Focus the previous window on the current workspace.
    CyclePrev,
    /// Swap with the next window.
    SwapNext,
    /// Pin the active window (float it across all workspaces).
    Pin,
    /// Pass the keybind through to the next matching bind.
    Pass(String),
    /// Any dispatcher not covered above. `(name, optional_params)`.
    Custom(String, Option<String>),
}

impl Dispatcher {
    pub fn to_dispatch_str(&self) -> String {
        match self {
            Dispatcher::Exec(cmd) => format!("exec, {}", cmd),
            Dispatcher::KillActive => "killactive".into(),
            Dispatcher::ForceCloseActive => "forcecloseactive".into(),
            Dispatcher::ToggleFloating => "togglefloating".into(),
            Dispatcher::FullScreen(n) => format!("fullscreen, {}", n),
            Dispatcher::FakeFullScreen => "fakefullscreen".into(),
            Dispatcher::TogglePseudo => "pseudo".into(),
            Dispatcher::ToggleSplit => "togglesplit".into(),
            Dispatcher::MoveFocus(d) => format!("movefocus, {}", d),
            Dispatcher::SwapWindow(d) => format!("swapwindow, {}", d),
            Dispatcher::MoveWindow(d) => format!("movewindow, {}", d),
            Dispatcher::ResizeActive(args) => format!("resizeactive, {}", args),
            Dispatcher::Workspace(id) => format!("workspace, {}", id),
            Dispatcher::MoveToWorkspace(id) => format!("movetoworkspace, {}", id),
            Dispatcher::MoveToWorkspaceSilent(id) => format!("movetoworkspacesilent, {}", id),
            Dispatcher::ToggleSpecialWorkspace(name) => match name {
                Some(n) => format!("togglespecialworkspace, {}", n),
                None => "togglespecialworkspace".into(),
            },
            Dispatcher::MoveToSpecialWorkspace(name) => match name {
                Some(n) => format!("movetospecialworkspace, {}", n),
                None => "movetospecialworkspace".into(),
            },
            Dispatcher::MoveCurrentWorkspaceToMonitor(mon) => {
                format!("movecurrentworkspacetomonitor, {}", mon)
            }
            Dispatcher::SplitRatio(delta) => format!("splitratio, {}", delta),
            Dispatcher::CycleNext => "cyclenext".into(),
            Dispatcher::CyclePrev => "cyclenext, prev".into(),
            Dispatcher::SwapNext => "swapnext".into(),
            Dispatcher::Pin => "pin".into(),
            Dispatcher::Pass(name) => format!("pass, {}", name),
            Dispatcher::Custom(name, params) => match params {
                Some(p) => format!("{}, {}", name, p),
                None => name.clone(),
            },
        }
    }
}

/// Modifier flags that change bind behaviour.
#[derive(Debug, Clone, Default)]
pub struct BindFlags {
    /// Works even when the session is locked.
    pub locked: bool,
    /// Trigger on key **release** rather than press.
    pub release: bool,
    /// Repeat the action while the key is held.
    pub repeat: bool,
    /// Non-consuming: the key event is also passed to the focused window.
    pub non_consuming: bool,
    /// Transparent: passes through even if window accepts the key.
    pub transparent: bool,
    /// Inhibit IME while the key is held.
    pub inhibit_ime: bool,
    /// Mouse button binding — switches the keyword from `bind` to `bindm`.
    pub mouse: bool,
}

impl BindFlags {
    fn flag_str(&self) -> String {
        let mut s = String::new();
        if self.locked {
            s.push('l');
        }
        if self.release {
            s.push('r');
        }
        if self.repeat {
            s.push('e');
        }
        if self.non_consuming {
            s.push('n');
        }
        if self.transparent {
            s.push('t');
        }
        if self.inhibit_ime {
            s.push('i');
        }
        s
    }
}

/// A Hyprland keybind rule.
///
/// Rendered as: `bind[flags] = MODIFIERS, key, dispatcher[, params]`
///
/// For mouse bindings (move/resize window) set [`BindFlags::mouse`] via
/// [`Bind::mouse`]; the keyword becomes `bindm` automatically.
///
/// # Gestures
/// Hyprland does not model touch gestures as `bind` lines; gesture settings
/// live in the `gestures { }` section (see [`crate::hyprland::section::Section`]).
///
/// # Example
/// ```
/// # use toconfig::hyprland::HyprlandConfig;
/// use toconfig::hyprland::bind::{Bind, Dispatcher};
///
/// // Launch terminal
/// let b = Bind::new("SUPER", "Return", Dispatcher::Exec("$terminal".into()));
/// assert_eq!(b.generate(), "bind = SUPER, Return, exec, $terminal");
///
/// // Kill window
/// let k = Bind::new("SUPER", "Q", Dispatcher::KillActive);
/// assert_eq!(k.generate(), "bind = SUPER, Q, killactive");
///
/// // Move window with mouse (bindm)
/// let m = Bind::new("SUPER", "mouse:272", Dispatcher::Custom("movewindow".into(), None))
///     .mouse();
/// assert_eq!(m.generate(), "bindm = SUPER, mouse:272, movewindow");
/// ```
pub struct Bind {
    pub modifiers: String,
    pub key: String,
    pub dispatcher: Dispatcher,
    pub flags: BindFlags,
}

impl Bind {
    pub fn new(
        modifiers: impl Into<String>,
        key: impl Into<String>,
        dispatcher: Dispatcher,
    ) -> Self {
        Self {
            modifiers: modifiers.into(),
            key: key.into(),
            dispatcher,
            flags: BindFlags::default(),
        }
    }

    /// Works even when the screen is locked (`bindl`).
    pub fn locked(mut self) -> Self {
        self.flags.locked = true;
        self
    }

    /// Trigger on key release (`bindr`).
    pub fn release(mut self) -> Self {
        self.flags.release = true;
        self
    }

    /// Repeat while held (`binde`).
    pub fn repeat(mut self) -> Self {
        self.flags.repeat = true;
        self
    }

    /// Mouse-button binding — emits `bindm` instead of `bind`.
    pub fn mouse(mut self) -> Self {
        self.flags.mouse = true;
        self
    }

    /// Non-consuming: also passes the key to the window (`bindn`).
    pub fn non_consuming(mut self) -> Self {
        self.flags.non_consuming = true;
        self
    }
}

impl HyprlandConfig for Bind {
    fn render(&self, ctx: &HyprlandRenderContext) -> String {
        let keyword = if self.flags.mouse {
            "bindm".to_string()
        } else {
            let flags = self.flags.flag_str();
            format!("bind{}", flags)
        };
        let dispatch = self.dispatcher.to_dispatch_str();
        format!(
            "{}{} = {}, {}, {}",
            ctx.indent(),
            keyword,
            self.modifiers,
            self.key,
            dispatch
        )
    }

    fn validate(&self) -> Result<(), String> {
        if self.key.is_empty() {
            return Err("Bind key cannot be empty".into());
        }
        Ok(())
    }
}
