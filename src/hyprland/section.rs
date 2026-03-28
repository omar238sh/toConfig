use super::{HyprlandConfig, HyprlandRenderContext};

/// A single key-value pair inside a [`Section`].
pub struct KeywordPair {
    pub key: String,
    pub value: String,
}

impl KeywordPair {
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }
}

/// An entry that can appear inside a [`Section`].
pub enum SectionEntry {
    Pair(KeywordPair),
    Nested(Section),
    Comment(String),
    Blank,
}

/// A Hyprland configuration section block, e.g. `general { ... }`.
///
/// Covers all named blocks in Hyprland: `general`, `input`, `decoration`,
/// `misc`, `binds`, `cursor`, `render`, `opengl`, `debug`, `group`,
/// `group:groupbar`, and user-defined or plugin sections.
///
/// # Example
/// ```
/// # use toconfig::hyprland::HyprlandConfig;
/// use toconfig::hyprland::section::Section;
/// let sec = Section::new("general")
///     .pair("gaps_in", "5")
///     .pair("gaps_out", "20")
///     .pair("border_size", "2");
///
/// let out = sec.generate();
/// assert!(out.contains("general {"));
/// assert!(out.contains("gaps_in = 5"));
/// ```
pub struct Section {
    pub name: String,
    pub entries: Vec<SectionEntry>,
}

impl Section {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            entries: Vec::new(),
        }
    }

    /// Add a key-value pair (consuming builder).
    pub fn pair(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.entries
            .push(SectionEntry::Pair(KeywordPair::new(key, value)));
        self
    }

    /// Add an inline comment (consuming builder).
    pub fn comment(mut self, text: impl Into<String>) -> Self {
        self.entries.push(SectionEntry::Comment(text.into()));
        self
    }

    /// Add a blank line (consuming builder).
    pub fn blank(mut self) -> Self {
        self.entries.push(SectionEntry::Blank);
        self
    }

    /// Add a nested sub-section (consuming builder).
    pub fn nested(mut self, subsection: Section) -> Self {
        self.entries.push(SectionEntry::Nested(subsection));
        self
    }

    /// Add a key-value pair (mutable borrow).
    pub fn add_pair(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.entries
            .push(SectionEntry::Pair(KeywordPair::new(key, value)));
        self
    }

    /// Add a nested sub-section (mutable borrow).
    pub fn add_nested(&mut self, subsection: Section) -> &mut Self {
        self.entries.push(SectionEntry::Nested(subsection));
        self
    }
}

impl HyprlandConfig for Section {
    fn render(&self, ctx: &HyprlandRenderContext) -> String {
        let indent = ctx.indent();
        let inner_ctx = ctx.deeper();
        let inner_indent = inner_ctx.indent();

        let mut lines = vec![format!("{}{} {{", indent, self.name)];
        for entry in &self.entries {
            match entry {
                SectionEntry::Pair(kv) => {
                    lines.push(format!("{}{} = {}", inner_indent, kv.key, kv.value));
                }
                SectionEntry::Nested(sub) => {
                    lines.push(sub.render(&inner_ctx));
                }
                SectionEntry::Comment(c) => {
                    lines.push(format!("{}# {}", inner_indent, c));
                }
                SectionEntry::Blank => {
                    lines.push(String::new());
                }
            }
        }
        lines.push(format!("{}}}", indent));
        lines.join("\n")
    }
}

/// Raw Hyprland configuration text — escape hatch for directives not yet
/// modelled by a dedicated struct.
///
/// # Example
/// ```
/// # use toconfig::hyprland::HyprlandConfig;
/// use toconfig::hyprland::section::RawHyprland;
/// let raw = RawHyprland::new("misc {\n    disable_hyprland_logo = true\n}");
/// assert!(raw.generate().contains("disable_hyprland_logo"));
/// ```
pub struct RawHyprland {
    pub code: String,
}

impl RawHyprland {
    pub fn new(code: impl Into<String>) -> Self {
        Self { code: code.into() }
    }
}

impl HyprlandConfig for RawHyprland {
    fn render(&self, ctx: &HyprlandRenderContext) -> String {
        let indent = ctx.indent();
        self.code
            .lines()
            .map(|line| {
                if line.is_empty() {
                    String::new()
                } else {
                    format!("{}{}", indent, line)
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}
