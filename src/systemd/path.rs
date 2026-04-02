use super::{SystemdConfig, SystemdRenderContext};

/// The `[Path]` section for `.path` unit files.
///
/// Path units monitor filesystem paths using `inotify` and activate a
/// corresponding service unit when the specified condition is met.  The
/// activated unit defaults to the same name with `.service` suffix.
///
/// # Example
/// ```
/// # use toconfig::systemd::SystemdConfig;
/// use toconfig::systemd::path::PathSection;
///
/// let p = PathSection::new()
///     .path_exists_glob("/var/spool/myapp/*.job")
///     .path_changed("/etc/myapp/config.toml")
///     .unit("myapp-reload.service")
///     .make_directory(true);
///
/// let out = p.generate();
/// assert!(out.contains("[Path]"));
/// assert!(out.contains("PathExistsGlob=/var/spool/myapp/*.job"));
/// assert!(out.contains("MakeDirectory=yes"));
/// ```
#[derive(Default)]
pub struct PathSection {
    /// `PathExists=` — activate when the path exists.
    pub path_exists: Vec<String>,
    /// `PathExistsGlob=` — activate when the glob has at least one match.
    pub path_exists_glob: Vec<String>,
    /// `PathChanged=` — activate whenever the path is created, removed, or written.
    pub path_changed: Vec<String>,
    /// `PathModified=` — like `PathChanged` but also triggers on access-time changes.
    pub path_modified: Vec<String>,
    /// `DirectoryNotEmpty=` — activate when the directory is non-empty.
    pub directory_not_empty: Vec<String>,
    /// `Unit=` — override the activated unit name.
    pub unit: Option<String>,
    /// `MakeDirectory=` — create the watched path if it does not exist.
    pub make_directory: Option<bool>,
    /// `DirectoryMode=` — permissions for auto-created directories.
    pub directory_mode: Option<String>,
    /// `TriggerLimitIntervalSec=` — rate-limit activation within this window.
    pub trigger_limit_interval_sec: Option<String>,
    /// `TriggerLimitBurst=` — max activations per `TriggerLimitIntervalSec`.
    pub trigger_limit_burst: Option<u32>,
}

impl PathSection {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a `PathExists=` watch.
    pub fn path_exists(mut self, path: impl Into<String>) -> Self {
        self.path_exists.push(path.into());
        self
    }

    /// Add a `PathExistsGlob=` watch.
    pub fn path_exists_glob(mut self, glob: impl Into<String>) -> Self {
        self.path_exists_glob.push(glob.into());
        self
    }

    /// Add a `PathChanged=` watch.
    pub fn path_changed(mut self, path: impl Into<String>) -> Self {
        self.path_changed.push(path.into());
        self
    }

    /// Add a `PathModified=` watch.
    pub fn path_modified(mut self, path: impl Into<String>) -> Self {
        self.path_modified.push(path.into());
        self
    }

    /// Add a `DirectoryNotEmpty=` watch.
    pub fn directory_not_empty(mut self, path: impl Into<String>) -> Self {
        self.directory_not_empty.push(path.into());
        self
    }

    /// Override the activated unit with `Unit=`.
    pub fn unit(mut self, name: impl Into<String>) -> Self {
        self.unit = Some(name.into());
        self
    }

    /// Set `MakeDirectory=` — auto-create watched paths.
    pub fn make_directory(mut self, v: bool) -> Self {
        self.make_directory = Some(v);
        self
    }

    /// Set `DirectoryMode=` — permissions for auto-created directories.
    pub fn directory_mode(mut self, mode: impl Into<String>) -> Self {
        self.directory_mode = Some(mode.into());
        self
    }

    /// Set `TriggerLimitIntervalSec=`.
    pub fn trigger_limit_interval_sec(mut self, s: impl Into<String>) -> Self {
        self.trigger_limit_interval_sec = Some(s.into());
        self
    }

    /// Set `TriggerLimitBurst=`.
    pub fn trigger_limit_burst(mut self, n: u32) -> Self {
        self.trigger_limit_burst = Some(n);
        self
    }
}

fn bool_str(b: bool) -> &'static str {
    if b { "yes" } else { "no" }
}

impl SystemdConfig for PathSection {
    fn render(&self, _ctx: &SystemdRenderContext) -> String {
        let mut lines = vec!["[Path]".to_string()];

        for p in &self.path_exists {
            lines.push(format!("PathExists={}", p));
        }
        for p in &self.path_exists_glob {
            lines.push(format!("PathExistsGlob={}", p));
        }
        for p in &self.path_changed {
            lines.push(format!("PathChanged={}", p));
        }
        for p in &self.path_modified {
            lines.push(format!("PathModified={}", p));
        }
        for p in &self.directory_not_empty {
            lines.push(format!("DirectoryNotEmpty={}", p));
        }
        if let Some(ref u) = self.unit {
            lines.push(format!("Unit={}", u));
        }
        if let Some(v) = self.make_directory {
            lines.push(format!("MakeDirectory={}", bool_str(v)));
        }
        if let Some(ref m) = self.directory_mode {
            lines.push(format!("DirectoryMode={}", m));
        }
        if let Some(ref s) = self.trigger_limit_interval_sec {
            lines.push(format!("TriggerLimitIntervalSec={}", s));
        }
        if let Some(n) = self.trigger_limit_burst {
            lines.push(format!("TriggerLimitBurst={}", n));
        }

        lines.join("\n")
    }

    fn validate(&self) -> Result<(), String> {
        let has_watch = !self.path_exists.is_empty()
            || !self.path_exists_glob.is_empty()
            || !self.path_changed.is_empty()
            || !self.path_modified.is_empty()
            || !self.directory_not_empty.is_empty();
        if !has_watch {
            return Err(
                "[Path] at least one Path* or DirectoryNotEmpty= directive is required".into(),
            );
        }
        Ok(())
    }
}
