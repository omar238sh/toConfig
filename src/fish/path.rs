use super::util::quote_fish_value;
use crate::core::{Config, RenderContext};

/// Adds one or more entries to `$PATH` via `fish_add_path`.
#[derive(Debug, Clone)]
pub struct FishAddPath {
    pub paths: Vec<String>,
    /// Add at the front of `$PATH` (`--prepend`).
    pub prepend: bool,
    /// Modify the global PATH, not the universal fish_user_paths.
    pub global: bool,
    /// Move already-present entries to the front/back.
    pub move_to_front: bool,
}

impl FishAddPath {
    pub fn new(paths: &[&str]) -> Self {
        Self {
            paths: paths.iter().map(|s| s.to_string()).collect(),
            prepend: false,
            global: false,
            move_to_front: false,
        }
    }

    pub fn prepend(mut self) -> Self {
        self.prepend = true;
        self
    }

    pub fn global(mut self) -> Self {
        self.global = true;
        self
    }

    pub fn move_to_front(mut self) -> Self {
        self.move_to_front = true;
        self
    }
}

impl Config for FishAddPath {
    fn render(&self, ctx: &RenderContext) -> String {
        let mut parts = vec!["fish_add_path".to_string()];
        if self.prepend {
            parts.push("--prepend".to_string());
        }
        if self.global {
            parts.push("--global".to_string());
        }
        if self.move_to_front {
            parts.push("--move".to_string());
        }
        for p in &self.paths {
            parts.push(quote_fish_value(p));
        }
        format!("{}{}", ctx.indent(), parts.join(" "))
    }
}
