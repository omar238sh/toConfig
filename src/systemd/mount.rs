use super::{SystemdConfig, SystemdRenderContext};

/// The `[Mount]` section for `.mount` unit files.
///
/// Mount units describe filesystem mount points managed by systemd,
/// providing a declarative alternative to `/etc/fstab`.  The unit file
/// name must encode the mount point path
/// (e.g. `home-omar.mount` for `/home/omar`).
///
/// # Example
/// ```
/// # use toconfig::systemd::SystemdConfig;
/// use toconfig::systemd::mount::MountSection;
///
/// let m = MountSection::new()
///     .what("/dev/sda1")
///     .where_("/mnt/data")
///     .type_("ext4")
///     .options("defaults,noatime")
///     .timeout_sec("30s");
///
/// let out = m.generate();
/// assert!(out.contains("[Mount]"));
/// assert!(out.contains("What=/dev/sda1"));
/// assert!(out.contains("Where=/mnt/data"));
/// assert!(out.contains("Type=ext4"));
/// ```
#[derive(Default)]
pub struct MountSection {
    pub what: Option<String>,
    pub where_: Option<String>,
    pub type_: Option<String>,
    pub options: Option<String>,
    pub lazy_unmount: Option<bool>,
    pub read_write_only: Option<bool>,
    pub force_unmount: Option<bool>,
    pub directory_mode: Option<String>,
    pub timeout_sec: Option<String>,
    pub sloppy_options: Option<bool>,
}

impl MountSection {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set `What=` — the device, path, or network share to mount.
    pub fn what(mut self, v: impl Into<String>) -> Self {
        self.what = Some(v.into());
        self
    }

    /// Set `Where=` — the absolute mount point path.
    ///
    /// This is also used to derive the unit file name; encode `/` as `-`
    /// (e.g. `/home/omar` → `home-omar.mount`).
    pub fn where_(mut self, v: impl Into<String>) -> Self {
        self.where_ = Some(v.into());
        self
    }

    /// Set `Type=` — the filesystem type (e.g. `"ext4"`, `"btrfs"`, `"tmpfs"`).
    pub fn type_(mut self, v: impl Into<String>) -> Self {
        self.type_ = Some(v.into());
        self
    }

    /// Set `Options=` — mount options (comma-separated, as in `fstab`).
    pub fn options(mut self, v: impl Into<String>) -> Self {
        self.options = Some(v.into());
        self
    }

    /// Set `LazyUnmount=` — detach the filesystem immediately even if busy.
    pub fn lazy_unmount(mut self, v: bool) -> Self {
        self.lazy_unmount = Some(v);
        self
    }

    /// Set `ReadWriteOnly=` — remount read-write if the kernel mounted it read-only.
    pub fn read_write_only(mut self, v: bool) -> Self {
        self.read_write_only = Some(v);
        self
    }

    /// Set `ForceUnmount=` — force unmount even if busy (use with care).
    pub fn force_unmount(mut self, v: bool) -> Self {
        self.force_unmount = Some(v);
        self
    }

    /// Set `DirectoryMode=` — permissions for auto-created mount point directory.
    pub fn directory_mode(mut self, mode: impl Into<String>) -> Self {
        self.directory_mode = Some(mode.into());
        self
    }

    /// Set `TimeoutSec=` — how long to wait for the mount to complete.
    pub fn timeout_sec(mut self, s: impl Into<String>) -> Self {
        self.timeout_sec = Some(s.into());
        self
    }

    /// Set `SloppyOptions=` — ignore unknown mount options instead of failing.
    pub fn sloppy_options(mut self, v: bool) -> Self {
        self.sloppy_options = Some(v);
        self
    }
}

fn bool_str(b: bool) -> &'static str {
    if b { "yes" } else { "no" }
}

impl SystemdConfig for MountSection {
    fn render(&self, _ctx: &SystemdRenderContext) -> String {
        let mut lines = vec!["[Mount]".to_string()];

        if let Some(ref v) = self.what {
            lines.push(format!("What={}", v));
        }
        if let Some(ref v) = self.where_ {
            lines.push(format!("Where={}", v));
        }
        if let Some(ref v) = self.type_ {
            lines.push(format!("Type={}", v));
        }
        if let Some(ref v) = self.options {
            lines.push(format!("Options={}", v));
        }
        if let Some(v) = self.lazy_unmount {
            lines.push(format!("LazyUnmount={}", bool_str(v)));
        }
        if let Some(v) = self.read_write_only {
            lines.push(format!("ReadWriteOnly={}", bool_str(v)));
        }
        if let Some(v) = self.force_unmount {
            lines.push(format!("ForceUnmount={}", bool_str(v)));
        }
        if let Some(ref m) = self.directory_mode {
            lines.push(format!("DirectoryMode={}", m));
        }
        if let Some(ref s) = self.timeout_sec {
            lines.push(format!("TimeoutSec={}", s));
        }
        if let Some(v) = self.sloppy_options {
            lines.push(format!("SloppyOptions={}", bool_str(v)));
        }

        lines.join("\n")
    }

    fn validate(&self) -> Result<(), String> {
        if self.what.is_none() {
            return Err("[Mount] What= is required".into());
        }
        if self.where_.is_none() {
            return Err("[Mount] Where= is required".into());
        }
        Ok(())
    }
}
