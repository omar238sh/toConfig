//! Neovim configuration modules.
//!
//! This namespace groups all Neovim-specific builders under a single parent,
//! keeping them clearly separated from Hyprland and Fish shell modules.

pub mod autocmd;
pub mod command;
pub mod keymap;
pub mod options;
pub mod plugins;
pub mod profile;
pub mod theme;

// ── Convenient re-exports ─────────────────────────────────────────────────────

pub use autocmd::{Augroup, AutocmdAction, AutocmdNode, AutocmdPattern};
pub use command::{CmdCompletion, UserCommand};
pub use keymap::{KeymapGroup, KeymapNode, MapOpts, MapRhs, Mode};
pub use options::{OptionNode, OptionScope, OptionsBlock, default_editor_options};
pub use plugins::{
    CmpConfig, CmpSource, LazyManager, LspConfig, LspConfigNode, Plugin, ServerConfig,
    TelescopeConfigNode, TelescopeDefaults, TreesitterConfig, TreesitterHighlight,
};
pub use profile::Profile;
pub use theme::{ColorschemeNode, HexColor, HighlightAttrs, HighlightNode, ThemeNode};
