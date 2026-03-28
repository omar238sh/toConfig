use crate::fstab::options::{MountOpt, render_opts};

/// A single `/etc/fstab` entry.
///
/// Each field corresponds to a column in the fstab file:
///
/// ```text
/// <device>  <mountpoint>  <fstype>  <options>  <dump>  <pass>
/// ```
///
/// # Example
/// ```
/// use toconfig::fstab::FstabEntry;
///
/// let entry = FstabEntry::new("/dev/sda1", "/", "ext4")
///     .options_str("defaults,noatime")
///     .pass(1);
///
/// assert!(entry.render().contains("/dev/sda1"));
/// assert!(entry.render().contains("noatime"));
/// ```
pub struct FstabEntry {
    /// Column 1: block device, label (`LABEL=…`), UUID (`UUID=…`), or network share.
    pub device: String,
    /// Column 2: absolute mount point (or `none` for swap).
    pub mount_point: String,
    /// Column 3: filesystem type (e.g. `ext4`, `btrfs`, `tmpfs`, `swap`).
    pub fs_type: String,
    /// Column 4: mount options.
    pub options: Vec<MountOpt>,
    /// Column 4 override: a raw options string; takes precedence over `options`.
    pub options_raw: Option<String>,
    /// Column 5: dump flag (`0` = skip, `1` = include in `dump` backups).
    pub dump: u8,
    /// Column 6: fsck pass order (`0` = skip, `1` = root, `2` = other).
    pub pass: u8,
    /// Optional inline comment rendered above the entry.
    pub comment: Option<String>,
}

impl FstabEntry {
    /// Create a new entry with the minimum required fields.
    ///
    /// `dump` defaults to `0`, `pass` defaults to `0`.
    pub fn new(
        device: impl Into<String>,
        mount_point: impl Into<String>,
        fs_type: impl Into<String>,
    ) -> Self {
        Self {
            device: device.into(),
            mount_point: mount_point.into(),
            fs_type: fs_type.into(),
            options: Vec::new(),
            options_raw: None,
            dump: 0,
            pass: 0,
            comment: None,
        }
    }

    // ── Device helpers ────────────────────────────────────────────────────

    /// Use a `UUID=<uuid>` device identifier (preferred — survives device renaming).
    pub fn uuid(mut self, uuid: impl Into<String>) -> Self {
        self.device = format!("UUID={}", uuid.into());
        self
    }

    /// Use a `LABEL=<label>` device identifier.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.device = format!("LABEL={}", label.into());
        self
    }

    /// Use a `PARTUUID=<uuid>` GPT partition UUID.
    pub fn partuuid(mut self, uuid: impl Into<String>) -> Self {
        self.device = format!("PARTUUID={}", uuid.into());
        self
    }

    // ── Option setters ────────────────────────────────────────────────────

    /// Add a typed [`MountOpt`].
    pub fn option(mut self, opt: MountOpt) -> Self {
        self.options.push(opt);
        self
    }

    /// Add multiple typed [`MountOpt`]s (e.g. from a builder's `.build()`).
    pub fn options(mut self, opts: Vec<MountOpt>) -> Self {
        self.options.extend(opts);
        self
    }

    /// Set the options column as a raw comma-separated string.
    ///
    /// When set, this takes priority over [`MountOpt`] entries added via
    /// `option()` / `options()`.
    pub fn options_str(mut self, raw: impl Into<String>) -> Self {
        self.options_raw = Some(raw.into());
        self
    }

    // ── Other column setters ──────────────────────────────────────────────

    /// Set the `dump` flag (`0` or `1`).
    pub fn dump(mut self, v: u8) -> Self {
        self.dump = v;
        self
    }

    /// Set the `pass` (fsck order) flag (`0`, `1`, or `2`).
    pub fn pass(mut self, v: u8) -> Self {
        self.pass = v;
        self
    }

    /// Attach an inline comment rendered as a `# …` line above the entry.
    pub fn comment(mut self, text: impl Into<String>) -> Self {
        self.comment = Some(text.into());
        self
    }

    // ── Rendering ─────────────────────────────────────────────────────────

    /// Render this entry as a single fstab line (plus optional comment).
    pub fn render(&self) -> String {
        let opts = if let Some(ref raw) = self.options_raw {
            raw.clone()
        } else {
            render_opts(&self.options)
        };

        let line = format!(
            "{:<44} {:<24} {:<8} {:<24} {} {}",
            self.device, self.mount_point, self.fs_type, opts, self.dump, self.pass
        );

        if let Some(ref c) = self.comment {
            format!("# {}\n{}", c, line)
        } else {
            line
        }
    }

    /// Validate the entry: device and mount point must not be empty.
    pub fn validate(&self) -> Result<(), String> {
        if self.device.is_empty() {
            return Err("fstab: device field cannot be empty".into());
        }
        if self.mount_point.is_empty() {
            return Err("fstab: mount point cannot be empty".into());
        }
        Ok(())
    }
}

