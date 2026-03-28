use super::{SystemdConfig, SystemdRenderContext};

/// Service activation type for `Type=`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceType {
    /// Default. The service is considered ready when the main process exits setup.
    Simple,
    /// Like `simple`, but the process must call `sd_notify(READY=1)` before
    /// systemd marks it as active.  Requires `NotifyAccess`.
    Exec,
    /// The process forks; the parent exits once setup is complete.
    Forking,
    /// The process runs once; systemd waits for it to exit.
    Oneshot,
    /// For D-Bus services; systemd waits for the bus name to be acquired.
    Dbus,
    /// The process sends a `sd_notify(READY=1)` message.
    Notify,
    /// Like `notify`, but uses a file descriptor for notification.
    NotifyReload,
    /// Scheduled to run when CPU is idle.
    Idle,
}

impl ServiceType {
    fn as_str(&self) -> &'static str {
        match self {
            ServiceType::Simple => "simple",
            ServiceType::Exec => "exec",
            ServiceType::Forking => "forking",
            ServiceType::Oneshot => "oneshot",
            ServiceType::Dbus => "dbus",
            ServiceType::Notify => "notify",
            ServiceType::NotifyReload => "notify-reload",
            ServiceType::Idle => "idle",
        }
    }
}

/// Restart policy for `Restart=`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Restart {
    No,
    OnSuccess,
    OnFailure,
    OnAbnormal,
    OnWatchdog,
    OnAbort,
    Always,
}

impl Restart {
    fn as_str(&self) -> &'static str {
        match self {
            Restart::No => "no",
            Restart::OnSuccess => "on-success",
            Restart::OnFailure => "on-failure",
            Restart::OnAbnormal => "on-abnormal",
            Restart::OnWatchdog => "on-watchdog",
            Restart::OnAbort => "on-abort",
            Restart::Always => "always",
        }
    }
}

/// The `[Service]` section for `.service` unit files.
///
/// # Example
/// ```
/// # use toconfig::systemd::SystemdConfig;
/// use toconfig::systemd::service::{ServiceSection, ServiceType, Restart};
///
/// let svc = ServiceSection::new()
///     .service_type(ServiceType::Simple)
///     .exec_start("/usr/bin/myapp --config /etc/myapp.conf")
///     .restart(Restart::OnFailure)
///     .restart_sec("5s")
///     .user("myapp")
///     .working_directory("/var/lib/myapp")
///     .environment("RUST_LOG=info");
///
/// let out = svc.generate();
/// assert!(out.contains("[Service]"));
/// assert!(out.contains("Type=simple"));
/// assert!(out.contains("ExecStart=/usr/bin/myapp"));
/// assert!(out.contains("Restart=on-failure"));
/// ```
#[derive(Default)]
pub struct ServiceSection {
    pub service_type: Option<ServiceType>,
    pub exec_start: Option<String>,
    pub exec_start_pre: Vec<String>,
    pub exec_start_post: Vec<String>,
    pub exec_stop: Option<String>,
    pub exec_stop_post: Option<String>,
    pub exec_reload: Option<String>,
    pub restart: Option<Restart>,
    pub restart_sec: Option<String>,
    pub timeout_start_sec: Option<String>,
    pub timeout_stop_sec: Option<String>,
    pub timeout_sec: Option<String>,
    pub watchdog_sec: Option<String>,
    pub user: Option<String>,
    pub group: Option<String>,
    pub working_directory: Option<String>,
    pub root_directory: Option<String>,
    pub environment: Vec<String>,
    pub environment_file: Vec<String>,
    pub standard_output: Option<String>,
    pub standard_error: Option<String>,
    pub pid_file: Option<String>,
    pub bus_name: Option<String>,
    pub notify_access: Option<String>,
    pub remain_after_exit: Option<bool>,
    pub kill_mode: Option<String>,
    pub kill_signal: Option<String>,
    pub private_tmp: Option<bool>,
    pub private_network: Option<bool>,
    pub protect_system: Option<String>,
    pub protect_home: Option<String>,
    pub no_new_privileges: Option<bool>,
    pub capability_bounding_set: Vec<String>,
    pub ambient_capabilities: Vec<String>,
    pub memory_max: Option<String>,
    pub cpu_quota: Option<String>,
    pub tasks_max: Option<String>,
    pub limit_nofile: Option<String>,
    pub limit_nproc: Option<String>,
    pub supplementary_groups: Vec<String>,
    pub state_directory: Option<String>,
    pub cache_directory: Option<String>,
    pub logs_directory: Option<String>,
    pub runtime_directory: Option<String>,
    pub config_directory: Option<String>,
}

