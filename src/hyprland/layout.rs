use super::section::Section;
use super::{HyprlandConfig, HyprlandRenderContext};

fn bool_str(v: bool) -> &'static str {
    if v {
        "true"
    } else {
        "false"
    }
}

/// Dwindle tiling layout configuration.
///
/// Rendered as a `dwindle { }` section block.
///
/// The dwindle layout recursively splits the screen in alternating
/// horizontal/vertical halves — similar to bspwm.  It is the default layout.
///
/// # Example
/// ```
/// # use toconfig::hyprland::HyprlandConfig;
/// use toconfig::hyprland::layout::DwindleLayout;
/// let d = DwindleLayout::new().pseudotile(true).preserve_split(true);
/// let out = d.generate();
/// assert!(out.contains("dwindle {"));
/// assert!(out.contains("pseudotile = true"));
/// ```
#[derive(Default)]
pub struct DwindleLayout {
    pub pseudotile: Option<bool>,
    pub preserve_split: Option<bool>,
    /// 0 = inherit last, 1 = always split to the left, 2 = always split to the right.
    pub force_split: Option<u8>,
    pub smart_split: Option<bool>,
    pub smart_resizing: Option<bool>,
    /// 0 = gaps always, 1 = no gaps when one window, 2 = no gaps when one tiled window.
    pub no_gaps_when_only: Option<u8>,
    pub use_active_for_splits: Option<bool>,
    pub default_split_ratio: Option<f32>,
    pub split_width_multiplier: Option<f32>,
    pub special_scale_factor: Option<f32>,
}

impl DwindleLayout {
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable pseudo-tiling (window acts tiled but keeps its floating size).
    pub fn pseudotile(mut self, v: bool) -> Self {
        self.pseudotile = Some(v);
        self
    }

    /// Preserve the split direction when a window is removed.
    pub fn preserve_split(mut self, v: bool) -> Self {
        self.preserve_split = Some(v);
        self
    }

    /// Force split direction: 0 = last used, 1 = always left/top, 2 = always right/bottom.
    pub fn force_split(mut self, v: u8) -> Self {
        self.force_split = Some(v);
        self
    }

    pub fn smart_split(mut self, v: bool) -> Self {
        self.smart_split = Some(v);
        self
    }

    pub fn smart_resizing(mut self, v: bool) -> Self {
        self.smart_resizing = Some(v);
        self
    }

    pub fn no_gaps_when_only(mut self, v: u8) -> Self {
        self.no_gaps_when_only = Some(v);
        self
    }

    pub fn use_active_for_splits(mut self, v: bool) -> Self {
        self.use_active_for_splits = Some(v);
        self
    }

    pub fn default_split_ratio(mut self, v: f32) -> Self {
        self.default_split_ratio = Some(v);
        self
    }

    pub fn split_width_multiplier(mut self, v: f32) -> Self {
        self.split_width_multiplier = Some(v);
        self
    }
}

impl HyprlandConfig for DwindleLayout {
    fn render(&self, ctx: &HyprlandRenderContext) -> String {
        let mut sec = Section::new("dwindle");
        if let Some(v) = self.pseudotile {
            sec.add_pair("pseudotile", bool_str(v));
        }
        if let Some(v) = self.preserve_split {
            sec.add_pair("preserve_split", bool_str(v));
        }
        if let Some(v) = self.force_split {
            sec.add_pair("force_split", v.to_string());
        }
        if let Some(v) = self.smart_split {
            sec.add_pair("smart_split", bool_str(v));
        }
        if let Some(v) = self.smart_resizing {
            sec.add_pair("smart_resizing", bool_str(v));
        }
        if let Some(v) = self.no_gaps_when_only {
            sec.add_pair("no_gaps_when_only", v.to_string());
        }
        if let Some(v) = self.use_active_for_splits {
            sec.add_pair("use_active_for_splits", bool_str(v));
        }
        if let Some(v) = self.default_split_ratio {
            sec.add_pair("default_split_ratio", v.to_string());
        }
        if let Some(v) = self.split_width_multiplier {
            sec.add_pair("split_width_multiplier", v.to_string());
        }
        if let Some(v) = self.special_scale_factor {
            sec.add_pair("special_scale_factor", v.to_string());
        }
        sec.render(ctx)
    }
}

