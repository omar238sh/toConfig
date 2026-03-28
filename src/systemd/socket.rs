use super::{SystemdConfig, SystemdRenderContext};

/// The `[Socket]` section for `.socket` unit files.
///
/// Socket activation allows systemd to listen on a socket and start the
/// associated service only when a connection arrives.
///
/// # Example
/// ```
/// # use toconfig::systemd::SystemdConfig;
/// use toconfig::systemd::socket::SocketSection;
///
/// let sock = SocketSection::new()
///     .listen_stream("/run/myapp.sock")
///     .socket_user("myapp")
///     .socket_mode("0660")
///     .accept(false);
///
/// let out = sock.generate();
/// assert!(out.contains("[Socket]"));
/// assert!(out.contains("ListenStream=/run/myapp.sock"));
/// assert!(out.contains("SocketMode=0660"));
/// ```
#[derive(Default)]
pub struct SocketSection {
    pub listen_stream: Vec<String>,
    pub listen_datagram: Vec<String>,
    pub listen_sequential_packet: Vec<String>,
    pub listen_netlink: Vec<String>,
    pub listen_special: Vec<String>,
    pub listen_fifo: Vec<String>,
    pub accept: Option<bool>,
    pub socket_user: Option<String>,
    pub socket_group: Option<String>,
    pub socket_mode: Option<String>,
    pub directory_mode: Option<String>,
    pub service: Option<String>,
    pub pass_credentials: Option<bool>,
    pub pass_security: Option<bool>,
    pub backlog: Option<u32>,
    pub bind_ipv6_only: Option<String>,
    pub max_connections: Option<u32>,
    pub max_connections_per_source: Option<u32>,
    pub keep_alive: Option<bool>,
    pub keep_alive_time_sec: Option<String>,
    pub keep_alive_interval_sec: Option<String>,
    pub keep_alive_probes: Option<u32>,
    pub no_delay: Option<bool>,
    pub priority: Option<i32>,
    pub receive_buffer: Option<String>,
    pub send_buffer: Option<String>,
    pub iptos: Option<i32>,
    pub ipttl: Option<u32>,
    pub pipe_size: Option<String>,
    pub free_bind: Option<bool>,
    pub transparent: Option<bool>,
    pub broadcast: Option<bool>,
    pub pass_packet_info: Option<bool>,
    pub smack_label: Option<String>,
    pub smack_label_ip_in: Option<String>,
    pub smack_label_ip_out: Option<String>,
    pub selinux_context_from_net: Option<bool>,
    pub writable: Option<bool>,
    pub trigger_limit_interval_sec: Option<String>,
    pub trigger_limit_burst: Option<u32>,
}

impl SocketSection {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a `ListenStream=` address (TCP or Unix stream socket).
    pub fn listen_stream(mut self, addr: impl Into<String>) -> Self {
        self.listen_stream.push(addr.into());
        self
    }

    /// Add a `ListenDatagram=` address (UDP or Unix datagram socket).
    pub fn listen_datagram(mut self, addr: impl Into<String>) -> Self {
        self.listen_datagram.push(addr.into());
        self
    }

    /// Add a `ListenSequentialPacket=` address (Unix seqpacket socket).
    pub fn listen_sequential_packet(mut self, addr: impl Into<String>) -> Self {
        self.listen_sequential_packet.push(addr.into());
        self
    }

    /// Add a `ListenNetlink=` address.
    pub fn listen_netlink(mut self, addr: impl Into<String>) -> Self {
        self.listen_netlink.push(addr.into());
        self
    }

    /// Add a `ListenSpecial=` device node.
    pub fn listen_special(mut self, path: impl Into<String>) -> Self {
        self.listen_special.push(path.into());
        self
    }

    /// Add a `ListenFIFO=` FIFO path.
    pub fn listen_fifo(mut self, path: impl Into<String>) -> Self {
        self.listen_fifo.push(path.into());
        self
    }

    /// Set `Accept=` (whether to spawn a service instance per connection).
    pub fn accept(mut self, v: bool) -> Self {
        self.accept = Some(v);
        self
    }

    /// Set `SocketUser=`.
    pub fn socket_user(mut self, u: impl Into<String>) -> Self {
        self.socket_user = Some(u.into());
        self
    }

