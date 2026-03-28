use super::{SystemdConfig, SystemdRenderContext};

/// The `[Swap]` section for `.swap` unit files.
///
/// Swap units manage swap devices or swap files, giving systemd full
/// lifecycle control over swap space (equivalent to `swapon`/`swapoff`).
///
/// # Example
/// ```
/// # use toconfig::systemd::SystemdConfig;
/// use toconfig::systemd::swap::SwapSection;
///
/// let s = SwapSection::new()
///     .what("/dev/sdb1")
///     .priority(10)
///     .timeout_sec("5s");
///
/// let out = s.generate();
/// assert!(out.contains("[Swap]"));
/// assert!(out.contains("What=/dev/sdb1"));
/// assert!(out.contains("Priority=10"));
/// ```
#[derive(Default)]
pub struct SwapSection {
    /// `What=` — the block device or swap file path.
    pub what: Option<String>,
    /// `Priority=` — swap priority (`-1` = kernel default, higher = preferred).
    pub priority: Option<i32>,
    /// `Options=` — options passed to `swapon -o` (comma-separated).
    pub options: Option<String>,
    /// `TimeoutSec=` — how long to wait for the swap to activate.
    pub timeout_sec: Option<String>,
}

impl SwapSection {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set `What=` — the block device or file to use as swap.
    pub fn what(mut self, v: impl Into<String>) -> Self {
        self.what = Some(v.into());
        self
    }

    /// Set `Priority=` — swap priority (`-1` to `32767`; higher = more preferred).
    pub fn priority(mut self, p: i32) -> Self {
        self.priority = Some(p);
        self
    }

    /// Set `Options=` — options forwarded to `swapon -o`.
    pub fn options(mut self, opts: impl Into<String>) -> Self {
        self.options = Some(opts.into());
        self
    }

    /// Set `TimeoutSec=` — activation timeout.
    pub fn timeout_sec(mut self, s: impl Into<String>) -> Self {
        self.timeout_sec = Some(s.into());
        self
    }
}

impl SystemdConfig for SwapSection {
    fn render(&self, _ctx: &SystemdRenderContext) -> String {
        let mut lines = vec!["[Swap]".to_string()];

        if let Some(ref v) = self.what {
            lines.push(format!("What={}", v));
        }
        if let Some(p) = self.priority {
            lines.push(format!("Priority={}", p));
        }
        if let Some(ref o) = self.options {
            lines.push(format!("Options={}", o));
        }
        if let Some(ref s) = self.timeout_sec {
            lines.push(format!("TimeoutSec={}", s));
        }

        lines.join("\n")
    }

    fn validate(&self) -> Result<(), String> {
        if self.what.is_none() {
            return Err("[Swap] What= is required".into());
        }
        Ok(())
    }
}
