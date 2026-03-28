use super::section::Section;
use super::{HyprlandConfig, HyprlandRenderContext};

/// XWayland configuration section.
///
/// Rendered as an `xwayland { }` section block.
///
/// XWayland provides backwards compatibility for X11 applications on a Wayland
/// compositor.  The most important option for HiDPI setups is
/// [`XWaylandSection::force_zero_scaling`], which prevents XWayland from
/// inheriting the Wayland output scale and producing blurry windows.
///
/// # Performance note
/// Disabling XWayland entirely (`enabled(false)`) gives a small memory saving
/// and removes the X11 attack surface, but breaks any application that has not
/// been ported to Wayland.
///
/// # Example
/// ```
/// # use toconfig::hyprland::HyprlandConfig;
/// use toconfig::hyprland::xwayland::XWaylandSection;
/// let sec = XWaylandSection::new().force_zero_scaling(true);
/// let out = sec.generate();
/// assert!(out.contains("xwayland {"));
/// assert!(out.contains("force_zero_scaling = true"));
/// ```
#[derive(Default)]
pub struct XWaylandSection {
    /// Enable or disable XWayland entirely.
    pub enabled: Option<bool>,
    /// Force all XWayland windows to report 1× scaling regardless of monitor DPI.
    /// Recommended on HiDPI displays to avoid blurry X11 windows.
    pub force_zero_scaling: Option<bool>,
    /// Use nearest-neighbour (pixelated) scaling instead of bilinear.
    pub use_nearest_neighbor: Option<bool>,
}

impl XWaylandSection {
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable or disable XWayland.  Defaults to enabled when not set.
    pub fn enabled(mut self, v: bool) -> Self {
        self.enabled = Some(v);
        self
    }

    /// Force XWayland to report 1× scaling for all windows.
    /// Recommended for HiDPI setups to avoid blurry XWayland apps.
    pub fn force_zero_scaling(mut self, v: bool) -> Self {
        self.force_zero_scaling = Some(v);
        self
    }

    /// Use nearest-neighbour scaling for XWayland windows (sharper for pixel art).
    pub fn use_nearest_neighbor(mut self, v: bool) -> Self {
        self.use_nearest_neighbor = Some(v);
        self
    }
}

impl HyprlandConfig for XWaylandSection {
    fn render(&self, ctx: &HyprlandRenderContext) -> String {
        let mut sec = Section::new("xwayland");
        if let Some(v) = self.enabled {
            sec.add_pair("enabled", if v { "true" } else { "false" });
        }
        if let Some(v) = self.force_zero_scaling {
            sec.add_pair("force_zero_scaling", if v { "true" } else { "false" });
        }
        if let Some(v) = self.use_nearest_neighbor {
            sec.add_pair("use_nearest_neighbor", if v { "true" } else { "false" });
        }
        sec.render(ctx)
    }
}
