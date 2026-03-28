use super::{HyprlandConfig, HyprlandRenderContext};

/// A Hyprland workspace rule.
///
/// Rendered as: `workspace = id, key:value[, key:value...]`
///
/// # Common rule keys
/// - `monitor:<name>` — bind workspace to a specific monitor
/// - `default:true` — make this the default workspace on its monitor
/// - `persistent:true` — keep the workspace alive even when empty
/// - `gapsin:<n>` / `gapsout:<n>` — per-workspace gap overrides
/// - `bordersize:<n>` — per-workspace border size
/// - `shadow:false` — disable shadows for this workspace
/// - `rounding:false` — disable rounding for this workspace
/// - `decorate:false` — disable decorations for this workspace
/// - `on-created-empty:<dispatch>` — dispatch run when workspace is first opened empty
///
/// # Special workspaces
/// Special workspaces (e.g. `special:magic`) act as scratchpads.
/// Use `workspace = special:magic, on-created-empty:kitty` to auto-launch a terminal.
///
/// # Example
/// ```
/// # use toconfig::hyprland::HyprlandConfig;
/// use toconfig::hyprland::workspace::WorkspaceRule;
///
/// let w = WorkspaceRule::new("1").monitor("eDP-1").default();
/// assert_eq!(w.generate(), "workspace = 1, monitor:eDP-1, default:true");
/// ```
pub struct WorkspaceRule {
    pub id: String,
    pub rules: Vec<(String, String)>,
}

impl WorkspaceRule {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            rules: Vec::new(),
        }
    }

    /// Add an arbitrary key:value rule.
    pub fn rule(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.rules.push((key.into(), value.into()));
        self
    }

    /// Bind this workspace to a specific monitor.
    pub fn monitor(self, name: impl Into<String>) -> Self {
        self.rule("monitor", name)
    }

    /// Mark this workspace as the default for its monitor.
    pub fn default(self) -> Self {
        self.rule("default", "true")
    }

    /// Keep the workspace alive even when no windows are on it.
    pub fn persistent(self) -> Self {
        self.rule("persistent", "true")
    }

    /// Run a dispatch command when the workspace is first opened empty.
    pub fn on_created_empty(self, dispatch: impl Into<String>) -> Self {
        self.rule("on-created-empty", dispatch)
    }

    /// Override gap inside for this workspace.
    pub fn gaps_in(self, px: u32) -> Self {
        self.rule("gapsin", px.to_string())
    }

    /// Override gap outside for this workspace.
    pub fn gaps_out(self, px: u32) -> Self {
        self.rule("gapsout", px.to_string())
    }
}

impl HyprlandConfig for WorkspaceRule {
    fn render(&self, ctx: &HyprlandRenderContext) -> String {
        if self.rules.is_empty() {
            return format!("{}workspace = {}", ctx.indent(), self.id);
        }
        let rules_str = self
            .rules
            .iter()
            .map(|(k, v)| format!("{}:{}", k, v))
            .collect::<Vec<_>>()
            .join(", ");
        format!("{}workspace = {}, {}", ctx.indent(), self.id, rules_str)
    }

    fn validate(&self) -> Result<(), String> {
        if self.id.is_empty() {
            return Err("WorkspaceRule id cannot be empty".into());
        }
        Ok(())
    }
}
