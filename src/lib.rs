

pub mod core;
pub mod fish;
pub mod fontconfig;
pub mod ghostty;
pub mod gtk;
pub mod fstab;
pub mod helix;
pub mod hyprland;
pub mod kitty;
pub mod ini;
pub mod lua;
pub mod neovim;
pub mod output;
pub mod qt;
pub mod theme;
pub mod systemd;
pub mod waybar;



// ── Top-level convenience re-exports ─────────────────────────────────────────

pub use core::{Config, ConfigTree, RenderContext};
pub use ini::{IniConfig, IniFile, IniSection};
pub use lua::{LuaValue, RawLua};
pub use theme::Theme;
