use super::{KittyConfig, KittyRenderContext};

/// Raw kitty configuration text — escape hatch for directives not yet modelled
/// by a dedicated struct.
///
/// # Example
/// ```
/// use toconfig::kitty::KittyConfig;
/// use toconfig::kitty::raw::RawKitty;
/// let raw = RawKitty::new("# my custom setting\nsome_key some_value");
/// assert!(raw.generate().contains("some_key some_value"));
/// ```
pub struct RawKitty {
    pub code: String,
}

impl RawKitty {
    pub fn new(code: impl Into<String>) -> Self {
        Self { code: code.into() }
    }
}

impl KittyConfig for RawKitty {
    fn render(&self, ctx: &KittyRenderContext) -> String {
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

/// An `include` directive that pulls in another kitty config file.
///
/// Rendered as: `include <path>`
///
/// # Example
/// ```
/// use toconfig::kitty::KittyConfig;
/// use toconfig::kitty::raw::KittyInclude;
/// let inc = KittyInclude::new("~/.config/kitty/theme.conf");
/// assert_eq!(inc.generate(), "include ~/.config/kitty/theme.conf");
/// ```
pub struct KittyInclude {
    pub path: String,
}

impl KittyInclude {
    pub fn new(path: impl Into<String>) -> Self {
        Self { path: path.into() }
    }
}

impl KittyConfig for KittyInclude {
    fn render(&self, ctx: &KittyRenderContext) -> String {
        format!("{}include {}", ctx.indent(), self.path)
    }
}

/// An inline comment line in the config file.
///
/// # Example
/// ```
/// use toconfig::kitty::KittyConfig;
/// use toconfig::kitty::raw::KittyComment;
/// let c = KittyComment::new("This is a comment");
/// assert_eq!(c.generate(), "# This is a comment");
/// ```
pub struct KittyComment {
    pub text: String,
}

impl KittyComment {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

impl KittyConfig for KittyComment {
    fn render(&self, ctx: &KittyRenderContext) -> String {
        format!("{}# {}", ctx.indent(), self.text)
    }
}

/// A blank separator line.
///
/// Useful for visual grouping of sections inside a [`super::KittyConfigTree`].
pub struct KittyBlank;

impl KittyConfig for KittyBlank {
    fn render(&self, _ctx: &KittyRenderContext) -> String {
        String::new()
    }
}