impl ServiceSection {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set `Type=`.
    pub fn service_type(mut self, t: ServiceType) -> Self {
        self.service_type = Some(t);
        self
    }

    /// Set `ExecStart=`.
    pub fn exec_start(mut self, cmd: impl Into<String>) -> Self {
        self.exec_start = Some(cmd.into());
        self
    }

    /// Add an `ExecStartPre=` command.
    pub fn exec_start_pre(mut self, cmd: impl Into<String>) -> Self {
        self.exec_start_pre.push(cmd.into());
        self
    }

    /// Add an `ExecStartPost=` command.
    pub fn exec_start_post(mut self, cmd: impl Into<String>) -> Self {
        self.exec_start_post.push(cmd.into());
        self
    }

    /// Set `ExecStop=`.
    pub fn exec_stop(mut self, cmd: impl Into<String>) -> Self {
        self.exec_stop = Some(cmd.into());
        self
    }

    /// Set `ExecStopPost=`.
    pub fn exec_stop_post(mut self, cmd: impl Into<String>) -> Self {
        self.exec_stop_post = Some(cmd.into());
        self
    }

    /// Set `ExecReload=`.
    pub fn exec_reload(mut self, cmd: impl Into<String>) -> Self {
        self.exec_reload = Some(cmd.into());
        self
    }

    /// Set `Restart=`.
    pub fn restart(mut self, r: Restart) -> Self {
        self.restart = Some(r);
        self
    }

    /// Set `RestartSec=` (e.g. `"5s"`, `"2min"`).
    pub fn restart_sec(mut self, s: impl Into<String>) -> Self {
        self.restart_sec = Some(s.into());
        self
    }

    /// Set `TimeoutStartSec=`.
    pub fn timeout_start_sec(mut self, s: impl Into<String>) -> Self {
        self.timeout_start_sec = Some(s.into());
        self
    }

    /// Set `TimeoutStopSec=`.
    pub fn timeout_stop_sec(mut self, s: impl Into<String>) -> Self {
        self.timeout_stop_sec = Some(s.into());
        self
    }

    /// Set `TimeoutSec=` (sets both start and stop).
    pub fn timeout_sec(mut self, s: impl Into<String>) -> Self {
        self.timeout_sec = Some(s.into());
        self
    }

    /// Set `WatchdogSec=`.
    pub fn watchdog_sec(mut self, s: impl Into<String>) -> Self {
        self.watchdog_sec = Some(s.into());
        self
    }

    /// Set `User=`.
    pub fn user(mut self, u: impl Into<String>) -> Self {
        self.user = Some(u.into());
        self
    }

    /// Set `Group=`.
    pub fn group(mut self, g: impl Into<String>) -> Self {
        self.group = Some(g.into());
        self
    }

    /// Set `WorkingDirectory=`.
    pub fn working_directory(mut self, d: impl Into<String>) -> Self {
        self.working_directory = Some(d.into());
        self
    }

    /// Set `RootDirectory=`.
    pub fn root_directory(mut self, d: impl Into<String>) -> Self {
        self.root_directory = Some(d.into());
        self
    }

