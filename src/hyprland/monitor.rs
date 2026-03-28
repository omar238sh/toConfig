use super::{HyprlandConfig, HyprlandRenderContext};

/// Monitor configuration line.
///
/// Rendered as: `monitor=name,resolution,position,scale[,extra...]`
///
/// # Notes on Multi-GPU
/// When using multiple GPUs, pair this with [`crate::hyprland::EnvVar::preferred_gpu`]
/// to route rendering to a specific DRM node, and use separate `MonitorConfig` entries
/// per output. VRR (Variable Refresh Rate) can be enabled per-monitor via [`MonitorConfig::vrr`].
///
/// # Tearing
/// Per-monitor VRR mode 1 (`vrr(1)`) enables adaptive sync / tearing support.
/// Combine with a window rule `immediate` class matcher for game windows.
///
/// # Example
/// ```
/// # use toconfig::hyprland::HyprlandConfig;
/// use toconfig::hyprland::monitor::MonitorConfig;
///
/// // Named monitor at 1080p 60 Hz, 1× scaling
/// let m = MonitorConfig::new("eDP-1", "1920x1080@60", "0x0", 1.0);
/// assert_eq!(m.generate(), "monitor=eDP-1,1920x1080@60,0x0,1");
///
/// // Catch-all fallback
/// let fallback = MonitorConfig::auto();
/// assert_eq!(fallback.generate(), "monitor=,preferred,auto,1");
///
/// // HiDPI with 90-degree rotation
/// let hidpi = MonitorConfig::new("DP-1", "3840x2160@60", "1920x0", 2.0).transform(1);
/// assert!(hidpi.generate().contains("transform:1"));
/// ```
pub struct MonitorConfig {
    pub name: String,
    /// Resolution string, e.g. `"1920x1080@60"`, `"preferred"`, `"highres"`, `"highrr"`.
    pub resolution: String,
    /// Position string, e.g. `"0x0"`, `"auto"`, `"auto-left"`, `"auto-right"`.
    pub position: String,
    pub scale: f32,
    /// Transform: 0=normal, 1=90°, 2=180°, 3=270°, 4=flipped, 5=flipped+90°, etc.
    pub transform: Option<u8>,
    /// Mirror the named monitor: `mirror:<name>`.
    pub mirror: Option<String>,
    /// Bit depth override (e.g. 10 for HDR).
    pub bitdepth: Option<u8>,
    /// Variable Refresh Rate: 0=off, 1=on, 2=fullscreen only.
    pub vrr: Option<u8>,
}

impl MonitorConfig {
    /// Create a monitor rule with the four required fields.
    pub fn new(
        name: impl Into<String>,
        resolution: impl Into<String>,
        position: impl Into<String>,
        scale: f32,
    ) -> Self {
        Self {
            name: name.into(),
            resolution: resolution.into(),
            position: position.into(),
            scale,
            transform: None,
            mirror: None,
            bitdepth: None,
            vrr: None,
        }
    }

    /// Catch-all rule: `monitor=,preferred,auto,1`
    pub fn auto() -> Self {
        Self::new("", "preferred", "auto", 1.0)
    }

    /// Set the display transform (rotation/flip). Valid values: 0–7.
    pub fn transform(mut self, t: u8) -> Self {
        self.transform = Some(t);
        self
    }

    /// Mirror another monitor by name.
    pub fn mirror(mut self, source: impl Into<String>) -> Self {
        self.mirror = Some(source.into());
        self
    }

    /// Override bit depth (e.g. `10` for HDR / 10-bit colour).
    pub fn bitdepth(mut self, bits: u8) -> Self {
        self.bitdepth = Some(bits);
        self
    }

    /// Enable Variable Refresh Rate.  0 = off, 1 = on, 2 = fullscreen only.
    pub fn vrr(mut self, mode: u8) -> Self {
        self.vrr = Some(mode);
        self
    }
}

impl HyprlandConfig for MonitorConfig {
    fn render(&self, ctx: &HyprlandRenderContext) -> String {
        let scale_str = if self.scale.fract() == 0.0 {
            format!("{}", self.scale as i32)
        } else {
            format!("{}", self.scale)
        };

        let mut parts = vec![
            self.name.clone(),
            self.resolution.clone(),
            self.position.clone(),
            scale_str,
        ];

        if let Some(t) = self.transform {
            parts.push(format!("transform:{}", t));
        }
        if let Some(ref m) = self.mirror {
            parts.push(format!("mirror:{}", m));
        }
        if let Some(b) = self.bitdepth {
            parts.push(format!("bitdepth:{}", b));
        }
        if let Some(v) = self.vrr {
            parts.push(format!("vrr:{}", v));
        }

        format!("{}monitor={}", ctx.indent(), parts.join(","))
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(t) = self.transform
            && t > 7
        {
            return Err(format!(
                "Monitor transform {} is out of range (valid: 0–7)",
                t
            ));
        }
        if let Some(v) = self.vrr
            && v > 2
        {
            return Err(format!("Monitor vrr {} is out of range (valid: 0–2)", v));
        }
        Ok(())
    }
}
