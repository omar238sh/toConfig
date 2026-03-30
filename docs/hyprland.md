# Hyprland

The `toconfig::hyprland` module generates Hyprland window manager configuration files from typed Rust structs. All nodes implement the `HyprlandConfig` trait and are collected in a `HyprlandConfigTree`. The trait is **separate** from `Config` so the compiler prevents accidentally mixing Neovim or Fish nodes into a Hyprland tree.

```rust
pub trait HyprlandConfig {
    fn render(&self, ctx: &HyprlandRenderContext) -> String;
    fn generate(&self) -> String { ... }   // render with default context
    fn validate(&self) -> Result<(), String> { Ok(()) }
}
```

`HyprlandRenderContext` defaults to 0-level, 4-space indentation.

---

## Table of contents

1. [Variables](#variables)
2. [Environment variables](#environment-variables)
3. [Monitor configuration](#monitor-configuration)
4. [Exec / startup commands](#exec--startup-commands)
5. [Keybinds](#keybinds)
6. [Window rules](#window-rules)
7. [Workspace rules](#workspace-rules)
8. [Animations](#animations)
9. [Layouts](#layouts)
10. [Sections (generic blocks)](#sections-generic-blocks)
11. [XWayland](#xwayland)
12. [Permissions](#permissions)
13. [Output](#output)
14. [End-to-end example](#end-to-end-example)

---

## Variables

```rust
use toconfig::hyprland::{HyprlandConfig, Variable};

let v = Variable::new("terminal", "kitty");
println!("{}", v.generate());
// → $terminal = kitty
```

---

## Environment variables

```rust
use toconfig::hyprland::{HyprlandConfig, EnvVar};

// Arbitrary variable
let e = EnvVar::new("MY_VAR", "hello");
// → env = MY_VAR,hello

// Convenience constructors for common Wayland / NVIDIA setups:
EnvVar::xcursor_size(24)               // env = XCURSOR_SIZE,24
EnvVar::xcursor_theme("Adwaita")       // env = XCURSOR_THEME,Adwaita
EnvVar::qt_wayland()                   // env = QT_QPA_PLATFORM,wayland
EnvVar::qt_no_csd()
EnvVar::xdg_current_desktop("Hyprland")
EnvVar::xdg_session_wayland()
EnvVar::gdk_wayland()
EnvVar::sdl_wayland()
EnvVar::preferred_gpu("/dev/dri/card1:/dev/dri/card0")  // multi-GPU
EnvVar::nvidia_libva()
EnvVar::nvidia_gbm()
EnvVar::nvidia_glx()
EnvVar::nvidia_explicit_sync()
```

---

## Monitor configuration

```rust
use toconfig::hyprland::{HyprlandConfig, MonitorConfig};

// Named monitor at 1080p 60 Hz, 1× scaling
let m = MonitorConfig::new("eDP-1", "1920x1080@60", "0x0", 1.0);
// → monitor=eDP-1,1920x1080@60,0x0,1

// Catch-all fallback
let fallback = MonitorConfig::auto();
// → monitor=,preferred,auto,1

// HiDPI with 90° rotation and VRR
let hidpi = MonitorConfig::new("DP-1", "3840x2160@60", "1920x0", 2.0)
    .transform(1)
    .vrr(1);
```

### Optional setters

`transform` (0–7), `mirror`, `bitdepth`, `vrr` (0–2).

---

## Exec / startup commands

```rust
use toconfig::hyprland::exec::{ExecOnce, Exec, PluginLoad, Source};

ExecOnce::new("waybar")                        // exec-once = waybar
Exec::new("swww img ~/wallpaper.png")          // exec = swww img ~/wallpaper.png
PluginLoad::new("/usr/lib/hyprland/plugin.so") // plugin = /usr/lib/hyprland/plugin.so
Source::new("~/.config/hypr/keybinds.conf")   // source = ~/.config/hypr/keybinds.conf
ExecOnce::hyprctl_dispatch("workspace 1")      // exec-once = hyprctl dispatch workspace 1
```

---

## Keybinds

```rust
use toconfig::hyprland::{HyprlandConfig, Bind, Dispatcher};

// Launch terminal
let b = Bind::new("SUPER", "Return", Dispatcher::Exec("$terminal".into()));
// → bind = SUPER, Return, exec, $terminal

// Kill window
let k = Bind::new("SUPER", "Q", Dispatcher::KillActive);
// → bind = SUPER, Q, killactive

// Switch workspace
let ws = Bind::new("SUPER", "1", Dispatcher::Workspace("1".into()));

// Mouse bind (bindm)
let m = Bind::new("SUPER", "mouse:272", Dispatcher::Custom("movewindow".into(), None))
    .mouse();
// → bindm = SUPER, mouse:272, movewindow
```

### `Bind` flag modifiers

| Method | Flag |
|--------|------|
| `.locked()` | `bindl` |
| `.release()` | `bindr` |
| `.repeat()` | `binde` |
| `.non_consuming()` | `bindn` |
| `.mouse()` | `bindm` |

### Common `Dispatcher` variants

`Exec`, `KillActive`, `ForceCloseActive`, `ToggleFloating`, `FullScreen(u8)`, `TogglePseudo`, `ToggleSplit`, `MoveFocus`, `SwapWindow`, `MoveWindow`, `ResizeActive`, `Workspace`, `MoveToWorkspace`, `MoveToWorkspaceSilent`, `ToggleSpecialWorkspace`, `CycleNext`, `CyclePrev`, `Pin`, `SplitRatio`, `Custom`.

---

## Window rules

```rust
use toconfig::hyprland::{HyprlandConfig, WindowRule};

// Float pavucontrol
let r = WindowRule::new("float", "class:^(pavucontrol)$");
// → windowrulev2 = float, class:^(pavucontrol)$

// Multiple matchers (AND logic)
let r2 = WindowRule::new("float", "class:^(kitty)$")
    .and("title:^(float)$");

// Allow tearing for games
let tearing = WindowRule::new("immediate", "class:^(game_binary)$");
```

---

## Workspace rules

```rust
use toconfig::hyprland::workspace::WorkspaceRule;

let w = WorkspaceRule::new("1")
    .monitor("eDP-1")
    .default();
// → workspace = 1, monitor:eDP-1, default:true

// Special workspace (scratchpad) with auto-launch
WorkspaceRule::new("special:magic")
    .on_created_empty("kitty");
```

Other setters: `persistent()`, `gaps_in(px)`, `gaps_out(px)`, `rule(key, value)`.

---

## Animations

```rust
use toconfig::hyprland::{HyprlandConfig, AnimationsSection, Bezier, Animation};

let sec = AnimationsSection::new()
    .bezier(Bezier::ease_out_back("myBezier"))
    .bezier(Bezier::ease_in_out("smoothBezier"))
    .animation(Animation::new("windows", 7.0, "myBezier").style("slide"))
    .animation(Animation::new("workspaces", 6.0, "smoothBezier"));
```

### Bezier presets

`ease_in_out`, `ease_out_back`, `ease_out`, `linear`. Custom: `Bezier::new(name, p1x, p1y, p2x, p2y)`.

`Animation::new(name, speed, curve)` — optional `.style("slide"|"popin"|"fade")` and `.disabled()`.

---

## Layouts

```rust
use toconfig::hyprland::layout::{DwindleLayout, MasterLayout, ScrollingLayout, MonocleLayout};

// Dwindle (default)
DwindleLayout::new()
    .pseudotile(true)
    .preserve_split(true)
    .force_split(2);

// Master
MasterLayout::new()
    .mfact(0.55)
    .new_status("master")
    .orientation("left");

// Scrolling (hyprscroller plugin required)
ScrollingLayout::new()
    .column_default_width("onehalf")
    .focus_wrap(true);

// Monocle (hyprmonocle plugin required)
MonocleLayout::new().center(true);
```

---

## Sections (generic blocks)

`Section` covers every named block in Hyprland: `general`, `input`, `decoration`, `misc`, `binds`, `cursor`, `render`, `opengl`, `debug`, `group`, `group:groupbar`, and any plugin-defined section.

```rust
use toconfig::hyprland::section::{Section, RawHyprland};

let general = Section::new("general")
    .pair("gaps_in", "5")
    .pair("gaps_out", "20")
    .pair("border_size", "2")
    .pair("allow_tearing", "false");

// Nested sub-section
let input = Section::new("input")
    .pair("kb_layout", "us")
    .nested(
        Section::new("touchpad")
            .pair("natural_scroll", "true")
    );

// Raw escape hatch
let raw = RawHyprland::new("misc {\n    disable_hyprland_logo = true\n}");
```

---

## XWayland

```rust
use toconfig::hyprland::xwayland::XWayland;

XWayland::new()
    .enabled(true)
    .use_nearest_neighbor(false)
    .force_zero_scaling(true);
// → xwayland { enabled = true, use_nearest_neighbor = false, force_zero_scaling = true }
```

---

## Permissions

```rust
use toconfig::hyprland::permission::Permission;

Permission::new("screensharing", "allow");
// → permission = screensharing, allow
```

---

## Output

```rust
use toconfig::hyprland::output::HyprlandOutput;

// Write to ~/.config/hypr/hyprland.conf
HyprlandOutput::hyprland_conf().write(&hypr_tree)?;

// Custom path
HyprlandOutput::at_path("/tmp/test.conf").write(&hypr_tree)?;

// Preview
let text = HyprlandOutput::hyprland_conf().preview(&hypr_tree);
```

---

## End-to-end example

```rust
use toconfig::hyprland::{
    HyprlandConfig, HyprlandConfigTree,
    Variable, EnvVar, MonitorConfig,
    Bind, Dispatcher,
    WindowRule,
    AnimationsSection, Bezier, Animation,
};
use toconfig::hyprland::exec::ExecOnce;
use toconfig::hyprland::section::Section;
use toconfig::hyprland::workspace::WorkspaceRule;
use toconfig::hyprland::output::HyprlandOutput;

fn main() -> std::io::Result<()> {
    let mut tree = HyprlandConfigTree::new();

    // Variables
    tree.add(Variable::new("terminal", "kitty"));
    tree.add(Variable::new("browser",  "firefox"));

    // Environment
    tree.add(EnvVar::xcursor_size(24));
    tree.add(EnvVar::xcursor_theme("Bibata-Modern-Ice"));
    tree.add(EnvVar::qt_wayland());
    tree.add(EnvVar::xdg_session_wayland());

    // Monitor
    tree.add(MonitorConfig::new("eDP-1", "1920x1080@60", "0x0", 1.0));
    tree.add(MonitorConfig::auto());

    // Startup
    tree.add(ExecOnce::new("waybar"));
    tree.add(ExecOnce::new("hyprpaper"));

    // General settings
    tree.add(
        Section::new("general")
            .pair("gaps_in", "5")
            .pair("gaps_out", "15")
            .pair("border_size", "2")
    );
    tree.add(
        Section::new("input")
            .pair("kb_layout", "us")
            .nested(Section::new("touchpad").pair("natural_scroll", "true"))
    );

    // Keybinds
    tree.add(Bind::new("SUPER", "Return", Dispatcher::Exec("$terminal".into())));
    tree.add(Bind::new("SUPER", "Q",      Dispatcher::KillActive));
    tree.add(Bind::new("SUPER", "F",      Dispatcher::ToggleFloating));
    for i in 1..=9 {
        tree.add(Bind::new("SUPER", &i.to_string(), Dispatcher::Workspace(i.to_string())));
    }

    // Window rules
    tree.add(WindowRule::new("float", "class:^(pavucontrol)$"));
    tree.add(WindowRule::new("float", "class:^(nm-connection-editor)$"));

    // Animations
    tree.add(
        AnimationsSection::new()
            .bezier(Bezier::ease_out_back("myBezier"))
            .animation(Animation::new("windows", 7.0, "myBezier").style("slide"))
            .animation(Animation::new("workspaces", 6.0, "default"))
    );

    HyprlandOutput::hyprland_conf().write(&tree)?;
    Ok(())
}
```
