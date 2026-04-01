use super::core::{GhosttyConfig, GhosttyRenderContext};

/// Shell integration auto-detection policy.
#[derive(Clone, Debug)]
pub enum ShellIntegration {
    /// Auto-detect the shell (default).
    Detect,
    /// Disable shell integration entirely.
    None,
    Bash,
    Fish,
    Zsh,
    Elvish,
}

impl std::fmt::Display for ShellIntegration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Detect => "detect",
            Self::None => "none",
            Self::Bash => "bash",
            Self::Fish => "fish",
            Self::Zsh => "zsh",
            Self::Elvish => "elvish",
        };
        write!(f, "{}", s)
    }
}

/// Shell and process configuration for Ghostty.
///
/// Renders as `command`, `login-shell`, `working-directory`, and
/// `shell-integration*` entries.
#[derive(Default, Clone, Debug)]
pub struct ShellConfig {
    /// Override the shell command (default: user's login shell).
    pub command: Option<String>,
    /// Command to run for the initial surface only.
    pub initial_command: Option<String>,
    /// Invoke the shell as a login shell.
    pub login_shell: Option<bool>,
    /// Starting working directory.
    pub working_directory: Option<String>,
    /// Which shell integration to inject.
    pub shell_integration: Option<ShellIntegration>,
    /// Comma-separated list of integration features to enable/disable,
    /// e.g. `"cursor,title"` or `"no-cursor"`.
    pub shell_integration_features: Option<String>,
}

impl ShellConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn command(mut self, v: impl Into<String>) -> Self {
        self.command = Some(v.into());
        self
    }

    pub fn initial_command(mut self, v: impl Into<String>) -> Self {
        self.initial_command = Some(v.into());
        self
    }

    pub fn login_shell(mut self, v: bool) -> Self {
        self.login_shell = Some(v);
        self
    }

    pub fn working_directory(mut self, v: impl Into<String>) -> Self {
        self.working_directory = Some(v.into());
        self
    }

    pub fn shell_integration(mut self, v: ShellIntegration) -> Self {
        self.shell_integration = Some(v);
        self
    }

    pub fn shell_integration_features(mut self, v: impl Into<String>) -> Self {
        self.shell_integration_features = Some(v.into());
        self
    }
}

impl GhosttyConfig for ShellConfig {
    fn render(&self, _ctx: &GhosttyRenderContext) -> String {
        let mut out = String::new();

        if let Some(ref v) = self.command {
            out.push_str(&format!("command = {}\n", v));
        }
        if let Some(ref v) = self.initial_command {
            out.push_str(&format!("initial-command = {}\n", v));
        }
        if let Some(v) = self.login_shell {
            out.push_str(&format!("login-shell = {}\n", v));
        }
        if let Some(ref v) = self.working_directory {
            out.push_str(&format!("working-directory = {}\n", v));
        }
        if let Some(ref v) = self.shell_integration {
            out.push_str(&format!("shell-integration = {}\n", v));
        }
        if let Some(ref v) = self.shell_integration_features {
            out.push_str(&format!("shell-integration-features = {}\n", v));
        }

        out
    }
}