    /// Add an `Environment=` assignment, e.g. `"KEY=VALUE"`.
    pub fn environment(mut self, kv: impl Into<String>) -> Self {
        self.environment.push(kv.into());
        self
    }

    /// Add an `EnvironmentFile=` path.
    pub fn environment_file(mut self, path: impl Into<String>) -> Self {
        self.environment_file.push(path.into());
        self
    }

    /// Set `StandardOutput=` (e.g. `"journal"`, `"null"`, `"syslog"`).
    pub fn standard_output(mut self, v: impl Into<String>) -> Self {
        self.standard_output = Some(v.into());
        self
    }

    /// Set `StandardError=`.
    pub fn standard_error(mut self, v: impl Into<String>) -> Self {
        self.standard_error = Some(v.into());
        self
    }

    /// Set `PIDFile=`.
    pub fn pid_file(mut self, path: impl Into<String>) -> Self {
        self.pid_file = Some(path.into());
        self
    }

    /// Set `BusName=` (required for `Type=dbus`).
    pub fn bus_name(mut self, name: impl Into<String>) -> Self {
        self.bus_name = Some(name.into());
        self
    }

    /// Set `NotifyAccess=` (`"main"`, `"exec"`, `"all"`, `"none"`).
    pub fn notify_access(mut self, v: impl Into<String>) -> Self {
        self.notify_access = Some(v.into());
        self
    }

    /// Set `RemainAfterExit=`.
    pub fn remain_after_exit(mut self, v: bool) -> Self {
        self.remain_after_exit = Some(v);
        self
    }

    /// Set `KillMode=` (`"control-group"`, `"process"`, `"mixed"`, `"none"`).
    pub fn kill_mode(mut self, m: impl Into<String>) -> Self {
        self.kill_mode = Some(m.into());
        self
    }

    /// Set `KillSignal=` (e.g. `"SIGTERM"`, `"SIGINT"`).
    pub fn kill_signal(mut self, sig: impl Into<String>) -> Self {
        self.kill_signal = Some(sig.into());
        self
    }

    /// Set `PrivateTmp=`.
    pub fn private_tmp(mut self, v: bool) -> Self {
        self.private_tmp = Some(v);
        self
    }

    /// Set `PrivateNetwork=`.
    pub fn private_network(mut self, v: bool) -> Self {
        self.private_network = Some(v);
        self
    }

    /// Set `ProtectSystem=` (`"strict"`, `"full"`, `"yes"`, `"no"`).
    pub fn protect_system(mut self, v: impl Into<String>) -> Self {
        self.protect_system = Some(v.into());
        self
    }

    /// Set `ProtectHome=` (`"yes"`, `"read-only"`, `"tmpfs"`, `"no"`).
    pub fn protect_home(mut self, v: impl Into<String>) -> Self {
        self.protect_home = Some(v.into());
        self
    }

    /// Set `NoNewPrivileges=`.
    pub fn no_new_privileges(mut self, v: bool) -> Self {
        self.no_new_privileges = Some(v);
        self
    }

    /// Add a `CapabilityBoundingSet=` capability (e.g. `"CAP_NET_BIND_SERVICE"`).
    pub fn capability_bounding_set(mut self, cap: impl Into<String>) -> Self {
        self.capability_bounding_set.push(cap.into());
        self
    }

    /// Add an `AmbientCapabilities=` capability.
    pub fn ambient_capabilities(mut self, cap: impl Into<String>) -> Self {
        self.ambient_capabilities.push(cap.into());
        self
    }

    /// Set `MemoryMax=` (e.g. `"512M"`, `"1G"`).
    pub fn memory_max(mut self, v: impl Into<String>) -> Self {
        self.memory_max = Some(v.into());
        self
    }

    /// Set `CPUQuota=` (e.g. `"50%"`).
    pub fn cpu_quota(mut self, v: impl Into<String>) -> Self {
        self.cpu_quota = Some(v.into());
        self
    }

