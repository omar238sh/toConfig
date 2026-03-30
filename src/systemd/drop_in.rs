use super::{SystemdConfig, SystemdRenderContext};

/// A drop-in override file for an existing systemd unit.
///
/// Drop-in files live at
/// `/etc/systemd/system/<unit>.d/<name>.conf`
/// and allow selectively overriding or extending settings without
/// replacing the original unit file.  Every field left as `None` is
/// simply omitted, making it easy to build minimal targeted overrides.
///
/// # Common patterns
///
/// ## Override restart policy
/// ```
/// # use toconfig::systemd::SystemdConfig;
/// use toconfig::systemd::drop_in::DropIn;
/// use toconfig::systemd::service::{ServiceSection, Restart};
///
/// let di = DropIn::new("override")
///     .service(
///         ServiceSection::new()
///             .restart(Restart::Always)
///             .restart_sec("5s"),
///     );
///
/// let out = di.generate();
/// assert!(out.contains("[Service]"));
/// assert!(out.contains("Restart=always"));
/// ```
///
/// ## Add an `After=` dependency to any unit
/// ```
/// # use toconfig::systemd::SystemdConfig;
/// use toconfig::systemd::drop_in::DropIn;
/// use toconfig::systemd::unit_section::UnitSection;
///
/// let di = DropIn::new("after-network")
///     .unit(UnitSection::new().after("network-online.target"));
///
/// let out = di.generate();
/// assert!(out.contains("[Unit]"));
/// assert!(out.contains("After=network-online.target"));
/// ```
pub struct DropIn {
    /// Logical name of this drop-in (used as the `.conf` filename stem).
    pub name: String,
    /// Optional header comment rendered at the top of the file.
    pub header_comment: Option<String>,
    /// Ordered list of section overrides.
    pub sections: Vec<Box<dyn SystemdConfig>>,
}

impl DropIn {
    /// Create a new drop-in with the given name (no `.conf` extension needed).
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            header_comment: None,
            sections: Vec::new(),
        }
    }

    /// Attach a header comment (rendered as `# …` lines).
    pub fn with_comment(mut self, comment: impl Into<String>) -> Self {
        self.header_comment = Some(comment.into());
        self
    }

    /// Add any [`SystemdConfig`] section to the drop-in (consuming builder).
    pub fn push<S: SystemdConfig + 'static>(mut self, section: S) -> Self {
        self.sections.push(Box::new(section));
        self
    }

    /// Add any [`SystemdConfig`] section to the drop-in (mutable borrow).
    pub fn add<S: SystemdConfig + 'static>(&mut self, section: S) -> &mut Self {
        self.sections.push(Box::new(section));
        self
    }

    // ── Typed convenience constructors ─────────────────────────────────────

    /// Add a `[Unit]` override section.
    pub fn unit(self, section: crate::systemd::unit_section::UnitSection) -> Self {
        self.push(section)
    }

    /// Add a `[Service]` override section.
    pub fn service(self, section: crate::systemd::service::ServiceSection) -> Self {
        self.push(section)
    }

    /// Add a `[Install]` override section.
    pub fn install(self, section: crate::systemd::install::InstallSection) -> Self {
        self.push(section)
    }

    /// Add a `[Timer]` override section.
    pub fn timer(self, section: crate::systemd::timer::TimerSection) -> Self {
        self.push(section)
    }

    /// Add a `[Socket]` override section.
    pub fn socket(self, section: crate::systemd::socket::SocketSection) -> Self {
        self.push(section)
    }

    /// Add a `[Mount]` override section.
    pub fn mount(self, section: crate::systemd::mount::MountSection) -> Self {
        self.push(section)
    }

    /// Add a `[Path]` override section.
    pub fn path(self, section: crate::systemd::path::PathSection) -> Self {
        self.push(section)
    }

    /// Validate all child sections, collecting every error.
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

    /// Returns the recommended file name for this drop-in: `<name>.conf`.
    pub fn file_name(&self) -> String {
        if self.name.ends_with(".conf") {
            self.name.clone()
        } else {
            format!("{}.conf", self.name)
        }
    }
}

impl SystemdConfig for DropIn {
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

        let sep = if ctx.blank_between_sections {
            "\n\n"
        } else {
            "\n"
        };
        parts.join(sep)
    }
}