// ── Convenience constructors ──────────────────────────────────────────────────

impl FstabEntry {
    /// Shortcut: create a swap entry (`none` mount point, `swap` fstype).
    ///
    /// ```
    /// use toconfig::fstab::FstabEntry;
    /// let e = FstabEntry::swap("/dev/sdb1");
    /// assert!(e.render().contains("swap"));
    /// ```
    pub fn swap(device: impl Into<String>) -> Self {
        Self::new(device, "none", "swap").options_str("sw")
    }

    /// Shortcut: create a tmpfs entry.
    ///
    /// ```
    /// use toconfig::fstab::FstabEntry;
    /// use toconfig::fstab::options::{TmpfsOptions, CommonOptions};
    /// let e = FstabEntry::tmpfs("/tmp")
    ///     .options(TmpfsOptions::new().size("4G").mode("1777").build())
    ///     .options(CommonOptions::new().noexec().nosuid().nodev().build());
    /// assert!(e.render().contains("tmpfs"));
    /// ```
    pub fn tmpfs(mount_point: impl Into<String>) -> Self {
        Self::new("tmpfs", mount_point, "tmpfs")
    }
}

// ── Fstab collection ──────────────────────────────────────────────────────────

/// A complete `/etc/fstab` file.
///
/// Appends entries in insertion order, optionally with a header comment.
///
/// # Example
/// ```
/// use toconfig::fstab::{Fstab, FstabEntry};
///
/// let fstab = Fstab::new()
///     .with_header("# /etc/fstab — generated by toconfig")
///     .add(
///         FstabEntry::new("/dev/sda1", "/", "ext4")
///             .options_str("defaults,noatime")
///             .pass(1),
///     )
///     .add(FstabEntry::swap("/dev/sda2"));
///
/// let out = fstab.generate();
/// assert!(out.contains("/dev/sda1"));
/// assert!(out.contains("swap"));
/// ```
pub struct Fstab {
    pub header: Option<String>,
    pub entries: Vec<FstabEntry>,
}

impl Default for Fstab {
    fn default() -> Self {
        Self::new()
    }
}

impl Fstab {
    pub fn new() -> Self {
        Self {
            header: None,
            entries: Vec::new(),
        }
    }

    /// Set the file header (rendered verbatim at the top).
    pub fn with_header(mut self, header: impl Into<String>) -> Self {
        self.header = Some(header.into());
        self
    }

    /// Add an entry (consuming builder).
    pub fn add(mut self, entry: FstabEntry) -> Self {
        self.entries.push(entry);
        self
    }

    /// Add an entry (mutable borrow).
    pub fn push(&mut self, entry: FstabEntry) -> &mut Self {
        self.entries.push(entry);
        self
    }

    /// Validate all entries, collecting every error.
    pub fn validate_all(&self) -> Result<(), Vec<String>> {
        let errors: Vec<String> = self
            .entries
            .iter()
            .filter_map(|e| e.validate().err())
            .collect();
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Render the complete fstab file as a `String`.
    pub fn generate(&self) -> String {
        let mut parts: Vec<String> = Vec::new();

        if let Some(ref h) = self.header {
            parts.push(h.clone());
        }

        for entry in &self.entries {
            parts.push(entry.render());
        }

        parts.join("\n")
    }
}
