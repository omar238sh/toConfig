use super::{HyprlandConfig, HyprlandRenderContext};

/// A Hyprland window rule (v2 syntax).
///
/// Rendered as: `windowrulev2 = rule, matcher[, matcher...]`
///
/// Multiple matchers are combined with AND logic; the rule applies only when
/// **all** matchers match.
///
/// # Common rules
/// - `float` — make the window floating
/// - `tile` — force the window to be tiled
/// - `workspace <id>` — open the window on a specific workspace
/// - `monitor <name>` — open on a specific monitor
/// - `size <w> <h>` — set initial size
/// - `move <x> <y>` — set initial position
/// - `pin` — pin (float across all workspaces)
/// - `nofocus` — prevent the window from receiving focus
/// - `noblur` — disable blur for this window
/// - `nodim` — do not dim this window
/// - `fullscreen` — open in fullscreen
/// - `immediate` — enable tearing / direct scanout (for games)
/// - `suppressevent maximize` — suppress maximise events
///
/// # Common matchers
/// - `class:^(regex)$` — match by WM_CLASS
/// - `title:^(regex)$` — match by window title
/// - `tag:<name>` — match by tag (set with `addwindowruleV2 tag`)
/// - `xwayland:1` — match XWayland windows
/// - `floating:1` / `tiled:1` — match by tiling state
/// - `fullscreen:1` — match fullscreen windows
/// - `onworkspace:<id>` — match windows on a specific workspace
///
/// # Tearing
/// To allow per-game tearing / direct scanout, add:
/// ```
/// use toconfig::hyprland::window_rule::WindowRule;
/// let r = WindowRule::new("immediate", "class:^(game_binary)$");
/// ```
/// This must be combined with `allow_tearing = true` in the `general` section.
///
/// # Example
/// ```
/// # use toconfig::hyprland::HyprlandConfig;
/// use toconfig::hyprland::window_rule::WindowRule;
///
/// // Float pavucontrol
/// let r = WindowRule::new("float", "class:^(pavucontrol)$");
/// assert_eq!(r.generate(), "windowrulev2 = float, class:^(pavucontrol)$");
///
/// // Float AND match title
/// let r2 = WindowRule::new("float", "class:^(kitty)$")
///     .and("title:^(float)$");
/// assert!(r2.generate().contains("title:^(float)$"));
/// ```
pub struct WindowRule {
    pub rule: String,
    pub matchers: Vec<String>,
}

impl WindowRule {
    /// Create a window rule with a single matcher.
    pub fn new(rule: impl Into<String>, matcher: impl Into<String>) -> Self {
        Self {
            rule: rule.into(),
            matchers: vec![matcher.into()],
        }
    }

    /// Add an additional matcher (all matchers must match — AND logic).
    pub fn and(mut self, matcher: impl Into<String>) -> Self {
        self.matchers.push(matcher.into());
        self
    }
}

impl HyprlandConfig for WindowRule {
    fn render(&self, ctx: &HyprlandRenderContext) -> String {
        let matchers = self.matchers.join(", ");
        format!(
            "{}windowrulev2 = {}, {}",
            ctx.indent(),
            self.rule,
            matchers
        )
    }

    fn validate(&self) -> Result<(), String> {
        if self.rule.is_empty() {
            return Err("WindowRule rule cannot be empty".into());
        }
        if self.matchers.is_empty() {
            return Err("WindowRule must have at least one matcher".into());
        }
        Ok(())
    }
}
