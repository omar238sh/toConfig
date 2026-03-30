//! `<description>` element — human-readable description for a fontconfig file.

use super::{FontconfigConfig, FontconfigRenderContext};
use super::value::xml_escape;

/// A `<description>` element — a human-readable label for the config file.
///
/// # Example
/// ```
/// use toconfig::fontconfig::FontconfigConfig;
/// use toconfig::fontconfig::description::Description;
///
/// let d = Description::new("My custom font preferences");
/// assert_eq!(d.generate(), "<description>My custom font preferences</description>");
/// ```
pub struct Description {
    pub text: String,
}

impl Description {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

impl FontconfigConfig for Description {
    fn render(&self, ctx: &FontconfigRenderContext) -> String {
        format!(
            "{}<description>{}</description>",
            ctx.indent(),
            xml_escape(&self.text)
        )
    }
}
