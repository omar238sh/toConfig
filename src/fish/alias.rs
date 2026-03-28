use super::util::quote_fish_value;
use crate::core::{Config, RenderContext};

/// An `alias name 'expansion'` statement.
#[derive(Debug, Clone)]
pub struct FishAlias {
    pub name: String,
    pub command: String,
    pub doc: Option<String>,
}

impl FishAlias {
    pub fn new(name: &str, command: &str) -> Self {
        Self {
            name: name.to_string(),
            command: command.to_string(),
            doc: None,
        }
    }

    pub fn doc(mut self, d: &str) -> Self {
        self.doc = Some(d.to_string());
        self
    }
}

impl Config for FishAlias {
    fn render(&self, ctx: &RenderContext) -> String {
        format!(
            "{}alias {} {}",
            ctx.indent(),
            self.name,
            quote_fish_value(&self.command)
        )
    }

    fn doc_comment(&self) -> Option<&str> {
        self.doc.as_deref()
    }
}
