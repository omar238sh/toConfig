use super::{SystemdConfig, SystemdRenderContext};

/// The `[Automount]` section for `.automount` unit files.
///
/// Automount units implement on-demand mounting: systemd watches the mount
/// point and activates the corresponding `.mount` unit the first time
/// something accesses the path.
///
/// # Example
/// ```
/// # use toconfig::systemd::SystemdConfig;
/// use toconfig::systemd::automount::AutomountSection;
///
/// let am = AutomountSection::new()
///     .where_("/mnt/data")
///     .timeout_idle_sec("5min");
///
/// let out = am.generate();
/// assert!(out.contains("[Automount]"));
/// assert!(out.contains("Where=/mnt/data"));
/// assert!(out.contains("TimeoutIdleSec=5min"));
/// ```
#[derive(Default)]
pub struct AutomountSection {
    /// `Where=` — the mount point path (must match the corresponding `.mount` unit).
    pub where_: Option<String>,
    /// `TimeoutIdleSec=` — unmount after this idle time (0 = never auto-unmount).
    pub timeout_idle_sec: Option<String>,
    /// `DirectoryMode=` — permissions for the auto-created mount point directory.
    pub directory_mode: Option<String>,
}

impl AutomountSection {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set `Where=` — the absolute path of the automount point.
    pub fn where_(mut self, v: impl Into<String>) -> Self {
        self.where_ = Some(v.into());
        self
    }

    /// Set `TimeoutIdleSec=` — idle timeout before the mount is deactivated.
    ///
    /// Accepts systemd time-span values: `"0"`, `"30s"`, `"5min"`, etc.
    /// A value of `"0"` disables automatic unmounting.
    pub fn timeout_idle_sec(mut self, s: impl Into<String>) -> Self {
        self.timeout_idle_sec = Some(s.into());
        self
    }

    /// Set `DirectoryMode=` — permissions for the auto-created mount point.
    pub fn directory_mode(mut self, mode: impl Into<String>) -> Self {
        self.directory_mode = Some(mode.into());
        self
    }
}

impl SystemdConfig for AutomountSection {
    fn render(&self, _ctx: &SystemdRenderContext) -> String {
        let mut lines = vec!["[Automount]".to_string()];

        if let Some(ref v) = self.where_ {
            lines.push(format!("Where={}", v));
        }
        if let Some(ref s) = self.timeout_idle_sec {
            lines.push(format!("TimeoutIdleSec={}", s));
        }
        if let Some(ref m) = self.directory_mode {
            lines.push(format!("DirectoryMode={}", m));
        }

        lines.join("\n")
    }

    fn validate(&self) -> Result<(), String> {
        if self.where_.is_none() {
            return Err("[Automount] Where= is required".into());
        }
        Ok(())
    }
}
