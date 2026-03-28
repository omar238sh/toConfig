//! Fish shell configuration builder.
//!
//! Generates `~/.config/fish/config.fish` (and related files) from type-safe
//! Rust structs. Every construct in this module implements [`crate::core::Config`]
//! so it can be composed inside a [`FishConfigTree`].

mod util;

pub mod abbr;
pub mod alias;
pub mod bind;
pub mod color;
pub mod completion;
pub mod conditional;
pub mod config_tree;
pub mod function;
pub mod path;
pub mod prompt;
pub mod source;
pub mod variable;

// ── Re-exports ────────────────────────────────────────────────────────────────

pub use abbr::{AbbrPosition, FishAbbr};
pub use alias::FishAlias;
pub use bind::{BindMode, FishBind};
pub use color::{FishColor, FishColorVar};
pub use completion::FishCompletion;
pub use conditional::{
    FishBegin, FishCase, FishCondition, FishElseIf, FishFor, FishIf, FishSwitch, FishWhile,
};
pub use config_tree::FishConfigTree;
pub use function::{FishEvent, FishFunction};
pub use path::FishAddPath;
pub use prompt::{FishGreeting, FishModePrompt, FishPrompt, FishRightPrompt};
pub use source::{FishPlugin, FishRawLine, FishSource};
pub use variable::{FishVariable, VarScope};