    /// Set `TasksMax=`.
    pub fn tasks_max(mut self, v: impl Into<String>) -> Self {
        self.tasks_max = Some(v.into());
        self
    }

    /// Set `LimitNOFILE=`.
    pub fn limit_nofile(mut self, v: impl Into<String>) -> Self {
        self.limit_nofile = Some(v.into());
        self
    }

    /// Set `LimitNPROC=`.
    pub fn limit_nproc(mut self, v: impl Into<String>) -> Self {
        self.limit_nproc = Some(v.into());
        self
    }

    /// Add a `SupplementaryGroups=` group.
    pub fn supplementary_groups(mut self, group: impl Into<String>) -> Self {
        self.supplementary_groups.push(group.into());
        self
    }

    /// Set `StateDirectory=` (relative path under `/var/lib/`).
    pub fn state_directory(mut self, d: impl Into<String>) -> Self {
        self.state_directory = Some(d.into());
        self
    }

    /// Set `CacheDirectory=`.
    pub fn cache_directory(mut self, d: impl Into<String>) -> Self {
        self.cache_directory = Some(d.into());
        self
    }

    /// Set `LogsDirectory=`.
    pub fn logs_directory(mut self, d: impl Into<String>) -> Self {
        self.logs_directory = Some(d.into());
        self
    }

    /// Set `RuntimeDirectory=`.
    pub fn runtime_directory(mut self, d: impl Into<String>) -> Self {
        self.runtime_directory = Some(d.into());
        self
    }

    /// Set `ConfigurationDirectory=`.
    pub fn config_directory(mut self, d: impl Into<String>) -> Self {
        self.config_directory = Some(d.into());
        self
    }
}

fn bool_str(b: bool) -> &'static str {
    if b { "yes" } else { "no" }
}

