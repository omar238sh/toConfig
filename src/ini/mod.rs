//! INI-format rendering infrastructure.
//!
//! This module provides the [`IniConfig`] trait and the [`IniSection`] /
//! [`IniFile`] helpers that back the [`gtk`](crate::gtk) and [`qt`](crate::qt)
//! configuration backends.

/// Rendering context for INI-format configuration files.
#[derive(Debug, Clone, Default)]
pub struct IniRenderContext {
    /// When `true`, optional comment lines are emitted above sections.
    pub emit_comments: bool,
}

/// Trait for types that can render themselves to an INI-format string.
pub trait IniConfig {
    /// Render this node into an INI string using the provided context.
    fn render(&self, ctx: &IniRenderContext) -> String;

    /// Convenience: render with a default context.
    fn generate(&self) -> String {
        self.render(&IniRenderContext::default())
    }

    /// Optional validation step run before rendering.
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// A single key = value pair inside an INI section.
#[derive(Debug, Clone)]
pub struct IniEntry {
    pub key: String,
    pub value: String,
}

/// An INI section (`[Name]`) holding an ordered list of key-value entries.
///
/// # Example
/// ```
/// use toconfig::ini::{IniSection, IniConfig};
///
/// let s = IniSection::new("Settings")
///     .set("gtk-theme-name", "Catppuccin-Mocha-Standard-Blue-Dark")
///     .set("gtk-xft-antialias", 1);
///
/// assert!(s.generate().starts_with("[Settings]"));
/// ```
#[derive(Debug, Clone)]
pub struct IniSection {
    pub name: String,
    pub entries: Vec<IniEntry>,
}

impl IniSection {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            entries: Vec::new(),
        }
    }

    /// Append a key-value entry and return `self` (builder style).
    pub fn set(mut self, key: &str, value: impl ToString) -> Self {
        self.entries.push(IniEntry {
            key: key.to_string(),
            value: value.to_string(),
        });
        self
    }
}

impl IniConfig for IniSection {
    fn render(&self, _ctx: &IniRenderContext) -> String {
        let mut lines = vec![format!("[{}]", self.name)];
        for entry in &self.entries {
            lines.push(format!("{}={}", entry.key, entry.value));
        }
        lines.join("\n")
    }
}

/// A complete INI file composed of an ordered list of [`IniSection`]s.
///
/// # Example
/// ```
/// use toconfig::ini::{IniFile, IniSection, IniConfig};
///
/// let file = IniFile::new()
///     .section(IniSection::new("A").set("x", 1))
///     .section(IniSection::new("B").set("y", 2));
///
/// let out = file.generate();
/// assert!(out.contains("[A]"));
/// assert!(out.contains("[B]"));
/// ```
#[derive(Debug, Clone, Default)]
pub struct IniFile {
    pub sections: Vec<IniSection>,
}

impl IniFile {
    pub fn new() -> Self {
        Self::default()
    }

    /// Append a section and return `self` (builder style).
    pub fn section(mut self, s: IniSection) -> Self {
        self.sections.push(s);
        self
    }
}

impl IniConfig for IniFile {
    fn render(&self, ctx: &IniRenderContext) -> String {
        self.sections
            .iter()
            .map(|s| s.render(ctx))
            .collect::<Vec<_>>()
            .join("\n\n")
    }
}
