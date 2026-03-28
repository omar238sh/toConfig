//! Neovim plugin builders (lazy.nvim, nvim-cmp, LSP, Telescope, Treesitter).

pub mod cmp;
pub mod lazy;
pub mod lsp;
pub mod telescope;
pub mod treesitter;

pub use cmp::{CmpConfig, CmpSource};
pub use lazy::{LazyManager, Plugin};
pub use lsp::{LspConfig, LspConfigNode, ServerConfig};
pub use telescope::{TelescopeConfigNode, TelescopeDefaults};
pub use treesitter::{TreesitterConfig, TreesitterHighlight};
