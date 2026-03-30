use super::{SystemdConfig, SystemdRenderContext};

/// The `[Timer]` section for `.timer` unit files.
///
/// Timers allow scheduling units to activate at specific times or intervals,
/// similar to cron but integrated with the systemd journal.
///
/// # Example
/// ```
/// # use toconfig::systemd::SystemdConfig;
/// use toconfig::systemd::timer::TimerSection;
///
/// // Run daily at midnight
/// let daily = TimerSection::new()
///     .on_calendar("daily")
///     .persistent(true);
///
/// let out = daily.generate();
/// assert!(out.contains("[Timer]"));
/// assert!(out.contains("OnCalendar=daily"));
/// assert!(out.contains("Persistent=yes"));
///
/// // Run 5 minutes after boot
/// let boot = TimerSection::new().on_boot_sec("5min");
/// assert!(boot.generate().contains("OnBootSec=5min"));
/// ```
#[derive(Default)]
pub struct TimerSection {
    pub on_active_sec: Option<String>,
    pub on_boot_sec: Option<String>,
    pub on_startup_sec: Option<String>,
    pub on_unit_active_sec: Option<String>,
    pub on_unit_inactive_sec: Option<String>,
    pub on_calendar: Option<String>,
    pub on_clock_change: Option<bool>,
    pub on_timezone_change: Option<bool>,
    pub accuracy_sec: Option<String>,
    pub randomized_delay_sec: Option<String>,
    pub fixed_random_delay: Option<bool>,
    pub on_clock_change_sec: Option<String>,
    pub unit: Option<String>,
    pub persistent: Option<bool>,
    pub wake_system: Option<bool>,
    pub remain_after_elapse: Option<bool>,
}

impl TimerSection {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set `OnActiveSec=` — elapsed time after the timer unit itself activates.
    pub fn on_active_sec(mut self, s: impl Into<String>) -> Self {
        self.on_active_sec = Some(s.into());
        self
    }

    /// Set `OnBootSec=` — elapsed time after system boot.
    pub fn on_boot_sec(mut self, s: impl Into<String>) -> Self {
        self.on_boot_sec = Some(s.into());
        self
    }

    /// Set `OnStartupSec=` — elapsed time after the service manager starts.
    pub fn on_startup_sec(mut self, s: impl Into<String>) -> Self {
        self.on_startup_sec = Some(s.into());
        self
    }

    /// Set `OnUnitActiveSec=` — elapsed time after the unit last activated.
    pub fn on_unit_active_sec(mut self, s: impl Into<String>) -> Self {
        self.on_unit_active_sec = Some(s.into());
        self
    }

    /// Set `OnUnitInactiveSec=` — elapsed time after the unit last deactivated.
    pub fn on_unit_inactive_sec(mut self, s: impl Into<String>) -> Self {
        self.on_unit_inactive_sec = Some(s.into());
        self
    }

    /// Set `OnCalendar=` calendar event expression (e.g. `"daily"`, `"Mon *-*-* 04:00:00"`).
    pub fn on_calendar(mut self, expr: impl Into<String>) -> Self {
        self.on_calendar = Some(expr.into());
        self
    }

    /// Set `OnClockChange=` — trigger when the realtime clock jumps.
    pub fn on_clock_change(mut self, v: bool) -> Self {
        self.on_clock_change = Some(v);
        self
    }

    /// Set `OnTimezoneChange=` — trigger when the local timezone changes.
    pub fn on_timezone_change(mut self, v: bool) -> Self {
        self.on_timezone_change = Some(v);
        self
    }

    /// Set `AccuracySec=` — allowed scheduling jitter (default: `1min`).
    pub fn accuracy_sec(mut self, s: impl Into<String>) -> Self {
        self.accuracy_sec = Some(s.into());
        self
    }

    /// Set `RandomizedDelaySec=` — add a random offset up to this value.
    pub fn randomized_delay_sec(mut self, s: impl Into<String>) -> Self {
        self.randomized_delay_sec = Some(s.into());
        self
    }

    /// Set `FixedRandomDelay=` — make the random delay reproducible per machine.
    pub fn fixed_random_delay(mut self, v: bool) -> Self {
        self.fixed_random_delay = Some(v);
        self
    }

    /// Override the activated unit with `Unit=`.
    pub fn unit(mut self, name: impl Into<String>) -> Self {
        self.unit = Some(name.into());
        self
    }

    /// Set `Persistent=` — catch up missed runs after downtime.
    pub fn persistent(mut self, v: bool) -> Self {
        self.persistent = Some(v);
        self
    }

    /// Set `WakeSystem=` — wake the system from suspend to fire the timer.
    pub fn wake_system(mut self, v: bool) -> Self {
        self.wake_system = Some(v);
        self
    }

    /// Set `RemainAfterElapse=` — keep the timer active after it fires.
    pub fn remain_after_elapse(mut self, v: bool) -> Self {
        self.remain_after_elapse = Some(v);
        self
    }
}

fn bool_str(b: bool) -> &'static str {
    if b { "yes" } else { "no" }
}

impl SystemdConfig for TimerSection {
    fn render(&self, _ctx: &SystemdRenderContext) -> String {
        let mut lines = vec!["[Timer]".to_string()];

        if let Some(ref s) = self.on_active_sec {
            lines.push(format!("OnActiveSec={}", s));
        }
        if let Some(ref s) = self.on_boot_sec {
            lines.push(format!("OnBootSec={}", s));
        }
        if let Some(ref s) = self.on_startup_sec {
            lines.push(format!("OnStartupSec={}", s));
        }
        if let Some(ref s) = self.on_unit_active_sec {
            lines.push(format!("OnUnitActiveSec={}", s));
        }
        if let Some(ref s) = self.on_unit_inactive_sec {
            lines.push(format!("OnUnitInactiveSec={}", s));
        }
        if let Some(ref s) = self.on_calendar {
            lines.push(format!("OnCalendar={}", s));
        }
        if let Some(v) = self.on_clock_change {
            lines.push(format!("OnClockChange={}", bool_str(v)));
        }
        if let Some(v) = self.on_timezone_change {
            lines.push(format!("OnTimezoneChange={}", bool_str(v)));
        }
        if let Some(ref s) = self.accuracy_sec {
            lines.push(format!("AccuracySec={}", s));
        }
        if let Some(ref s) = self.randomized_delay_sec {
            lines.push(format!("RandomizedDelaySec={}", s));
        }
        if let Some(v) = self.fixed_random_delay {
            lines.push(format!("FixedRandomDelay={}", bool_str(v)));
        }
        if let Some(ref u) = self.unit {
            lines.push(format!("Unit={}", u));
        }
        if let Some(v) = self.persistent {
            lines.push(format!("Persistent={}", bool_str(v)));
        }
        if let Some(v) = self.wake_system {
            lines.push(format!("WakeSystem={}", bool_str(v)));
        }
        if let Some(v) = self.remain_after_elapse {
            lines.push(format!("RemainAfterElapse={}", bool_str(v)));
        }

        lines.join("\n")
    }

    fn validate(&self) -> Result<(), String> {
        let has_trigger = self.on_active_sec.is_some()
            || self.on_boot_sec.is_some()
            || self.on_startup_sec.is_some()
            || self.on_unit_active_sec.is_some()
            || self.on_unit_inactive_sec.is_some()
            || self.on_calendar.is_some()
            || self.on_clock_change.is_some()
            || self.on_timezone_change.is_some();
        if !has_trigger {
            return Err("[Timer] at least one On* trigger directive is required".into());
        }
        Ok(())
    }
}
