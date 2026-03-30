use super::{SystemdConfig, SystemdRenderContext};

/// The `[Install]` section common to all installable systemd units.
///
/// Controls how `systemctl enable` / `systemctl disable` handles the unit.
///
/// # Example
/// ```
/// # use toconfig::systemd::SystemdConfig;
/// use toconfig::systemd::install::InstallSection;
///
/// let install = InstallSection::new()
///     .wanted_by("multi-user.target")
///     .alias("myapp.service");
///
/// let out = install.generate();
/// assert!(out.contains("[Install]"));
/// assert!(out.contains("WantedBy=multi-user.target"));
/// assert!(out.contains("Alias=myapp.service"));
/// ```
#[derive(Default)]
pub struct InstallSection {
    pub wanted_by: Vec<String>,
    pub required_by: Vec<String>,
    pub alias: Vec<String>,
    pub also: Vec<String>,
    pub default_instance: Option<String>,
}

impl InstallSection {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a `WantedBy=` target (most common: `"multi-user.target"`).
    pub fn wanted_by(mut self, target: impl Into<String>) -> Self {
        self.wanted_by.push(target.into());
        self
    }

    /// Add a `RequiredBy=` target.
    pub fn required_by(mut self, target: impl Into<String>) -> Self {
        self.required_by.push(target.into());
        self
    }

    /// Add an `Alias=` symlink name.
    pub fn alias(mut self, name: impl Into<String>) -> Self {
        self.alias.push(name.into());
        self
    }

    /// Add an `Also=` unit to enable/disable alongside this one.
    pub fn also(mut self, unit: impl Into<String>) -> Self {
        self.also.push(unit.into());
        self
    }

    /// Set `DefaultInstance=` for template units.
    pub fn default_instance(mut self, instance: impl Into<String>) -> Self {
        self.default_instance = Some(instance.into());
        self
    }
}

impl SystemdConfig for InstallSection {
    fn render(&self, _ctx: &SystemdRenderContext) -> String {
        let mut lines = vec!["[Install]".to_string()];

        if !self.wanted_by.is_empty() {
            lines.push(format!("WantedBy={}", self.wanted_by.join(" ")));
        }
        if !self.required_by.is_empty() {
            lines.push(format!("RequiredBy={}", self.required_by.join(" ")));
        }
        for a in &self.alias {
            lines.push(format!("Alias={}", a));
        }
        if !self.also.is_empty() {
            lines.push(format!("Also={}", self.also.join(" ")));
        }
        if let Some(ref di) = self.default_instance {
            lines.push(format!("DefaultInstance={}", di));
        }

        lines.join("\n")
    }
}
