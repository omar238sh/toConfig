//! `toconfig` — type-safe configuration generator for Neovim, Hyprland, Fish, and Ghostty.
//!
//! # Module Overview
//!
//! | Module       | Purpose                                              |
//! |--------------|------------------------------------------------------|
//! | [`core`]     | Shared `Config` trait, `ConfigTree`, `RenderContext`|
//! | [`lua`]      | `LuaValue` serialization + `RawLua` escape hatch     |
//! | [`output`]   | `ConfigOutput` — file writing with diff-check        |
//! | [`neovim`]   | All Neovim configuration builders                    |
//! | [`hyprland`] | Hyprland window manager builders                     |
//! | [`fish`]     | Fish shell configuration builders                    |
//! | [`ghostty`]  | Ghostty terminal emulator configuration builders     |

pub mod core;
pub mod fish;
pub mod ghostty;
pub mod hyprland;
pub mod lua;
pub mod neovim;
pub mod output;

// ── Top-level convenience re-exports ─────────────────────────────────────────

pub use core::{Config, ConfigTree, RenderContext};
pub use lua::{LuaValue, RawLua};
