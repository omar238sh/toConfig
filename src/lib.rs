//! `toconfig` — type-safe configuration generator for Neovim, Hyprland, Fish, GTK, and Qt.
//!
//! # Module Overview
//!
//! | Module     | Purpose                                                         |
//! |------------|-----------------------------------------------------------------|
//! | [`core`]   | Shared `Config` trait, `ConfigTree`, `RenderContext`            |
//! | [`ini`]    | `IniConfig` trait + `IniSection` / `IniFile` (GTK & Qt format) |
//! | [`lua`]    | `LuaValue` serialization + `RawLua` escape hatch                |
//! | [`output`] | `ConfigOutput` / `IniOutput` — diff-aware file writing          |
//! | [`neovim`] | All Neovim configuration builders                               |
//! | [`hyprland`] | Hyprland window manager builders                              |
//! | [`fish`]   | Fish shell configuration builders                               |
//! | [`gtk`]    | GTK 3 / GTK 4 `settings.ini` + `gtk.css` builders              |
//! | [`qt`]     | Qt 5 / Qt 6 qt5ct / qt6ct configuration builders               |
//! | [`theme`]  | Unified [`Theme`] with palette, fonts, cursor, and icon config  |

pub mod core;
pub mod fish;
pub mod gtk;
pub mod hyprland;
pub mod ini;
pub mod lua;
pub mod neovim;
pub mod output;
pub mod qt;
pub mod theme;

// ── Top-level convenience re-exports ─────────────────────────────────────────

pub use core::{Config, ConfigTree, RenderContext};
pub use ini::{IniConfig, IniFile, IniSection};
pub use lua::{LuaValue, RawLua};
pub use theme::Theme;
