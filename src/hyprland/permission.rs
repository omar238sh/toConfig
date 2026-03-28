use super::{HyprlandConfig, HyprlandRenderContext};

/// The action taken when a permission rule matches.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionAction {
    Allow,
    Deny,
    Ask,
}

impl PermissionAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            PermissionAction::Allow => "allow",
            PermissionAction::Deny => "deny",
            PermissionAction::Ask => "ask",
        }
    }
}

/// The category of capability being controlled.
#[derive(Debug, Clone)]
pub enum PermissionType {
    /// Screen recording / screencopy (wlr-screencopy protocol).
    ScreenCopy,
    /// Access to terminal emulators.
    Terminal,
    /// Camera access via pipewire.
    Camera,
    /// Microphone access.
    Microphone,
    /// Location services.
    Location,
    /// Custom / future capability name.
    Custom(String),
}

impl PermissionType {
    pub fn as_str(&self) -> &str {
        match self {
            PermissionType::ScreenCopy => "screencopy",
            PermissionType::Terminal => "terminal",
            PermissionType::Camera => "camera",
            PermissionType::Microphone => "microphone",
            PermissionType::Location => "location",
            PermissionType::Custom(s) => s.as_str(),
        }
    }
}

/// A Hyprland permission rule.
///
/// Rendered as: `permission = pattern, type, action`
///
/// Permission rules control which applications can access sensitive features
/// such as screen recording via the `hyprland-toplevel-export` or
/// `wlr-screencopy` protocols.
///
/// The `pattern` is a regex matched against the executable path.
///
/// # Example
/// ```
/// # use toconfig::hyprland::HyprlandConfig;
/// use toconfig::hyprland::permission::{Permission, PermissionType, PermissionAction};
///
/// // Allow OBS to record the screen
/// let p = Permission::allow_screencopy(r"/usr/bin/obs");
/// assert_eq!(
///     p.generate(),
///     "permission = /usr/bin/obs, screencopy, allow"
/// );
///
/// // Deny everything else
/// let deny = Permission::deny_screencopy(r"/usr/bin/.*");
/// assert!(deny.generate().contains("deny"));
/// ```
pub struct Permission {
    pub pattern: String,
    pub permission_type: PermissionType,
    pub action: PermissionAction,
}

impl Permission {
    pub fn new(
        pattern: impl Into<String>,
        permission_type: PermissionType,
        action: PermissionAction,
    ) -> Self {
        Self {
            pattern: pattern.into(),
            permission_type,
            action,
        }
    }

    /// Allow an application to perform screen recording.
    pub fn allow_screencopy(pattern: impl Into<String>) -> Self {
        Self::new(pattern, PermissionType::ScreenCopy, PermissionAction::Allow)
    }

    /// Deny an application from performing screen recording.
    pub fn deny_screencopy(pattern: impl Into<String>) -> Self {
        Self::new(pattern, PermissionType::ScreenCopy, PermissionAction::Deny)
    }
}

impl HyprlandConfig for Permission {
    fn render(&self, ctx: &HyprlandRenderContext) -> String {
        format!(
            "{}permission = {}, {}, {}",
            ctx.indent(),
            self.pattern,
            self.permission_type.as_str(),
            self.action.as_str()
        )
    }

    fn validate(&self) -> Result<(), String> {
        if self.pattern.is_empty() {
            return Err("Permission pattern cannot be empty".into());
        }
        Ok(())
    }
}
