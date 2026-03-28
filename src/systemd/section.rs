use super::{SystemdConfig, SystemdRenderContext};

/// A generic systemd INI section rendered as `[Name]\nkey=value\n…`.
///
/// Use this for any section not covered by the typed structs, or for
/// plugin/custom sections.
///
/// # Example
/// ```
/// # use toconfig::systemd::SystemdConfig;
/// use toconfig::systemd::section::SystemdSection;
/// let sec = SystemdSection::new("X-Custom")
///     .pair("Foo", "bar")
///     .pair("Baz", "qux");
/// let out = sec.generate();
/// assert!(out.contains("[X-Custom]"));
/// assert!(out.contains("Foo=bar"));
/// ```
pub struct SystemdSection {
    pub name: String,
    pub entries: Vec<SectionEntry>,
}

/// An entry inside a [`SystemdSection`].
pub enum SectionEntry {
    Pair(String, String),
    Comment(String),
    Blank,
}

impl SystemdSection {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            entries: Vec::new(),
        }
    }

    /// Add a key=value pair (consuming builder).
    pub fn pair(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.entries
            .push(SectionEntry::Pair(key.into(), value.into()));
        self
    }

    /// Add a comment line (consuming builder).
    pub fn comment(mut self, text: impl Into<String>) -> Self {
        self.entries.push(SectionEntry::Comment(text.into()));
        self
    }

    /// Add a blank line (consuming builder).
    pub fn blank(mut self) -> Self {
        self.entries.push(SectionEntry::Blank);
        self
    }

    /// Add a key=value pair (mutable borrow).
    pub fn add_pair(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.entries
            .push(SectionEntry::Pair(key.into(), value.into()));
        self
    }
}

impl SystemdConfig for SystemdSection {
    fn render(&self, _ctx: &SystemdRenderContext) -> String {
        let mut lines = vec![format!("[{}]", self.name)];
        for entry in &self.entries {
            match entry {
                SectionEntry::Pair(k, v) => lines.push(format!("{}={}", k, v)),
                SectionEntry::Comment(c) => lines.push(format!("# {}", c)),
                SectionEntry::Blank => lines.push(String::new()),
            }
        }
        lines.join("\n")
    }
}

/// Raw systemd unit-file text — escape hatch for directives not yet modelled
/// by a dedicated struct.
///
/// # Example
/// ```
/// # use toconfig::systemd::SystemdConfig;
/// use toconfig::systemd::section::RawSystemd;
/// let raw = RawSystemd::new("[X-Custom]\nFoo=bar");
/// assert!(raw.generate().contains("Foo=bar"));
/// ```
pub struct RawSystemd {
    pub code: String,
}

impl RawSystemd {
    pub fn new(code: impl Into<String>) -> Self {
        Self { code: code.into() }
    }
}

impl SystemdConfig for RawSystemd {
    fn render(&self, _ctx: &SystemdRenderContext) -> String {
        self.code.clone()
    }
}
