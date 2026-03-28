use super::{HyprlandConfig, HyprlandRenderContext};

/// A Hyprland environment variable declaration.
///
/// Rendered as: `env = KEY,value`
///
/// Environment variables set here are injected into every process Hyprland
/// spawns.  This is the primary mechanism for:
/// - Cursor theme and size
/// - Wayland/XDG hints for toolkits (Qt, GTK, SDL2 …)
/// - Multi-GPU DRM device selection
/// - NVIDIA-specific workarounds
///
/// # Multi-GPU
/// To steer rendering to a particular GPU set the `AQ_DRM_DEVICES` variable
/// to a colon-separated list of `/dev/dri/cardN` paths. The first listed
/// device is used as the primary renderer. Example:
/// ```
/// # use toconfig::hyprland::HyprlandConfig;
/// use toconfig::hyprland::environment::EnvVar;
/// let e = EnvVar::preferred_gpu("/dev/dri/card1:/dev/dri/card0");
/// assert_eq!(e.generate(), "env = AQ_DRM_DEVICES,/dev/dri/card1:/dev/dri/card0");
/// ```
///
/// # Example
/// ```
/// # use toconfig::hyprland::HyprlandConfig;
/// use toconfig::hyprland::environment::EnvVar;
/// let e = EnvVar::new("XCURSOR_SIZE", "24");
/// assert_eq!(e.generate(), "env = XCURSOR_SIZE,24");
/// ```
pub struct EnvVar {
    pub key: String,
    pub value: String,
}

impl EnvVar {
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }

    /// Set `XCURSOR_SIZE` for a consistent hardware cursor size.
    pub fn xcursor_size(size: u32) -> Self {
        Self::new("XCURSOR_SIZE", size.to_string())
    }

    /// Set the cursor theme name.
    pub fn xcursor_theme(theme: impl Into<String>) -> Self {
        Self::new("XCURSOR_THEME", theme)
    }

    /// Hint Qt to use the Wayland backend (`QT_QPA_PLATFORM=wayland`).
    pub fn qt_wayland() -> Self {
        Self::new("QT_QPA_PLATFORM", "wayland")
    }

    /// Set `QT_WAYLAND_DISABLE_WINDOWDECORATION=1` to disable Qt CSD on Wayland.
    pub fn qt_no_csd() -> Self {
        Self::new("QT_WAYLAND_DISABLE_WINDOWDECORATION", "1")
    }

    /// Set `XDG_CURRENT_DESKTOP` for portal compatibility (usually `"Hyprland"`).
    pub fn xdg_current_desktop(de: impl Into<String>) -> Self {
        Self::new("XDG_CURRENT_DESKTOP", de)
    }

    /// Set `XDG_SESSION_TYPE=wayland`.
    pub fn xdg_session_wayland() -> Self {
        Self::new("XDG_SESSION_TYPE", "wayland")
    }

    /// Set `GDK_BACKEND=wayland` to force GTK onto Wayland.
    pub fn gdk_wayland() -> Self {
        Self::new("GDK_BACKEND", "wayland")
    }

    /// Set `SDL_VIDEODRIVER=wayland` so SDL2 apps use Wayland.
    pub fn sdl_wayland() -> Self {
        Self::new("SDL_VIDEODRIVER", "wayland")
    }

    /// Set `CLUTTER_BACKEND=wayland`.
    pub fn clutter_wayland() -> Self {
        Self::new("CLUTTER_BACKEND", "wayland")
    }

    /// Preferred GPU for multi-GPU setups via `AQ_DRM_DEVICES`.
    ///
    /// `devices` is a colon-separated list of DRM card paths, e.g.
    /// `"/dev/dri/card1:/dev/dri/card0"`.  The first card is the render device.
    pub fn preferred_gpu(devices: impl Into<String>) -> Self {
        Self::new("AQ_DRM_DEVICES", devices)
    }

    /// NVIDIA: set `LIBVA_DRIVER_NAME=nvidia` for hardware video decoding.
    pub fn nvidia_libva() -> Self {
        Self::new("LIBVA_DRIVER_NAME", "nvidia")
    }

    /// NVIDIA: set `GBM_BACKEND=nvidia-drm`.
    pub fn nvidia_gbm() -> Self {
        Self::new("GBM_BACKEND", "nvidia-drm")
    }

    /// NVIDIA: set `__GLX_VENDOR_LIBRARY_NAME=nvidia`.
    pub fn nvidia_glx() -> Self {
        Self::new("__GLX_VENDOR_LIBRARY_NAME", "nvidia")
    }

    /// NVIDIA: enable explicit sync (`__GL_GSYNC_ALLOWED=0` + `__NV_PRIME_RENDER_OFFLOAD=1`).
    pub fn nvidia_explicit_sync() -> Self {
        Self::new("NVD_BACKEND", "direct")
    }
}

impl HyprlandConfig for EnvVar {
    fn render(&self, ctx: &HyprlandRenderContext) -> String {
        format!("{}env = {},{}", ctx.indent(), self.key, self.value)
    }

    fn validate(&self) -> Result<(), String> {
        if self.key.is_empty() {
            return Err("EnvVar key cannot be empty".into());
        }
        if self.key.contains(',') {
            return Err(format!(
                "EnvVar key '{}' must not contain commas",
                self.key
            ));
        }
        Ok(())
    }
}
