use super::{HyprlandConfig, HyprlandRenderContext};

/// A custom cubic bezier curve used by animation rules.
///
/// Rendered as: `bezier = name, p1x, p1y, p2x, p2y`
///
/// The four values describe the two control points of a CSS-style cubic bezier.
/// Use <https://cubic-bezier.com/> to preview curves visually.
///
/// # Example
/// ```
/// # use toconfig::hyprland::HyprlandConfig;
/// use toconfig::hyprland::animation::Bezier;
/// let b = Bezier::new("myBezier", 0.05, 0.9, 0.1, 1.05);
/// assert_eq!(b.generate(), "bezier = myBezier, 0.05, 0.9, 0.1, 1.05");
/// ```
pub struct Bezier {
    pub name: String,
    pub p1x: f32,
    pub p1y: f32,
    pub p2x: f32,
    pub p2y: f32,
}

impl Bezier {
    pub fn new(name: impl Into<String>, p1x: f32, p1y: f32, p2x: f32, p2y: f32) -> Self {
        Self {
            name: name.into(),
            p1x,
            p1y,
            p2x,
            p2y,
        }
    }

    /// Ease-in-out (smooth, symmetrical).
    pub fn ease_in_out(name: impl Into<String>) -> Self {
        Self::new(name, 0.42, 0.0, 0.58, 1.0)
    }

    /// Ease-out with a slight overshoot — popular "snappy" feel.
    pub fn ease_out_back(name: impl Into<String>) -> Self {
        Self::new(name, 0.05, 0.9, 0.1, 1.05)
    }

    /// Linear (constant speed).
    pub fn linear(name: impl Into<String>) -> Self {
        Self::new(name, 0.0, 0.0, 1.0, 1.0)
    }

    /// Ease-out (fast start, slow finish).
    pub fn ease_out(name: impl Into<String>) -> Self {
        Self::new(name, 0.0, 0.0, 0.58, 1.0)
    }
}

impl HyprlandConfig for Bezier {
    fn render(&self, ctx: &HyprlandRenderContext) -> String {
        format!(
            "{}bezier = {}, {}, {}, {}, {}",
            ctx.indent(),
            self.name,
            self.p1x,
            self.p1y,
            self.p2x,
            self.p2y
        )
    }
}

/// A single animation rule.
///
/// Rendered as: `animation = name, enabled, speed, curve[, style]`
///
/// # Standard animation names
/// - `windows` — window open/close
/// - `windowsIn` / `windowsOut` / `windowsMove` — more granular window events
/// - `fade` — fade in/out
/// - `fadeIn` / `fadeOut` / `fadeSwitch` / `fadeShadow` / `fadeDim`
/// - `border` — border colour transitions
/// - `borderangle` — animated gradient border rotation
/// - `workspaces` — workspace switch
/// - `specialWorkspace` — special workspace toggle
///
/// # Common styles
/// - `slide` / `slidevert` — slide in from an edge
/// - `popin` — scale from centre
/// - `fade` — cross-fade
///
/// # Example
/// ```
/// # use toconfig::hyprland::HyprlandConfig;
/// use toconfig::hyprland::animation::Animation;
/// let a = Animation::new("windows", 7.0, "myBezier").style("slide");
/// assert_eq!(a.generate(), "animation = windows, 1, 7, myBezier, slide");
/// ```
pub struct Animation {
    pub name: String,
    pub enabled: bool,
    pub speed: f32,
    pub curve: String,
    pub style: Option<String>,
}

impl Animation {
    pub fn new(name: impl Into<String>, speed: f32, curve: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            enabled: true,
            speed,
            curve: curve.into(),
            style: None,
        }
    }

    /// Disable this animation (sets enabled = 0).
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }

    /// Set the animation style, e.g. `"slide"`, `"popin"`.
    pub fn style(mut self, s: impl Into<String>) -> Self {
        self.style = Some(s.into());
        self
    }
}

impl HyprlandConfig for Animation {
    fn render(&self, ctx: &HyprlandRenderContext) -> String {
        let enabled = if self.enabled { 1 } else { 0 };
        let base = format!(
            "{}animation = {}, {}, {}, {}",
            ctx.indent(),
            self.name,
            enabled,
            self.speed,
            self.curve
        );
        match &self.style {
            Some(s) => format!("{}, {}", base, s),
            None => base,
        }
    }
}

/// The `animations { }` section, containing bezier curves and animation rules.
///
/// # Example
/// ```
/// # use toconfig::hyprland::HyprlandConfig;
/// use toconfig::hyprland::animation::{AnimationsSection, Bezier, Animation};
/// let sec = AnimationsSection::new()
///     .bezier(Bezier::ease_out_back("myBezier"))
///     .animation(Animation::new("windows", 7.0, "myBezier").style("slide"))
///     .animation(Animation::new("workspaces", 6.0, "default"));
/// let out = sec.generate();
/// assert!(out.contains("animations {"));
/// assert!(out.contains("bezier = myBezier"));
/// ```
pub struct AnimationsSection {
    pub enabled: bool,
    pub beziers: Vec<Bezier>,
    pub animations: Vec<Animation>,
}

impl Default for AnimationsSection {
    fn default() -> Self {
        Self::new()
    }
}

impl AnimationsSection {
    pub fn new() -> Self {
        Self {
            enabled: true,
            beziers: Vec::new(),
            animations: Vec::new(),
        }
    }

    /// Create a section with all animations disabled.
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            beziers: Vec::new(),
            animations: Vec::new(),
        }
    }

    /// Add a bezier curve definition.
    pub fn bezier(mut self, b: Bezier) -> Self {
        self.beziers.push(b);
        self
    }

    /// Add an animation rule.
    pub fn animation(mut self, a: Animation) -> Self {
        self.animations.push(a);
        self
    }
}

impl HyprlandConfig for AnimationsSection {
    fn render(&self, ctx: &HyprlandRenderContext) -> String {
        let indent = ctx.indent();
        let inner = ctx.deeper();
        let ii = inner.indent();

        let mut lines = vec![format!("{}animations {{", indent)];
        lines.push(format!(
            "{}enabled = {}",
            ii,
            if self.enabled { 1 } else { 0 }
        ));
        for bezier in &self.beziers {
            lines.push(bezier.render(&inner));
        }
        for anim in &self.animations {
            lines.push(anim.render(&inner));
        }
        lines.push(format!("{}}}", indent));
        lines.join("\n")
    }
}
