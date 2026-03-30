//! Value types used inside fontconfig `<test>` and `<edit>` elements.

use super::FontconfigRenderContext;

/// Escape a string for safe embedding in XML text content.
pub(crate) fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

/// A typed value that appears inside a fontconfig `<test>` or `<edit>` element.
///
/// Fontconfig supports several primitive types as well as composite structures.
/// The variants here cover all commonly-used value types.
///
/// # Example
/// ```
/// use toconfig::fontconfig::value::FontconfigValue;
/// use toconfig::fontconfig::FontconfigRenderContext;
///
/// let ctx = FontconfigRenderContext::default();
/// assert_eq!(FontconfigValue::string("Noto Sans").render_xml(&ctx), "<string>Noto Sans</string>");
/// assert_eq!(FontconfigValue::int(12).render_xml(&ctx), "<int>12</int>");
/// assert_eq!(FontconfigValue::boolean(true).render_xml(&ctx), "<bool>true</bool>");
/// assert_eq!(FontconfigValue::constant("hintslight").render_xml(&ctx), "<const>hintslight</const>");
/// ```
pub enum FontconfigValue {
    /// `<string>text</string>` — a UTF-8 string (XML-escaped automatically).
    Str(String),
    /// `<int>n</int>` — a 64-bit signed integer.
    Int(i64),
    /// `<double>n</double>` — a double-precision float.
    Double(f64),
    /// `<bool>true|false</bool>` — a boolean flag.
    Bool(bool),
    /// `<const>name</const>` — a symbolic constant (e.g. `hintslight`, `rgba`).
    Const(String),
    /// `<name>prop</name>` — a reference to another font property.
    Name(String),
    /// `<list><string>a</string>…</list>` — an ordered list of values.
    List(Vec<FontconfigValue>),
    /// `<range><double>lo</double><double>hi</double></range>` — a numeric range.
    Range(f64, f64),
}

impl FontconfigValue {
    /// Construct a [`FontconfigValue::Str`].
    pub fn string(s: impl Into<String>) -> Self {
        Self::Str(s.into())
    }

    /// Construct a [`FontconfigValue::Int`].
    pub fn int(n: i64) -> Self {
        Self::Int(n)
    }

    /// Construct a [`FontconfigValue::Double`].
    pub fn double(n: f64) -> Self {
        Self::Double(n)
    }

    /// Construct a [`FontconfigValue::Bool`].
    pub fn boolean(b: bool) -> Self {
        Self::Bool(b)
    }

    /// Construct a [`FontconfigValue::Const`] for a symbolic constant.
    pub fn constant(name: impl Into<String>) -> Self {
        Self::Const(name.into())
    }

    /// Construct a [`FontconfigValue::Name`] (property reference).
    pub fn name(prop: impl Into<String>) -> Self {
        Self::Name(prop.into())
    }

    /// Construct a [`FontconfigValue::List`] from an iterator of values.
    pub fn list(items: impl IntoIterator<Item = FontconfigValue>) -> Self {
        Self::List(items.into_iter().collect())
    }

    /// Construct a [`FontconfigValue::Range`].
    pub fn range(lo: f64, hi: f64) -> Self {
        Self::Range(lo, hi)
    }

    /// Render this value as an XML element string at the given context level.
    pub fn render_xml(&self, ctx: &FontconfigRenderContext) -> String {
        let indent = ctx.indent();
        match self {
            Self::Str(s) => format!("{}<string>{}</string>", indent, xml_escape(s)),
            Self::Int(n) => format!("{}<int>{}</int>", indent, n),
            Self::Double(n) => format!("{}<double>{}</double>", indent, n),
            Self::Bool(b) => format!("{}<bool>{}</bool>", indent, b),
            Self::Const(c) => format!("{}<const>{}</const>", indent, xml_escape(c)),
            Self::Name(p) => format!("{}<name>{}</name>", indent, xml_escape(p)),
            Self::Range(lo, hi) => {
                let inner = ctx.deeper();
                let ii = inner.indent();
                format!(
                    "{}<range>\n{}<double>{}</double>\n{}<double>{}</double>\n{}</range>",
                    indent, ii, lo, ii, hi, indent
                )
            }
            Self::List(items) => {
                let inner = ctx.deeper();
                let mut parts = vec![format!("{}<list>", indent)];
                for item in items {
                    parts.push(item.render_xml(&inner));
                }
                parts.push(format!("{}</list>", indent));
                parts.join("\n")
            }
        }
    }
}
