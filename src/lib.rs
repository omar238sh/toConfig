//! `toconfig` — type-safe configuration generator for Neovim, Hyprland, Fish, and more.
//!
//! # Module Overview
//!
//! | Module         | Purpose                                              |
//! |----------------|------------------------------------------------------|
//! | [`core`]       | Shared `Config` trait, `ConfigTree`, `RenderContext`|
//! | [`lua`]        | `LuaValue` serialization + `RawLua` escape hatch     |
//! | [`output`]     | `ConfigOutput` — file writing with diff-check        |
//! | [`neovim`]     | All Neovim configuration builders                    |
//! | [`hyprland`]   | Hyprland window manager builders                     |
//! | [`fish`]       | Fish shell configuration builders                    |
//! | [`fontconfig`] | Fontconfig XML configuration builders                |

pub mod core;
pub mod fish;
pub mod fontconfig;
pub mod hyprland;
pub mod lua;
pub mod neovim;
pub mod output;

// ── Top-level convenience re-exports ─────────────────────────────────────────

pub use core::{Config, ConfigTree, RenderContext};
pub use lua::{LuaValue, RawLua};
