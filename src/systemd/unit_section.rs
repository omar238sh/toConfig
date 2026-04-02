use super::{SystemdConfig, SystemdRenderContext};

/// The `[Unit]` section common to every systemd unit file.
///
/// Carries generic metadata and ordering/dependency directives.
///
/// # Example
/// ```
/// # use toconfig::systemd::SystemdConfig;
/// use toconfig::systemd::unit_section::UnitSection;
///
/// let u = UnitSection::new()
///     .description("My daemon")
///     .documentation("https://example.com/docs")
///     .after("network-online.target")
///     .after("nss-lookup.target")
///     .wants("network-online.target");
///
/// let out = u.generate();
/// assert!(out.contains("[Unit]"));
/// assert!(out.contains("Description=My daemon"));
/// assert!(out.contains("After=network-online.target nss-lookup.target"));
/// ```
#[derive(Default)]
pub struct UnitSection {
    pub description: Option<String>,
    pub documentation: Vec<String>,
    pub after: Vec<String>,
    pub before: Vec<String>,
    pub requires: Vec<String>,
    pub wants: Vec<String>,
    pub conflicts: Vec<String>,
    pub binds_to: Vec<String>,
    pub part_of: Vec<String>,
    pub on_failure: Vec<String>,
    pub condition_path_exists: Vec<String>,
    pub assert_path_exists: Vec<String>,
    pub default_dependencies: Option<bool>,
    pub refuse_manual_start: Option<bool>,
    pub refuse_manual_stop: Option<bool>,
}

impl UnitSection {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the human-readable `Description=`.
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Add a `Documentation=` URI.
    pub fn documentation(mut self, uri: impl Into<String>) -> Self {
        self.documentation.push(uri.into());
        self
    }

    /// Add an `After=` ordering dependency.
    pub fn after(mut self, unit: impl Into<String>) -> Self {
        self.after.push(unit.into());
        self
    }

    /// Add a `Before=` ordering dependency.
    pub fn before(mut self, unit: impl Into<String>) -> Self {
        self.before.push(unit.into());
        self
    }

    /// Add a `Requires=` hard dependency.
    pub fn requires(mut self, unit: impl Into<String>) -> Self {
        self.requires.push(unit.into());
        self
    }

    /// Add a `Wants=` soft dependency.
    pub fn wants(mut self, unit: impl Into<String>) -> Self {
        self.wants.push(unit.into());
        self
    }

    /// Add a `Conflicts=` anti-dependency.
    pub fn conflicts(mut self, unit: impl Into<String>) -> Self {
        self.conflicts.push(unit.into());
        self
    }

    /// Add a `BindsTo=` dependency (stop together).
    pub fn binds_to(mut self, unit: impl Into<String>) -> Self {
        self.binds_to.push(unit.into());
        self
    }

    /// Add a `PartOf=` membership dependency.
    pub fn part_of(mut self, unit: impl Into<String>) -> Self {
        self.part_of.push(unit.into());
        self
    }

    /// Add an `OnFailure=` unit to activate when this unit fails.
    pub fn on_failure(mut self, unit: impl Into<String>) -> Self {
        self.on_failure.push(unit.into());
        self
    }

    /// Add a `ConditionPathExists=` check.
    pub fn condition_path_exists(mut self, path: impl Into<String>) -> Self {
        self.condition_path_exists.push(path.into());
        self
    }

    /// Add an `AssertPathExists=` check (hard assertion).
    pub fn assert_path_exists(mut self, path: impl Into<String>) -> Self {
        self.assert_path_exists.push(path.into());
        self
    }

    /// Set `DefaultDependencies=`.
    pub fn default_dependencies(mut self, v: bool) -> Self {
        self.default_dependencies = Some(v);
        self
    }

    /// Set `RefuseManualStart=`.
    pub fn refuse_manual_start(mut self, v: bool) -> Self {
        self.refuse_manual_start = Some(v);
        self
    }

    /// Set `RefuseManualStop=`.
    pub fn refuse_manual_stop(mut self, v: bool) -> Self {
        self.refuse_manual_stop = Some(v);
        self
    }
}

fn bool_str(b: bool) -> &'static str {
    if b { "yes" } else { "no" }
}

impl SystemdConfig for UnitSection {
    fn render(&self, _ctx: &SystemdRenderContext) -> String {
        let mut lines = vec!["[Unit]".to_string()];

        if let Some(ref d) = self.description {
            lines.push(format!("Description={}", d));
        }
        for doc in &self.documentation {
            lines.push(format!("Documentation={}", doc));
        }
        if !self.after.is_empty() {
            lines.push(format!("After={}", self.after.join(" ")));
        }
        if !self.before.is_empty() {
            lines.push(format!("Before={}", self.before.join(" ")));
        }
        if !self.requires.is_empty() {
            lines.push(format!("Requires={}", self.requires.join(" ")));
        }
        if !self.wants.is_empty() {
            lines.push(format!("Wants={}", self.wants.join(" ")));
        }
        if !self.conflicts.is_empty() {
            lines.push(format!("Conflicts={}", self.conflicts.join(" ")));
        }
        if !self.binds_to.is_empty() {
            lines.push(format!("BindsTo={}", self.binds_to.join(" ")));
        }
        if !self.part_of.is_empty() {
            lines.push(format!("PartOf={}", self.part_of.join(" ")));
        }
        if !self.on_failure.is_empty() {
            lines.push(format!("OnFailure={}", self.on_failure.join(" ")));
        }
        for p in &self.condition_path_exists {
            lines.push(format!("ConditionPathExists={}", p));
        }
        for p in &self.assert_path_exists {
            lines.push(format!("AssertPathExists={}", p));
        }
        if let Some(v) = self.default_dependencies {
            lines.push(format!("DefaultDependencies={}", bool_str(v)));
        }
        if let Some(v) = self.refuse_manual_start {
            lines.push(format!("RefuseManualStart={}", bool_str(v)));
        }
        if let Some(v) = self.refuse_manual_stop {
            lines.push(format!("RefuseManualStop={}", bool_str(v)));
        }

        lines.join("\n")
    }

    fn validate(&self) -> Result<(), String> {
        if self.description.as_deref().map(str::is_empty).unwrap_or(false) {
            return Err("[Unit] Description cannot be an empty string".into());
        }
        Ok(())
    }
}
