//! `toconfig` — type-safe configuration generator for Neovim, Hyprland, Fish,
//! Helix, Waybar, and systemd.
//!
//! # Module Overview
//!
//! | Module       | Purpose                                                          |
//! |--------------|------------------------------------------------------------------|
//! | [`core`]     | Shared `Config` trait, `ConfigTree`, `RenderContext`            |
//! | [`lua`]      | `LuaValue` serialization + `RawLua` escape hatch                 |
//! | [`output`]   | `ConfigOutput` — file writing with diff-check                    |
//! | [`neovim`]   | All Neovim configuration builders                                |
//! | [`hyprland`] | Hyprland window manager builders                                 |
//! | [`fish`]     | Fish shell configuration builders                                |
//! | [`helix`]    | Helix editor `config.toml` builders                              |
//! | [`waybar`]   | Waybar status bar JSON builders                                  |
//! | [`systemd`]  | Systemd unit-file builders (service, socket, timer, mount, …)   |
//! | [`fstab`]    | `/etc/fstab` builder with typed mount-option helpers             |

pub mod core;
pub mod fish;
pub mod fstab;
pub mod helix;
pub mod hyprland;
pub mod lua;
pub mod neovim;
pub mod output;
pub mod systemd;
pub mod waybar;

// ── Top-level convenience re-exports ─────────────────────────────────────────

pub use core::{Config, ConfigTree, RenderContext};
pub use lua::{LuaValue, RawLua};