/// Master tiling layout configuration.
///
/// Rendered as a `master { }` section block.
///
/// In the master layout one window is the "master" and all others are stacked
/// in a secondary column.  The split ratio, orientation, and new-window
/// placement are fully configurable.
///
/// # Example
/// ```
/// # use toconfig::hyprland::HyprlandConfig;
/// use toconfig::hyprland::layout::MasterLayout;
/// let m = MasterLayout::new().mfact(0.55).new_status("master");
/// let out = m.generate();
/// assert!(out.contains("master {"));
/// assert!(out.contains("mfact = 0.55"));
/// ```
#[derive(Default)]
pub struct MasterLayout {
    pub allow_small_split: Option<bool>,
    pub special_scale_factor: Option<f32>,
    /// Master area fraction of total width/height (0.0–1.0).
    pub mfact: Option<f32>,
    /// Where new windows go: `"master"`, `"slave"`, `"inherit"`.
    pub new_status: Option<String>,
    pub new_on_top: Option<bool>,
    /// `"master"` or `"slave"` or `"none"`.
    pub new_on_active: Option<String>,
    pub no_gaps_when_only: Option<u8>,
    /// Orientation of the stack: `"left"`, `"right"`, `"top"`, `"bottom"`, `"center"`.
    pub orientation: Option<String>,
    pub inherit_fullscreen: Option<bool>,
    pub always_center_master: Option<bool>,
    pub smart_resizing: Option<bool>,
    pub drop_at_cursor: Option<bool>,
}

impl MasterLayout {
    pub fn new() -> Self {
        Self::default()
    }

    /// Master area fraction (default 0.55).
    pub fn mfact(mut self, v: f32) -> Self {
        self.mfact = Some(v);
        self
    }

    /// Where new windows appear: `"master"`, `"slave"`, `"inherit"`.
    pub fn new_status(mut self, s: impl Into<String>) -> Self {
        self.new_status = Some(s.into());
        self
    }

    /// Stack orientation: `"left"`, `"right"`, `"top"`, `"bottom"`, `"center"`.
    pub fn orientation(mut self, s: impl Into<String>) -> Self {
        self.orientation = Some(s.into());
        self
    }

    pub fn new_on_top(mut self, v: bool) -> Self {
        self.new_on_top = Some(v);
        self
    }

    pub fn no_gaps_when_only(mut self, v: u8) -> Self {
        self.no_gaps_when_only = Some(v);
        self
    }

    pub fn always_center_master(mut self, v: bool) -> Self {
        self.always_center_master = Some(v);
        self
    }

    pub fn inherit_fullscreen(mut self, v: bool) -> Self {
        self.inherit_fullscreen = Some(v);
        self
    }
}

impl HyprlandConfig for MasterLayout {
    fn render(&self, ctx: &HyprlandRenderContext) -> String {
        let mut sec = Section::new("master");
        if let Some(v) = self.allow_small_split {
            sec.add_pair("allow_small_split", bool_str(v));
        }
        if let Some(v) = self.special_scale_factor {
            sec.add_pair("special_scale_factor", v.to_string());
        }
        if let Some(v) = self.mfact {
            sec.add_pair("mfact", v.to_string());
        }
        if let Some(ref v) = self.new_status {
            sec.add_pair("new_status", v.clone());
        }
        if let Some(v) = self.new_on_top {
            sec.add_pair("new_on_top", bool_str(v));
        }
        if let Some(ref v) = self.new_on_active {
            sec.add_pair("new_on_active", v.clone());
        }
        if let Some(v) = self.no_gaps_when_only {
            sec.add_pair("no_gaps_when_only", v.to_string());
        }
        if let Some(ref v) = self.orientation {
            sec.add_pair("orientation", v.clone());
        }
        if let Some(v) = self.inherit_fullscreen {
            sec.add_pair("inherit_fullscreen", bool_str(v));
        }
        if let Some(v) = self.always_center_master {
            sec.add_pair("always_center_master", bool_str(v));
        }
        if let Some(v) = self.smart_resizing {
            sec.add_pair("smart_resizing", bool_str(v));
        }
        if let Some(v) = self.drop_at_cursor {
            sec.add_pair("drop_at_cursor", bool_str(v));
        }
        sec.render(ctx)
    }
}

