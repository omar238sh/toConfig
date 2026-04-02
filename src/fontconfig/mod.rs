//! Fontconfig XML configuration builder.
//!
//! Generates `~/.config/fontconfig/fonts.conf` (or any other fontconfig XML
//! file) from type-safe Rust structs.
//!
//! The module is intentionally **separate** from the Neovim [`crate::core::Config`]
//! and Hyprland [`crate::hyprland::HyprlandConfig`] trait families.  Fontconfig
//! nodes implement [`FontconfigConfig`] and are collected inside a
//! [`FontconfigDocument`], which renders a complete, well-formed XML file.
//!
//! # Module layout
//!
//! | Sub-module        | Contents                                             |
//! |-------------------|------------------------------------------------------|
//! | [`core`]          | [`FontconfigConfig`] trait, [`FontconfigRenderContext`], [`FontconfigDocument`] |
//! | [`value`]         | [`value::FontconfigValue`] — typed XML value enum    |
//! | [`alias`]         | [`alias::Alias`] — family substitution aliases       |
//! | [`match_rule`]    | [`match_rule::Match`], [`match_rule::Test`], [`match_rule::Edit`] |
//! | [`dir`]           | [`dir::Dir`], [`dir::CacheDir`], [`dir::Include`]    |
//! | [`select`]        | [`select::SelectFont`] — accept/reject font sets     |
//! | [`description`]   | [`description::Description`] — config file label    |
//!
//! # Quick-start example
//! ```
//! use toconfig::fontconfig::{FontconfigConfig, FontconfigDocument};
//! use toconfig::fontconfig::description::Description;
//! use toconfig::fontconfig::dir::{Dir, CacheDir, Include};
//! use toconfig::fontconfig::alias::Alias;
//! use toconfig::fontconfig::match_rule::{Match, MatchTarget, Test, Edit, EditMode, EditBinding};
//! use toconfig::fontconfig::value::FontconfigValue;
//! use toconfig::fontconfig::select::{SelectFont, SelectAction, Glob};
//!
//! let doc = FontconfigDocument::new()
//!     .push(Description::new("Personal font preferences"))
//!     // Font search paths
//!     .push(Dir::new("~/.local/share/fonts"))
//!     .push(Dir::new("fonts").prefix("xdg"))
//!     // Cache location
//!     .push(CacheDir::new("~/.cache/fontconfig"))
//!     // Include distro defaults
//!     .push(Include::new("conf.d").ignore_missing(true))
//!     // Alias sans-serif to Noto Sans with a DejaVu fallback
//!     .push(
//!         Alias::new("sans-serif")
//!             .prefer(["Noto Sans"])
//!             .accept(["DejaVu Sans"]),
//!     )
//!     .push(
//!         Alias::new("monospace")
//!             .prefer(["JetBrains Mono", "Noto Mono"]),
//!     )
//!     // Substitute Helvetica → Noto Sans
//!     .push(
//!         Match::new()
//!             .target(MatchTarget::Pattern)
//!             .test(Test::new("family", FontconfigValue::string("Helvetica")))
//!             .edit(
//!                 Edit::new("family", FontconfigValue::string("Noto Sans"))
//!                     .mode(EditMode::Prepend)
//!                     .binding(EditBinding::Strong),
//!             ),
//!     )
//!     // Reject bitmap (non-scalable) fonts
//!     .push(
//!         SelectFont::new().block(
//!             SelectAction::Reject,
//!             vec![],
//!             vec![Glob::new("/usr/share/fonts/misc/*")],
//!         ),
//!     );
//!
//! let xml = doc.generate();
//! assert!(xml.starts_with("<?xml version=\"1.0\"?>"));
//! assert!(xml.contains("<fontconfig>"));
//! assert!(xml.contains("<alias>"));
//! assert!(xml.contains("<match target=\"pattern\">"));
//! assert!(xml.contains("<selectfont>"));
//! assert!(xml.contains("</fontconfig>"));
//! ```

pub mod alias;
pub mod core;
pub mod description;
pub mod dir;
pub mod match_rule;
pub mod select;
pub mod value;

// ── Core re-exports ───────────────────────────────────────────────────────────
pub use core::{FontconfigConfig, FontconfigDocument, FontconfigRenderContext};

// ── Sub-module re-exports ─────────────────────────────────────────────────────
pub use alias::Alias;
pub use description::Description;
pub use dir::{CacheDir, Dir, Include};
pub use match_rule::{Edit, EditBinding, EditMode, Match, MatchTarget, Test, TestCompare, TestQual};
pub use select::{Glob, PatternElement, SelectAction, SelectFont, SelectPattern};
pub use value::FontconfigValue;