    /// Set `SocketGroup=`.
    pub fn socket_group(mut self, g: impl Into<String>) -> Self {
        self.socket_group = Some(g.into());
        self
    }

    /// Set `SocketMode=` (octal, e.g. `"0660"`).
    pub fn socket_mode(mut self, mode: impl Into<String>) -> Self {
        self.socket_mode = Some(mode.into());
        self
    }

    /// Set `DirectoryMode=`.
    pub fn directory_mode(mut self, mode: impl Into<String>) -> Self {
        self.directory_mode = Some(mode.into());
        self
    }

    /// Override the activated service with `Service=`.
    pub fn service(mut self, name: impl Into<String>) -> Self {
        self.service = Some(name.into());
        self
    }

    /// Set `PassCredentials=`.
    pub fn pass_credentials(mut self, v: bool) -> Self {
        self.pass_credentials = Some(v);
        self
    }

    /// Set `PassSecurity=`.
    pub fn pass_security(mut self, v: bool) -> Self {
        self.pass_security = Some(v);
        self
    }

    /// Set `Backlog=`.
    pub fn backlog(mut self, n: u32) -> Self {
        self.backlog = Some(n);
        self
    }

    /// Set `BindIPv6Only=` (`"default"`, `"both"`, `"ipv6-only"`).
    pub fn bind_ipv6_only(mut self, v: impl Into<String>) -> Self {
        self.bind_ipv6_only = Some(v.into());
        self
    }

    /// Set `MaxConnections=`.
    pub fn max_connections(mut self, n: u32) -> Self {
        self.max_connections = Some(n);
        self
    }

    /// Set `MaxConnectionsPerSource=`.
    pub fn max_connections_per_source(mut self, n: u32) -> Self {
        self.max_connections_per_source = Some(n);
        self
    }

    /// Set `KeepAlive=`.
    pub fn keep_alive(mut self, v: bool) -> Self {
        self.keep_alive = Some(v);
        self
    }

    /// Set `KeepAliveTimeSec=`.
    pub fn keep_alive_time_sec(mut self, s: impl Into<String>) -> Self {
        self.keep_alive_time_sec = Some(s.into());
        self
    }

    /// Set `NoDelay=`.
    pub fn no_delay(mut self, v: bool) -> Self {
        self.no_delay = Some(v);
        self
    }

    /// Set `FreeBind=` (bind to an address not yet assigned to any interface).
    pub fn free_bind(mut self, v: bool) -> Self {
        self.free_bind = Some(v);
        self
    }

    /// Set `Transparent=` (IP transparent proxy).
    pub fn transparent(mut self, v: bool) -> Self {
        self.transparent = Some(v);
        self
    }

    /// Set `Broadcast=`.
    pub fn broadcast(mut self, v: bool) -> Self {
        self.broadcast = Some(v);
        self
    }

    /// Set `TriggerLimitIntervalSec=` (rate-limit socket activation).
    pub fn trigger_limit_interval_sec(mut self, s: impl Into<String>) -> Self {
        self.trigger_limit_interval_sec = Some(s.into());
        self
    }

    /// Set `TriggerLimitBurst=`.
    pub fn trigger_limit_burst(mut self, n: u32) -> Self {
        self.trigger_limit_burst = Some(n);
        self
    }
}

fn bool_str(b: bool) -> &'static str {
    if b { "yes" } else { "no" }
}

