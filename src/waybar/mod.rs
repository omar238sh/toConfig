//! Waybar status bar configuration builder.
//!
//! Generates `~/.config/waybar/config` (a JSON file) from type-safe Rust
//! structs.  Each module implements [`WaybarModule`] and is placed into a
//! [`Bar`]; the bar is then collected in a [`WaybarConfigTree`].
//!
//! # Quick-start example
//! ```
//! use toconfig::waybar::{Bar, WaybarConfig, WaybarConfigTree};
//! use toconfig::waybar::clock::Clock;
//! use toconfig::waybar::battery::Battery;
//! use toconfig::waybar::network::Network;
//!
//! let mut tree = WaybarConfigTree::new();
//! tree.add(
//!     Bar::new()
//!         .position("top")
//!         .height(30)
//!         .add_left(Network::new().format("{ifname}: {ipaddr}"))
//!         .add_center(Clock::new().format("{:%H:%M}"))
//!         .add_right(Battery::new().format("{capacity}% {icon}")),
//! );
//!
//! println!("{}", tree.generate());
//! ```

pub mod backlight;
pub mod bar;
pub mod battery;
pub mod clock;
pub mod core;
pub mod cpu;
pub mod memory;
pub mod network;
pub mod pulseaudio;

// ── Core re-exports ───────────────────────────────────────────────────────────
pub use core::{WaybarConfig, WaybarConfigTree, WaybarModule, WaybarRenderContext};

// ── Sub-module re-exports ─────────────────────────────────────────────────────
pub use backlight::Backlight;
pub use bar::Bar;
pub use battery::Battery;
pub use clock::Clock;
pub use cpu::Cpu;
pub use memory::Memory;
pub use network::Network;
pub use pulseaudio::Pulseaudio;