/// Scrolling layout configuration for the **hyprscroller** community plugin.
///
/// Rendered as a `plugin { scroller { } }` section block.
///
/// Install hyprscroller and load it with [`crate::hyprland::exec::PluginLoad`]
/// before activating via `general { layout = scroller }`.
///
/// # Example
/// ```
/// # use toconfig::hyprland::HyprlandConfig;
/// use toconfig::hyprland::layout::ScrollingLayout;
/// let s = ScrollingLayout::new().column_default_width("onehalf").focus_wrap(true);
/// let out = s.generate();
/// assert!(out.contains("plugin {"));
/// assert!(out.contains("scroller {"));
/// ```
#[derive(Default)]
pub struct ScrollingLayout {
    pub column_default_width: Option<String>,
    pub center_column_first: Option<bool>,
    pub focus_wrap: Option<bool>,
    pub column_widths: Option<String>,
    pub window_heights: Option<String>,
}

impl ScrollingLayout {
    pub fn new() -> Self {
        Self::default()
    }

    /// Default column width: `"onehalf"`, `"onethird"`, `"twothirds"`, `"maximized"`, `"floating"`.
    pub fn column_default_width(mut self, w: impl Into<String>) -> Self {
        self.column_default_width = Some(w.into());
        self
    }

    pub fn center_column_first(mut self, v: bool) -> Self {
        self.center_column_first = Some(v);
        self
    }

    /// Wrap focus when reaching the edge of the workspace.
    pub fn focus_wrap(mut self, v: bool) -> Self {
        self.focus_wrap = Some(v);
        self
    }

    /// Space-separated list of allowed column widths, e.g. `"onethird onehalf twothirds"`.
    pub fn column_widths(mut self, v: impl Into<String>) -> Self {
        self.column_widths = Some(v.into());
        self
    }

    /// Space-separated list of allowed window heights.
    pub fn window_heights(mut self, v: impl Into<String>) -> Self {
        self.window_heights = Some(v.into());
        self
    }
}

impl HyprlandConfig for ScrollingLayout {
    fn render(&self, ctx: &HyprlandRenderContext) -> String {
        let mut inner_sec = Section::new("scroller");
        if let Some(ref v) = self.column_default_width {
            inner_sec.add_pair("column_default_width", v.clone());
        }
        if let Some(v) = self.center_column_first {
            inner_sec.add_pair("center_column_first", bool_str(v));
        }
        if let Some(v) = self.focus_wrap {
            inner_sec.add_pair("focus_wrap", bool_str(v));
        }
        if let Some(ref v) = self.column_widths {
            inner_sec.add_pair("column_widths", v.clone());
        }
        if let Some(ref v) = self.window_heights {
            inner_sec.add_pair("window_heights", v.clone());
        }
        Section::new("plugin").nested(inner_sec).render(ctx)
    }
}

/// Monocle layout configuration for the **hyprmonocle** community plugin.
///
/// Rendered as a `plugin { monocle { } }` section block.
///
/// Monocle shows one maximised window at a time with optional centering.
/// Alternatively you can achieve a similar effect natively using the master
/// layout with `orientation = center` and `no_gaps_when_only = 1`.
///
/// # Example
/// ```
/// # use toconfig::hyprland::HyprlandConfig;
/// use toconfig::hyprland::layout::MonocleLayout;
/// let m = MonocleLayout::new().center(true);
/// let out = m.generate();
/// assert!(out.contains("plugin {"));
/// assert!(out.contains("monocle {"));
/// ```
#[derive(Default)]
pub struct MonocleLayout {
    pub center: Option<bool>,
    pub new_on_top: Option<bool>,
    pub special_scale_factor: Option<f32>,
}

impl MonocleLayout {
    pub fn new() -> Self {
        Self::default()
    }

    /// Centre the monocle window horizontally.
    pub fn center(mut self, v: bool) -> Self {
        self.center = Some(v);
        self
    }

    /// New windows appear at the top of the monocle stack.
    pub fn new_on_top(mut self, v: bool) -> Self {
        self.new_on_top = Some(v);
        self
    }

    pub fn special_scale_factor(mut self, v: f32) -> Self {
        self.special_scale_factor = Some(v);
        self
    }
}

impl HyprlandConfig for MonocleLayout {
    fn render(&self, ctx: &HyprlandRenderContext) -> String {
        let mut inner_sec = Section::new("monocle");
        if let Some(v) = self.center {
            inner_sec.add_pair("center", bool_str(v));
        }
        if let Some(v) = self.new_on_top {
            inner_sec.add_pair("new_on_top", bool_str(v));
        }
        if let Some(v) = self.special_scale_factor {
            inner_sec.add_pair("special_scale_factor", v.to_string());
        }
        Section::new("plugin").nested(inner_sec).render(ctx)
    }
}
