use std::fmt;

/// A single mount option value — either a bare flag or a key=value pair.
///
/// # Example
/// ```
/// use toconfig::fstab::options::MountOpt;
///
/// assert_eq!(MountOpt::flag("noatime").to_string(), "noatime");
/// assert_eq!(MountOpt::kv("uid", "1000").to_string(), "uid=1000");
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum MountOpt {
    /// A bare option flag, e.g. `defaults`, `noatime`, `ro`.
    Flag(String),
    /// A key=value option, e.g. `uid=1000`, `size=4G`, `mode=755`.
    KeyValue(String, String),
}

impl MountOpt {
    /// Create a bare flag option.
    pub fn flag(name: impl Into<String>) -> Self {
        MountOpt::Flag(name.into())
    }

    /// Create a key=value option.
    pub fn kv(key: impl Into<String>, value: impl Into<String>) -> Self {
        MountOpt::KeyValue(key.into(), value.into())
    }
}

impl fmt::Display for MountOpt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MountOpt::Flag(s) => write!(f, "{}", s),
            MountOpt::KeyValue(k, v) => write!(f, "{}={}", k, v),
        }
    }
}

// ── Option builder helpers ────────────────────────────────────────────────────

/// Render a list of [`MountOpt`] as a comma-separated string.
///
/// Returns `"defaults"` when the list is empty (safe fstab default).
pub fn render_opts(opts: &[MountOpt]) -> String {
    if opts.is_empty() {
        "defaults".to_string()
    } else {
        opts.iter()
            .map(|o| o.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }
}

// ── Typed option preset builders ──────────────────────────────────────────────

/// Common POSIX / VFS mount options applicable to most filesystems.
///
/// Methods return a [`Vec<MountOpt>`] so they can be merged with
/// filesystem-specific option builders.
///
/// # Example
/// ```
/// use toconfig::fstab::options::{CommonOptions, render_opts};
///
/// let opts = CommonOptions::new()
///     .ro()
///     .noexec()
///     .nosuid()
///     .build();
///
/// assert_eq!(render_opts(&opts), "ro,noexec,nosuid");
/// ```
#[derive(Default)]
pub struct CommonOptions {
    opts: Vec<MountOpt>,
}

impl CommonOptions {
    pub fn new() -> Self {
        Self::default()
    }

    /// Include `defaults` (rw, suid, dev, exec, auto, nouser, async).
    pub fn defaults(mut self) -> Self {
        self.opts.push(MountOpt::flag("defaults"));
        self
    }

    /// Mount read-only (`ro`).
    pub fn ro(mut self) -> Self {
        self.opts.push(MountOpt::flag("ro"));
        self
    }

    /// Mount read-write (`rw`). This is the default; add explicitly for clarity.
    pub fn rw(mut self) -> Self {
        self.opts.push(MountOpt::flag("rw"));
        self
    }

    /// Do not update access times (`noatime`).
    pub fn noatime(mut self) -> Self {
        self.opts.push(MountOpt::flag("noatime"));
        self
    }

    /// Update directory access times (`relatime` — kernel default since 2.6.30).
    pub fn relatime(mut self) -> Self {
        self.opts.push(MountOpt::flag("relatime"));
        self
    }

    /// Do not allow program execution (`noexec`).
    pub fn noexec(mut self) -> Self {
        self.opts.push(MountOpt::flag("noexec"));
        self
    }

    /// Ignore set-user-ID / set-group-ID bits (`nosuid`).
    pub fn nosuid(mut self) -> Self {
        self.opts.push(MountOpt::flag("nosuid"));
        self
    }

    /// Do not interpret character/block special devices (`nodev`).
    pub fn nodev(mut self) -> Self {
        self.opts.push(MountOpt::flag("nodev"));
        self
    }

    /// Mount asynchronously (`async` — usually the default).
    pub fn async_(mut self) -> Self {
        self.opts.push(MountOpt::flag("async"));
        self
    }

    /// Mount synchronously (`sync` — slower but safer for removable media).
    pub fn sync(mut self) -> Self {
        self.opts.push(MountOpt::flag("sync"));
        self
    }

    /// Allow non-root users to mount (`user`).
    pub fn user(mut self) -> Self {
        self.opts.push(MountOpt::flag("user"));
        self
    }

    /// Allow any user to mount/unmount (`users`).
    pub fn users(mut self) -> Self {
        self.opts.push(MountOpt::flag("users"));
        self
    }

    /// Do not mount at boot (`noauto`).
    pub fn noauto(mut self) -> Self {
        self.opts.push(MountOpt::flag("noauto"));
        self
    }

    /// Mount at boot (`auto` — default).
    pub fn auto(mut self) -> Self {
        self.opts.push(MountOpt::flag("auto"));
        self
    }

    /// Set the owner uid for the mount (`uid=<n>`).
    pub fn uid(mut self, uid: u32) -> Self {
        self.opts.push(MountOpt::kv("uid", uid.to_string()));
        self
    }

    /// Set the owner gid for the mount (`gid=<n>`).
    pub fn gid(mut self, gid: u32) -> Self {
        self.opts.push(MountOpt::kv("gid", gid.to_string()));
        self
    }

    /// Set the file creation mask (`umask=<octal>`).
    pub fn umask(mut self, mask: impl Into<String>) -> Self {
        self.opts.push(MountOpt::kv("umask", mask));
        self
    }

    /// Add an arbitrary raw option.
    pub fn raw(mut self, opt: impl Into<String>) -> Self {
        self.opts.push(MountOpt::flag(opt));
        self
    }

    /// Consume the builder and return the option list.
    pub fn build(self) -> Vec<MountOpt> {
        self.opts
    }
}

/// ext2 / ext3 / ext4 specific mount options.
///
/// # Example
/// ```
/// use toconfig::fstab::options::{ExtOptions, render_opts};
///
/// let opts = ExtOptions::new().journal_commit(5).data("ordered").build();
/// assert!(render_opts(&opts).contains("journal_commit=5"));
/// ```
#[derive(Default)]
pub struct ExtOptions {
    opts: Vec<MountOpt>,
}

impl ExtOptions {
    pub fn new() -> Self {
        Self::default()
    }

    /// `acl` — enable POSIX access-control lists.
    pub fn acl(mut self) -> Self {
        self.opts.push(MountOpt::flag("acl"));
        self
    }

    /// `user_xattr` — enable user extended attributes.
    pub fn user_xattr(mut self) -> Self {
        self.opts.push(MountOpt::flag("user_xattr"));
        self
    }

    /// `barrier=<0|1>` — enable/disable write barriers.
    pub fn barrier(mut self, v: u8) -> Self {
        self.opts.push(MountOpt::kv("barrier", v.to_string()));
        self
    }

    /// `data=<journal|ordered|writeback>` — journaling mode.
    pub fn data(mut self, mode: impl Into<String>) -> Self {
        self.opts.push(MountOpt::kv("data", mode));
        self
    }

    /// `journal_commit=<seconds>` — journal commit interval.
    pub fn journal_commit(mut self, secs: u32) -> Self {
        self.opts
            .push(MountOpt::kv("journal_commit", secs.to_string()));
        self
    }

    /// `errors=<continue|remount-ro|panic>` — error handling policy.
    pub fn errors(mut self, policy: impl Into<String>) -> Self {
        self.opts.push(MountOpt::kv("errors", policy));
        self
    }

    /// `noquota` — disable disk quotas.
    pub fn noquota(mut self) -> Self {
        self.opts.push(MountOpt::flag("noquota"));
        self
    }

    /// Consume and return options.
    pub fn build(self) -> Vec<MountOpt> {
        self.opts
    }
}

/// Btrfs-specific mount options.
///
/// # Example
/// ```
/// use toconfig::fstab::options::{BtrfsOptions, render_opts};
///
/// let opts = BtrfsOptions::new().compress("zstd:3").subvol("@home").build();
/// assert!(render_opts(&opts).contains("compress=zstd:3"));
/// assert!(render_opts(&opts).contains("subvol=@home"));
/// ```
#[derive(Default)]
pub struct BtrfsOptions {
    opts: Vec<MountOpt>,
}

impl BtrfsOptions {
    pub fn new() -> Self {
        Self::default()
    }

    /// `subvol=<path>` — mount the given subvolume path.
    pub fn subvol(mut self, path: impl Into<String>) -> Self {
        self.opts.push(MountOpt::kv("subvol", path));
        self
    }

    /// `subvolid=<id>` — mount the subvolume with the given numerical ID.
    pub fn subvolid(mut self, id: u64) -> Self {
        self.opts.push(MountOpt::kv("subvolid", id.to_string()));
        self
    }

    /// `compress=<algo>` — transparent compression algorithm.
    ///
    /// Common values: `"lzo"`, `"zlib"`, `"zstd"`, `"zstd:3"`, `"no"`.
    pub fn compress(mut self, algo: impl Into<String>) -> Self {
        self.opts.push(MountOpt::kv("compress", algo));
        self
    }

    /// `compress-force=<algo>` — force compression even for incompressible data.
    pub fn compress_force(mut self, algo: impl Into<String>) -> Self {
        self.opts.push(MountOpt::kv("compress-force", algo));
        self
    }

    /// `ssd` — enable SSD-specific optimisations.
    pub fn ssd(mut self) -> Self {
        self.opts.push(MountOpt::flag("ssd"));
        self
    }

    /// `nossd` — disable SSD optimisations (for spinning disks).
    pub fn nossd(mut self) -> Self {
        self.opts.push(MountOpt::flag("nossd"));
        self
    }

    /// `discard=async` — enable asynchronous TRIM (recommended for SSDs).
    pub fn discard_async(mut self) -> Self {
        self.opts.push(MountOpt::kv("discard", "async"));
        self
    }

    /// `nodatacow` — disable copy-on-write for new files.
    pub fn nodatacow(mut self) -> Self {
        self.opts.push(MountOpt::flag("nodatacow"));
        self
    }

    /// `space_cache=<v1|v2>` — free-space cache version.
    pub fn space_cache(mut self, version: impl Into<String>) -> Self {
        self.opts.push(MountOpt::kv("space_cache", version));
        self
    }

    /// `autodefrag` — enable automatic defragmentation.
    pub fn autodefrag(mut self) -> Self {
        self.opts.push(MountOpt::flag("autodefrag"));
        self
    }

    /// `degraded` — allow mounting a RAID array with missing devices.
    pub fn degraded(mut self) -> Self {
        self.opts.push(MountOpt::flag("degraded"));
        self
    }

    /// `noacl` — disable POSIX ACLs.
    pub fn noacl(mut self) -> Self {
        self.opts.push(MountOpt::flag("noacl"));
        self
    }

    /// Consume and return options.
    pub fn build(self) -> Vec<MountOpt> {
        self.opts
    }
}

/// tmpfs-specific mount options.
///
/// # Example
/// ```
/// use toconfig::fstab::options::{TmpfsOptions, render_opts};
///
/// let opts = TmpfsOptions::new().size("4G").mode("1777").build();
/// assert!(render_opts(&opts).contains("size=4G"));
/// assert!(render_opts(&opts).contains("mode=1777"));
/// ```
#[derive(Default)]
pub struct TmpfsOptions {
    opts: Vec<MountOpt>,
}

impl TmpfsOptions {
    pub fn new() -> Self {
        Self::default()
    }

    /// `size=<bytes|percent>` — maximum size (e.g. `"512M"`, `"4G"`, `"50%"`).
    pub fn size(mut self, s: impl Into<String>) -> Self {
        self.opts.push(MountOpt::kv("size", s));
        self
    }

    /// `nr_blocks=<n>` — maximum number of blocks (alternative to `size`).
    pub fn nr_blocks(mut self, n: u64) -> Self {
        self.opts.push(MountOpt::kv("nr_blocks", n.to_string()));
        self
    }

    /// `nr_inodes=<n>` — maximum number of inodes.
    pub fn nr_inodes(mut self, n: u64) -> Self {
        self.opts.push(MountOpt::kv("nr_inodes", n.to_string()));
        self
    }

    /// `mode=<octal>` — permission bits for the root tmpfs directory.
    pub fn mode(mut self, mode: impl Into<String>) -> Self {
        self.opts.push(MountOpt::kv("mode", mode));
        self
    }

    /// `uid=<n>` — owner uid for the root directory.
    pub fn uid(mut self, uid: u32) -> Self {
        self.opts.push(MountOpt::kv("uid", uid.to_string()));
        self
    }

    /// `gid=<n>` — owning group for the root directory.
    pub fn gid(mut self, gid: u32) -> Self {
        self.opts.push(MountOpt::kv("gid", gid.to_string()));
        self
    }

    /// Consume and return options.
    pub fn build(self) -> Vec<MountOpt> {
        self.opts
    }
}

/// NFS-specific mount options.
///
/// # Example
/// ```
/// use toconfig::fstab::options::{NfsOptions, render_opts};
///
/// let opts = NfsOptions::new().vers(4).soft().timeo(30).retrans(3).build();
/// assert!(render_opts(&opts).contains("vers=4"));
/// assert!(render_opts(&opts).contains("soft"));
/// ```
#[derive(Default)]
pub struct NfsOptions {
    opts: Vec<MountOpt>,
}

impl NfsOptions {
    pub fn new() -> Self {
        Self::default()
    }

    /// `vers=<2|3|4|4.1|4.2>` — NFS protocol version.
    pub fn vers(mut self, v: impl ToString) -> Self {
        self.opts.push(MountOpt::kv("vers", v.to_string()));
        self
    }

    /// `soft` — return an error if the serveris unavailable (vs. `hard`).
    pub fn soft(mut self) -> Self {
        self.opts.push(MountOpt::flag("soft"));
        self
    }

    /// `hard` — retry indefinitely (default, more reliable).
    pub fn hard(mut self) -> Self {
        self.opts.push(MountOpt::flag("hard"));
        self
    }

    /// `timeo=<n>` — timeout in tenths of a second before retrying.
    pub fn timeo(mut self, n: u32) -> Self {
        self.opts.push(MountOpt::kv("timeo", n.to_string()));
        self
    }

    /// `retrans=<n>` — number of retries before giving up.
    pub fn retrans(mut self, n: u32) -> Self {
        self.opts.push(MountOpt::kv("retrans", n.to_string()));
        self
    }

    /// `rsize=<n>` — read block size in bytes.
    pub fn rsize(mut self, n: u32) -> Self {
        self.opts.push(MountOpt::kv("rsize", n.to_string()));
        self
    }

    /// `wsize=<n>` — write block size in bytes.
    pub fn wsize(mut self, n: u32) -> Self {
        self.opts.push(MountOpt::kv("wsize", n.to_string()));
        self
    }

    /// `proto=<tcp|udp|rdma>` — transport protocol.
    pub fn proto(mut self, p: impl Into<String>) -> Self {
        self.opts.push(MountOpt::kv("proto", p));
        self
    }

    /// `_netdev` — hint that this is a network device (start after network is up).
    pub fn netdev(mut self) -> Self {
        self.opts.push(MountOpt::flag("_netdev"));
        self
    }

    /// Consume and return options.
    pub fn build(self) -> Vec<MountOpt> {
        self.opts
    }
}
