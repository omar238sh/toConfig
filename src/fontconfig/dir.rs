//! `<dir>`, `<cachedir>`, and `<include>` elements — filesystem paths for fontconfig.

use super::{FontconfigConfig, FontconfigRenderContext};
use super::value::xml_escape;

// ── Dir ───────────────────────────────────────────────────────────────────────

/// A `<dir>` element — adds a directory to the font search path.
///
/// # Example
/// ```
/// use toconfig::fontconfig::FontconfigConfig;
/// use toconfig::fontconfig::dir::Dir;
///
/// assert_eq!(
///     Dir::new("~/.local/share/fonts").generate(),
///     "<dir>~/.local/share/fonts</dir>",
/// );
///
/// // XDG base-dir relative path
/// let xdg = Dir::new("fonts").prefix("xdg");
/// assert!(xdg.generate().contains(r#"prefix="xdg""#));
/// ```
pub struct Dir {
    /// Path to the font directory.
    pub path: String,
    /// Optional path prefix type (`"xdg"`, `"relative"`, `"default"`, `"cwd"`, `"~"`, `"/"`).
    pub prefix: Option<String>,
    /// Optional salt string for cache segregation.
    pub salt: Option<String>,
}

impl Dir {
    /// Create a new `<dir>` entry with the given path.
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            prefix: None,
            salt: None,
        }
    }

    /// Set the `prefix` attribute (consuming builder).
    ///
    /// Common values: `"xdg"`, `"relative"`, `"default"`, `"cwd"`, `"~"`, `"/"`.
    pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    /// Set the `salt` attribute for cache segregation (consuming builder).
    pub fn salt(mut self, salt: impl Into<String>) -> Self {
        self.salt = Some(salt.into());
        self
    }
}

impl FontconfigConfig for Dir {
    fn render(&self, ctx: &FontconfigRenderContext) -> String {
        let indent = ctx.indent();
        let mut attrs = String::new();
        if let Some(ref p) = self.prefix {
            attrs.push_str(&format!(" prefix=\"{}\"", xml_escape(p)));
        }
        if let Some(ref s) = self.salt {
            attrs.push_str(&format!(" salt=\"{}\"", xml_escape(s)));
        }
        format!("{}<dir{}>{}</dir>", indent, attrs, xml_escape(&self.path))
    }

    fn validate(&self) -> Result<(), String> {
        if self.path.is_empty() {
            return Err("Dir path must not be empty".into());
        }
        Ok(())
    }
}

// ── CacheDir ─────────────────────────────────────────────────────────────────

/// A `<cachedir>` element — overrides the fontconfig cache directory.
///
/// # Example
/// ```
/// use toconfig::fontconfig::FontconfigConfig;
/// use toconfig::fontconfig::dir::CacheDir;
///
/// assert_eq!(
///     CacheDir::new("~/.cache/fontconfig").generate(),
///     "<cachedir>~/.cache/fontconfig</cachedir>",
/// );
/// ```
pub struct CacheDir {
    /// Path to the cache directory.
    pub path: String,
    /// Optional path prefix type.
    pub prefix: Option<String>,
}

impl CacheDir {
    /// Create a new `<cachedir>` entry.
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            prefix: None,
        }
    }

    /// Set the `prefix` attribute (consuming builder).
    pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }
}

impl FontconfigConfig for CacheDir {
    fn render(&self, ctx: &FontconfigRenderContext) -> String {
        let indent = ctx.indent();
        let attrs = match &self.prefix {
            Some(p) => format!(" prefix=\"{}\"", xml_escape(p)),
            None => String::new(),
        };
        format!(
            "{}<cachedir{}>{}</cachedir>",
            indent,
            attrs,
            xml_escape(&self.path)
        )
    }

    fn validate(&self) -> Result<(), String> {
        if self.path.is_empty() {
            return Err("CacheDir path must not be empty".into());
        }
        Ok(())
    }
}

// ── Include ───────────────────────────────────────────────────────────────────

/// An `<include>` element — includes another fontconfig file or directory.
///
/// # Example
/// ```
/// use toconfig::fontconfig::FontconfigConfig;
/// use toconfig::fontconfig::dir::Include;
///
/// let inc = Include::new("conf.d").ignore_missing(true);
/// assert!(inc.generate().contains(r#"ignore_missing="yes""#));
/// assert!(inc.generate().contains("conf.d"));
/// ```
pub struct Include {
    /// Path to the file or directory to include.
    pub path: String,
    /// Whether to silently skip the include if the path doesn't exist.
    pub ignore_missing: bool,
    /// Optional path prefix type.
    pub prefix: Option<String>,
    /// Whether to recursively scan subdirectories (fontconfig 2.15+).
    pub deprecated: bool,
}

impl Include {
    /// Create a new `<include>` directive.
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            ignore_missing: false,
            prefix: None,
            deprecated: false,
        }
    }

    /// Set whether a missing file/directory should be silently ignored (consuming builder).
    pub fn ignore_missing(mut self, ignore: bool) -> Self {
        self.ignore_missing = ignore;
        self
    }

    /// Set the `prefix` attribute (consuming builder).
    pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }
}

impl FontconfigConfig for Include {
    fn render(&self, ctx: &FontconfigRenderContext) -> String {
        let indent = ctx.indent();
        let mut attrs = String::new();
        if self.ignore_missing {
            attrs.push_str(" ignore_missing=\"yes\"");
        }
        if let Some(ref p) = self.prefix {
            attrs.push_str(&format!(" prefix=\"{}\"", xml_escape(p)));
        }
        format!(
            "{}<include{}>{}</include>",
            indent,
            attrs,
            xml_escape(&self.path)
        )
    }

    fn validate(&self) -> Result<(), String> {
        if self.path.is_empty() {
            return Err("Include path must not be empty".into());
        }
        Ok(())
    }
}
