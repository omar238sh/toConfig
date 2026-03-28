use super::util::quote_fish_value;
use crate::core::{Config, RenderContext};

/// A `complete` statement for tab-completion.
#[derive(Debug, Clone)]
pub struct FishCompletion {
    pub command: String,
    pub short_option: Option<String>,
    pub long_option: Option<String>,
    pub description: Option<String>,
    /// `-r` – the option requires an argument.
    pub requires_argument: bool,
    /// `-f` – the option takes no argument (disables file completions for this option).
    pub no_argument: bool,
    /// `--no-files` – suppress file completions entirely.
    pub no_files: bool,
    /// `--force-files` – allow file completions even when `--no-files` was set by another rule.
    pub force_files: bool,
    /// `--keep-order` – preserve argument order instead of sorting.
    pub keep_order: bool,
    /// `-n condition` – only complete when `condition` is true.
    pub condition: Option<String>,
    /// `-a arguments` – the possible argument values.
    pub arguments: Option<String>,
    /// `--wraps cmd` – reuse completions from another command.
    pub wraps: Option<String>,
    pub doc: Option<String>,
}

impl FishCompletion {
    pub fn new(command: &str) -> Self {
        Self {
            command: command.to_string(),
            short_option: None,
            long_option: None,
            description: None,
            requires_argument: false,
            no_argument: false,
            no_files: false,
            force_files: false,
            keep_order: false,
            condition: None,
            arguments: None,
            wraps: None,
            doc: None,
        }
    }

    pub fn short(mut self, s: &str) -> Self {
        self.short_option = Some(s.to_string());
        self
    }

    pub fn long(mut self, l: &str) -> Self {
        self.long_option = Some(l.to_string());
        self
    }

    pub fn description(mut self, d: &str) -> Self {
        self.description = Some(d.to_string());
        self
    }

    pub fn requires_argument(mut self) -> Self {
        self.requires_argument = true;
        self
    }

    pub fn no_argument(mut self) -> Self {
        self.no_argument = true;
        self
    }

    pub fn no_files(mut self) -> Self {
        self.no_files = true;
        self
    }

    pub fn force_files(mut self) -> Self {
        self.force_files = true;
        self
    }

    pub fn keep_order(mut self) -> Self {
        self.keep_order = true;
        self
    }

    pub fn condition(mut self, c: &str) -> Self {
        self.condition = Some(c.to_string());
        self
    }

    pub fn arguments(mut self, a: &str) -> Self {
        self.arguments = Some(a.to_string());
        self
    }

    pub fn wraps(mut self, w: &str) -> Self {
        self.wraps = Some(w.to_string());
        self
    }

    pub fn doc(mut self, d: &str) -> Self {
        self.doc = Some(d.to_string());
        self
    }
}

impl Config for FishCompletion {
    fn render(&self, ctx: &RenderContext) -> String {
        let mut parts = vec!["complete".to_string(), format!("-c {}", self.command)];

        if let Some(ref s) = self.short_option {
            parts.push(format!("-s {}", s));
        }
        if let Some(ref l) = self.long_option {
            parts.push(format!("-l {}", l));
        }
        if self.requires_argument {
            parts.push("-r".to_string());
        }
        if self.no_argument {
            parts.push("-f".to_string());
        }
        if self.no_files {
            parts.push("--no-files".to_string());
        }
        if self.force_files {
            parts.push("--force-files".to_string());
        }
        if self.keep_order {
            parts.push("--keep-order".to_string());
        }
        if let Some(ref c) = self.condition {
            parts.push(format!("-n {}", quote_fish_value(c)));
        }
        if let Some(ref a) = self.arguments {
            parts.push(format!("-a {}", quote_fish_value(a)));
        }
        if let Some(ref w) = self.wraps {
            parts.push(format!("--wraps {}", w));
        }
        if let Some(ref d) = self.description {
            parts.push(format!("-d {}", quote_fish_value(d)));
        }

        format!("{}{}", ctx.indent(), parts.join(" "))
    }

    fn doc_comment(&self) -> Option<&str> {
        self.doc.as_deref()
    }
}
