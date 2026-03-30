use super::core::{HelixConfig, HelixRenderContext, toml_str};

/// A generic TOML section (`[header]`) built from key-value pairs.
///
/// Use this as an escape hatch for Helix config sections that do not yet
/// have a dedicated struct.  Values are always rendered as quoted TOML
/// strings; use [`RawToml`] when you need unquoted values.
///
/// # Example
/// ```
/// # use toconfig::helix::HelixConfig;
/// use toconfig::helix::section::TomlSection;
/// let sec = TomlSection::new("editor.gutters")
///     .pair("layout", "[\"diagnostics\", \"line-numbers\"]");
///
/// let out = sec.generate();
/// assert!(out.contains("[editor.gutters]"));
/// assert!(out.contains("layout"));
/// ```
pub struct TomlSection {
    pub header: String,
    pub entries: Vec<TomlEntry>,
}

/// A single entry inside a [`TomlSection`].
pub enum TomlEntry {
    /// A key mapped to a pre-formatted TOML value (not further quoted).
    Raw(String, String),
    /// A key mapped to a quoted string value.
    Str(String, String),
    /// A blank line for readability.
    Blank,
    /// An inline comment.
    Comment(String),
}

impl TomlSection {
    pub fn new(header: impl Into<String>) -> Self {
        Self {
            header: header.into(),
            entries: Vec::new(),
        }
    }

    /// Add a key with a **raw** (already-formatted TOML) value.
    ///
    /// The value is inserted verbatim — use this for numbers, booleans, or
    /// pre-built arrays.
    pub fn pair(mut self, key: impl Into<String>, raw_value: impl Into<String>) -> Self {
        self.entries
            .push(TomlEntry::Raw(key.into(), raw_value.into()));
        self
    }

    /// Add a key with a **string** value (will be double-quoted).
    pub fn str_pair(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.entries
            .push(TomlEntry::Str(key.into(), value.into()));
        self
    }

    /// Add a blank line.
    pub fn blank(mut self) -> Self {
        self.entries.push(TomlEntry::Blank);
        self
    }

    /// Add an inline comment.
    pub fn comment(mut self, text: impl Into<String>) -> Self {
        self.entries.push(TomlEntry::Comment(text.into()));
        self
    }

    /// Add a key with a **raw** value (mutable borrow).
    pub fn add_pair(
        &mut self,
        key: impl Into<String>,
        raw_value: impl Into<String>,
    ) -> &mut Self {
        self.entries
            .push(TomlEntry::Raw(key.into(), raw_value.into()));
        self
    }

    /// Add a key with a **string** value (mutable borrow).
    pub fn add_str_pair(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> &mut Self {
        self.entries
            .push(TomlEntry::Str(key.into(), value.into()));
        self
    }
}

impl HelixConfig for TomlSection {
    fn render(&self, _ctx: &HelixRenderContext) -> String {
        let mut lines = vec![format!("[{}]", self.header)];
        for entry in &self.entries {
            match entry {
                TomlEntry::Raw(k, v) => lines.push(format!("{} = {}", k, v)),
                TomlEntry::Str(k, v) => lines.push(format!("{} = {}", k, toml_str(v))),
                TomlEntry::Blank => lines.push(String::new()),
                TomlEntry::Comment(c) => lines.push(format!("# {}", c)),
            }
        }
        lines.join("\n")
    }
}

/// Raw TOML text — escape hatch for config fragments not yet modelled by a
/// dedicated struct.
///
/// The text is emitted verbatim with no modification.
///
/// # Example
/// ```
/// # use toconfig::helix::HelixConfig;
/// use toconfig::helix::section::RawToml;
/// let raw = RawToml::new("[editor.auto-pairs]\n\"(\" = \")\"\n\"[\" = \"]\"");
/// assert!(raw.generate().contains("[editor.auto-pairs]"));
/// ```
pub struct RawToml {
    pub text: String,
}

impl RawToml {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

impl HelixConfig for RawToml {
    fn render(&self, _ctx: &HelixRenderContext) -> String {
        self.text.clone()
    }
}