impl SystemdConfig for ServiceSection {
    fn render(&self, _ctx: &SystemdRenderContext) -> String {
        let mut lines = vec!["[Service]".to_string()];

        if let Some(ref t) = self.service_type {
            lines.push(format!("Type={}", t.as_str()));
        }
        if let Some(ref cmd) = self.exec_start {
            lines.push(format!("ExecStart={}", cmd));
        }
        for cmd in &self.exec_start_pre {
            lines.push(format!("ExecStartPre={}", cmd));
        }
        for cmd in &self.exec_start_post {
            lines.push(format!("ExecStartPost={}", cmd));
        }
        if let Some(ref cmd) = self.exec_stop {
            lines.push(format!("ExecStop={}", cmd));
        }
        if let Some(ref cmd) = self.exec_stop_post {
            lines.push(format!("ExecStopPost={}", cmd));
        }
        if let Some(ref cmd) = self.exec_reload {
            lines.push(format!("ExecReload={}", cmd));
        }
        if let Some(ref r) = self.restart {
            lines.push(format!("Restart={}", r.as_str()));
        }
        if let Some(ref s) = self.restart_sec {
            lines.push(format!("RestartSec={}", s));
        }
        if let Some(ref s) = self.timeout_start_sec {
            lines.push(format!("TimeoutStartSec={}", s));
        }
        if let Some(ref s) = self.timeout_stop_sec {
            lines.push(format!("TimeoutStopSec={}", s));
        }
        if let Some(ref s) = self.timeout_sec {
            lines.push(format!("TimeoutSec={}", s));
        }
        if let Some(ref s) = self.watchdog_sec {
            lines.push(format!("WatchdogSec={}", s));
        }
        if let Some(ref u) = self.user {
            lines.push(format!("User={}", u));
        }
        if let Some(ref g) = self.group {
            lines.push(format!("Group={}", g));
        }
        if !self.supplementary_groups.is_empty() {
            lines.push(format!(
                "SupplementaryGroups={}",
                self.supplementary_groups.join(" ")
            ));
        }
        if let Some(ref d) = self.working_directory {
            lines.push(format!("WorkingDirectory={}", d));
        }
        if let Some(ref d) = self.root_directory {
            lines.push(format!("RootDirectory={}", d));
        }
        for kv in &self.environment {
            lines.push(format!("Environment={}", kv));
        }
        for path in &self.environment_file {
            lines.push(format!("EnvironmentFile={}", path));
        }
        if let Some(ref v) = self.standard_output {
            lines.push(format!("StandardOutput={}", v));
        }
        if let Some(ref v) = self.standard_error {
            lines.push(format!("StandardError={}", v));
        }
        if let Some(ref p) = self.pid_file {
            lines.push(format!("PIDFile={}", p));
        }
        if let Some(ref n) = self.bus_name {
            lines.push(format!("BusName={}", n));
        }
        if let Some(ref v) = self.notify_access {
            lines.push(format!("NotifyAccess={}", v));
        }
        if let Some(v) = self.remain_after_exit {
            lines.push(format!("RemainAfterExit={}", bool_str(v)));
        }
        if let Some(ref m) = self.kill_mode {
            lines.push(format!("KillMode={}", m));
        }
        if let Some(ref s) = self.kill_signal {
            lines.push(format!("KillSignal={}", s));
        }
        if let Some(v) = self.private_tmp {
            lines.push(format!("PrivateTmp={}", bool_str(v)));
        }
        if let Some(v) = self.private_network {
            lines.push(format!("PrivateNetwork={}", bool_str(v)));
        }
        if let Some(ref v) = self.protect_system {
            lines.push(format!("ProtectSystem={}", v));
        }
        if let Some(ref v) = self.protect_home {
            lines.push(format!("ProtectHome={}", v));
        }
        if let Some(v) = self.no_new_privileges {
            lines.push(format!("NoNewPrivileges={}", bool_str(v)));
        }
        if !self.capability_bounding_set.is_empty() {
            lines.push(format!(
                "CapabilityBoundingSet={}",
                self.capability_bounding_set.join(" ")
            ));
        }
        if !self.ambient_capabilities.is_empty() {
            lines.push(format!(
                "AmbientCapabilities={}",
                self.ambient_capabilities.join(" ")
            ));
        }
        if let Some(ref v) = self.memory_max {
            lines.push(format!("MemoryMax={}", v));
        }
        if let Some(ref v) = self.cpu_quota {
            lines.push(format!("CPUQuota={}", v));
        }
        if let Some(ref v) = self.tasks_max {
            lines.push(format!("TasksMax={}", v));
        }
        if let Some(ref v) = self.limit_nofile {
            lines.push(format!("LimitNOFILE={}", v));
        }
        if let Some(ref v) = self.limit_nproc {
            lines.push(format!("LimitNPROC={}", v));
        }
        if let Some(ref d) = self.state_directory {
            lines.push(format!("StateDirectory={}", d));
        }
        if let Some(ref d) = self.cache_directory {
            lines.push(format!("CacheDirectory={}", d));
        }
        if let Some(ref d) = self.logs_directory {
            lines.push(format!("LogsDirectory={}", d));
        }
        if let Some(ref d) = self.runtime_directory {
            lines.push(format!("RuntimeDirectory={}", d));
        }
        if let Some(ref d) = self.config_directory {
            lines.push(format!("ConfigurationDirectory={}", d));
        }

        lines.join("\n")
    }

    fn validate(&self) -> Result<(), String> {
        if self.exec_start.is_none()
            && self.service_type != Some(ServiceType::Oneshot)
            && self.service_type != Some(ServiceType::Forking)
        {
            return Err("[Service] ExecStart is required (unless Type=oneshot or Type=forking)".into());
        }
        if self.service_type == Some(ServiceType::Dbus) && self.bus_name.is_none() {
            return Err("[Service] BusName= is required when Type=dbus".into());
        }
        Ok(())
    }
}