impl SystemdConfig for SocketSection {
    fn render(&self, _ctx: &SystemdRenderContext) -> String {
        let mut lines = vec!["[Socket]".to_string()];

        for addr in &self.listen_stream {
            lines.push(format!("ListenStream={}", addr));
        }
        for addr in &self.listen_datagram {
            lines.push(format!("ListenDatagram={}", addr));
        }
        for addr in &self.listen_sequential_packet {
            lines.push(format!("ListenSequentialPacket={}", addr));
        }
        for addr in &self.listen_netlink {
            lines.push(format!("ListenNetlink={}", addr));
        }
        for path in &self.listen_special {
            lines.push(format!("ListenSpecial={}", path));
        }
        for path in &self.listen_fifo {
            lines.push(format!("ListenFIFO={}", path));
        }
        if let Some(v) = self.accept {
            lines.push(format!("Accept={}", bool_str(v)));
        }
        if let Some(ref u) = self.socket_user {
            lines.push(format!("SocketUser={}", u));
        }
        if let Some(ref g) = self.socket_group {
            lines.push(format!("SocketGroup={}", g));
        }
        if let Some(ref m) = self.socket_mode {
            lines.push(format!("SocketMode={}", m));
        }
        if let Some(ref m) = self.directory_mode {
            lines.push(format!("DirectoryMode={}", m));
        }
        if let Some(ref s) = self.service {
            lines.push(format!("Service={}", s));
        }
        if let Some(v) = self.pass_credentials {
            lines.push(format!("PassCredentials={}", bool_str(v)));
        }
        if let Some(v) = self.pass_security {
            lines.push(format!("PassSecurity={}", bool_str(v)));
        }
        if let Some(n) = self.backlog {
            lines.push(format!("Backlog={}", n));
        }
        if let Some(ref v) = self.bind_ipv6_only {
            lines.push(format!("BindIPv6Only={}", v));
        }
        if let Some(n) = self.max_connections {
            lines.push(format!("MaxConnections={}", n));
        }
        if let Some(n) = self.max_connections_per_source {
            lines.push(format!("MaxConnectionsPerSource={}", n));
        }
        if let Some(v) = self.keep_alive {
            lines.push(format!("KeepAlive={}", bool_str(v)));
        }
        if let Some(ref s) = self.keep_alive_time_sec {
            lines.push(format!("KeepAliveTimeSec={}", s));
        }
        if let Some(ref s) = self.keep_alive_interval_sec {
            lines.push(format!("KeepAliveIntervalSec={}", s));
        }
        if let Some(n) = self.keep_alive_probes {
            lines.push(format!("KeepAliveProbes={}", n));
        }
        if let Some(v) = self.no_delay {
            lines.push(format!("NoDelay={}", bool_str(v)));
        }
        if let Some(n) = self.priority {
            lines.push(format!("Priority={}", n));
        }
        if let Some(ref s) = self.receive_buffer {
            lines.push(format!("ReceiveBuffer={}", s));
        }
        if let Some(ref s) = self.send_buffer {
            lines.push(format!("SendBuffer={}", s));
        }
        if let Some(v) = self.free_bind {
            lines.push(format!("FreeBind={}", bool_str(v)));
        }
        if let Some(v) = self.transparent {
            lines.push(format!("Transparent={}", bool_str(v)));
        }
        if let Some(v) = self.broadcast {
            lines.push(format!("Broadcast={}", bool_str(v)));
        }
        if let Some(v) = self.pass_packet_info {
            lines.push(format!("PassPacketInfo={}", bool_str(v)));
        }
        if let Some(ref s) = self.smack_label {
            lines.push(format!("SmackLabel={}", s));
        }
        if let Some(ref s) = self.smack_label_ip_in {
            lines.push(format!("SmackLabelIPIn={}", s));
        }
        if let Some(ref s) = self.smack_label_ip_out {
            lines.push(format!("SmackLabelIPOut={}", s));
        }
        if let Some(v) = self.selinux_context_from_net {
            lines.push(format!("SELinuxContextFromNet={}", bool_str(v)));
        }
        if let Some(v) = self.writable {
            lines.push(format!("Writable={}", bool_str(v)));
        }
        if let Some(ref s) = self.trigger_limit_interval_sec {
            lines.push(format!("TriggerLimitIntervalSec={}", s));
        }
        if let Some(n) = self.trigger_limit_burst {
            lines.push(format!("TriggerLimitBurst={}", n));
        }

        lines.join("\n")
    }

    fn validate(&self) -> Result<(), String> {
        let has_listener = !self.listen_stream.is_empty()
            || !self.listen_datagram.is_empty()
            || !self.listen_sequential_packet.is_empty()
            || !self.listen_netlink.is_empty()
            || !self.listen_special.is_empty()
            || !self.listen_fifo.is_empty();
        if !has_listener {
            return Err("[Socket] at least one Listen* directive is required".into());
        }
        Ok(())
    }
}
