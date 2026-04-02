# Waybar

The `toconfig::waybar` module generates Waybar status bar configuration (`~/.config/waybar/config`, a JSON file) from typed Rust structs. Each module implements `WaybarModule`, modules are placed in a `Bar`, and bars are collected in a `WaybarConfigTree`.

---

## Table of contents

1. [WaybarConfigTree and Bar](#waybarconfigtree-and-bar)
2. [Clock](#clock)
3. [Battery](#battery)
4. [Network](#network)
5. [CPU](#cpu)
6. [Memory](#memory)
7. [Pulseaudio](#pulseaudio)
8. [Backlight](#backlight)
9. [End-to-end example](#end-to-end-example)

---

## WaybarConfigTree and Bar

```rust
use toconfig::waybar::{Bar, WaybarConfig, WaybarConfigTree};

let mut tree = WaybarConfigTree::new();
tree.add(
    Bar::new()
        .position("top")        // "top" | "bottom" | "left" | "right"
        .height(30)
        .layer("top")           // "top" | "bottom" | "overlay" | "background"
        .output("eDP-1")        // restrict to a specific monitor
        .spacing(4)
        .add_left(/* module */)
        .add_center(/* module */)
        .add_right(/* module */),
);

println!("{}", tree.generate());
```

---

## Clock

```rust
use toconfig::waybar::Clock;

Clock::new()
    .format("{:%H:%M}")               // strftime format
    .format_alt("{:%A, %B %d, %Y}")   // alt format (shown on click)
    .tooltip_format("{calendar}")
    .timezone("Europe/London")
    .locale("en_US.UTF-8")
// → { "clock": { "format": "{:%H:%M}", ... } }
```

---

## Battery

```rust
use toconfig::waybar::Battery;

Battery::new()
    .format("{capacity}% {icon}")
    .format_charging("{capacity}% ")
    .format_plugged("{capacity}% ")
    .states_warning(30)      // percentage threshold for "warning" CSS class
    .states_critical(15)     // percentage threshold for "critical" CSS class
    .interval(60)
```

---

## Network

```rust
use toconfig::waybar::Network;

Network::new()
    .format("{ifname}: {ipaddr}")
    .format_wifi("{essid} ({signalStrength}%) ")
    .format_ethernet("{ipaddr}/{cidr} ")
    .format_disconnected("Disconnected ⚠")
    .tooltip_format_wifi("{essid} {signalStrength}%")
    .on_click("nm-connection-editor")
    .interval(30)
```

---

## CPU

```rust
use toconfig::waybar::Cpu;

Cpu::new()
    .format("{usage}% ")
    .tooltip(true)
    .interval(5)
```

---

## Memory

```rust
use toconfig::waybar::Memory;

Memory::new()
    .format("{used:0.1f}G/{total:0.1f}G ")
    .tooltip_format("{used:0.1f}GiB used")
    .interval(30)
```

---

## Pulseaudio

```rust
use toconfig::waybar::Pulseaudio;

Pulseaudio::new()
    .format("{volume}% {icon} {format_source}")
    .format_bluetooth("{volume}% {icon}  {format_source}")
    .format_muted(" {format_source}")
    .format_source("{volume}% ")
    .format_source_muted("")
    .on_click("pavucontrol")
```

---

## Backlight

```rust
use toconfig::waybar::Backlight;

Backlight::new()
    .device("intel_backlight")
    .format("{percent}% {icon}")
    .format_icons(&["", "", ""])
    .on_scroll_up("brightnessctl set 5%+")
    .on_scroll_down("brightnessctl set 5%-")
```

---

## End-to-end example

```rust
use toconfig::waybar::{Bar, WaybarConfig, WaybarConfigTree};
use toconfig::waybar::clock::Clock;
use toconfig::waybar::battery::Battery;
use toconfig::waybar::network::Network;
use toconfig::waybar::cpu::Cpu;
use toconfig::waybar::memory::Memory;
use toconfig::waybar::pulseaudio::Pulseaudio;
use toconfig::waybar::backlight::Backlight;
use toconfig::output::ConfigOutput;

fn main() -> std::io::Result<()> {
    let mut tree = WaybarConfigTree::new();
    tree.add(
        Bar::new()
            .position("top")
            .height(32)
            .spacing(4)
            .add_left(Network::new()
                .format_wifi("{essid} ({signalStrength}%) ")
                .format_ethernet("{ipaddr}/{cidr} ")
                .format_disconnected("⚠ Disconnected"))
            .add_left(Cpu::new().format("{usage}% ").interval(5))
            .add_left(Memory::new().format("{used:0.1f}G ").interval(30))
            .add_center(Clock::new()
                .format("{:%H:%M}")
                .format_alt("{:%A, %B %d, %Y}"))
            .add_right(Backlight::new().format("{percent}% {icon}"))
            .add_right(Battery::new()
                .format("{capacity}% {icon}")
                .states_warning(30)
                .states_critical(15))
            .add_right(Pulseaudio::new()
                .format("{volume}% {icon}")
                .format_muted(" muted")
                .on_click("pavucontrol")),
    );

    let path = format!("{}/.config/waybar/config", std::env::var("HOME").unwrap());
    ConfigOutput::at_path(&path).write(&tree)?;
    Ok(())
}
```
