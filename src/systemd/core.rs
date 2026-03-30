//! Core traits and rendering context for systemd unit file generation.

/// Rendering context for systemd unit files.
///
/// Systemd INI files are flat (no indentation needed by default), but the context
/// is kept for forward-compatibility and consistency with other modules.
#[derive(Debug, Clone)]
pub struct SystemdRenderContext {
    /// Whether to emit blank lines between sections (default: true).
    pub blank_between_sections: bool,
}

impl Default for SystemdRenderContext {
    fn default() -> Self {
        Self {
            blank_between_sections: true,
        }
    }
}

/// Central trait for all systemd unit-file nodes.
///
/// Every section struct (`UnitSection`, `ServiceSection`, etc.) and the
/// top-level [`SystemdUnit`] implement this trait.
pub trait SystemdConfig {
    /// Render this node into a systemd unit-file string fragment.
    fn render(&self, ctx: &SystemdRenderContext) -> String;

    /// Convenience: render with a default context.
    fn generate(&self) -> String {
        self.render(&SystemdRenderContext::default())
    }

    /// Optional pre-render validation hook.
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Top-level systemd unit file builder.
///
/// Collects an ordered list of [`SystemdConfig`] sections and renders them
/// into a complete unit file.  Sections are separated by a blank line when
/// [`SystemdRenderContext::blank_between_sections`] is `true`.
///
/// # Example
/// ```
/// use toconfig::systemd::{SystemdConfig, SystemdUnit};
/// use toconfig::systemd::unit_section::UnitSection;
/// use toconfig::systemd::service::{ServiceSection, ServiceType, Restart};
/// use toconfig::systemd::install::InstallSection;
///
/// let unit = SystemdUnit::new()
///     .push(UnitSection::new().description("Example service").after("network.target"))
///     .push(
///         ServiceSection::new()
///             .service_type(ServiceType::Simple)
///             .exec_start("/usr/bin/myapp")
///             .restart(Restart::OnFailure),
///     )
///     .push(InstallSection::new().wanted_by("multi-user.target"));
///
/// let out = unit.generate();
/// assert!(out.contains("[Unit]"));
/// assert!(out.contains("[Service]"));
/// assert!(out.contains("[Install]"));
/// ```
pub struct SystemdUnit {
    pub sections: Vec<Box<dyn SystemdConfig>>,
    pub header_comment: Option<String>,
}

impl Default for SystemdUnit {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemdUnit {
    pub fn new() -> Self {
        Self {
            sections: Vec::new(),
            header_comment: None,
        }
    }

    /// Attach a header comment (rendered as `# …` lines at the top of the file).
    pub fn with_comment(mut self, comment: &str) -> Self {
        self.header_comment = Some(comment.to_string());
        self
    }

    /// Append a section (consuming, returns `Self` for builder chains).
    pub fn push<S: SystemdConfig + 'static>(mut self, section: S) -> Self {
        self.sections.push(Box::new(section));
        self
    }

    /// Append a section (mutable borrow, returns `&mut Self` for chaining).
    pub fn add<S: SystemdConfig + 'static>(&mut self, section: S) -> &mut Self {
        self.sections.push(Box::new(section));
        self
    }

    /// Run [`SystemdConfig::validate`] on every child section.
    pub fn validate_all(&self) -> Result<(), Vec<String>> {
        let errors: Vec<String> = self
            .sections
            .iter()
            .filter_map(|s| s.validate().err())
            .collect();
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl SystemdConfig for SystemdUnit {
    fn render(&self, ctx: &SystemdRenderContext) -> String {
        let mut parts: Vec<String> = Vec::new();

        if let Some(ref c) = self.header_comment {
            for line in c.lines() {
                parts.push(format!("# {}", line));
            }
        }

        for section in &self.sections {
            let rendered = section.render(ctx);
            if !rendered.is_empty() {
                parts.push(rendered);
            }
        }

        let sep = if ctx.blank_between_sections { "\n\n" } else { "\n" };
        parts.join(sep)
    }
}
