use super::util::quote_fish_value;
use crate::core::{Config, RenderContext};

// ── Variable ──────────────────────────────────────────────────────────────────

/// Scope flags for a fish `set` variable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VarScope {
    /// `-l` – local to the current block or function.
    Local,
    /// `-g` – global across the shell session.
    Global,
    /// `-U` – universal; persists across sessions in `fish_variables`.
    Universal,
}

impl VarScope {
    pub fn flag(self) -> &'static str {
        match self {
            VarScope::Local => "-l",
            VarScope::Global => "-g",
            VarScope::Universal => "-U",
        }
    }
}

/// A `set` statement for a fish variable (one or more values).
#[derive(Debug, Clone)]
pub struct FishVariable {
    pub name: String,
    pub values: Vec<String>,
    pub scope: Option<VarScope>,
    /// `-x` – export to child processes.
    pub export: bool,
    /// `--path` – treat value as a colon-separated path list.
    pub path_list: bool,
    /// `--erase` – remove the variable.
    pub erase: bool,
    pub doc: Option<String>,
}

impl FishVariable {
    pub fn new(name: &str, value: &str) -> Self {
        Self {
            name: name.to_string(),
            values: vec![value.to_string()],
            scope: None,
            export: false,
            path_list: false,
            erase: false,
            doc: None,
        }
    }

    pub fn with_values(name: &str, values: &[&str]) -> Self {
        Self {
            name: name.to_string(),
            values: values.iter().map(|s| s.to_string()).collect(),
            scope: None,
            export: false,
            path_list: false,
            erase: false,
            doc: None,
        }
    }

    /// Local variable (`set -l`).
    pub fn local(name: &str, value: &str) -> Self {
        Self::new(name, value).scope(VarScope::Local)
    }

    /// Global variable (`set -g`).
    pub fn global(name: &str, value: &str) -> Self {
        Self::new(name, value).scope(VarScope::Global)
    }

    /// Universal variable (`set -U`), persists across sessions.
    pub fn universal(name: &str, value: &str) -> Self {
        Self::new(name, value).scope(VarScope::Universal)
    }

    /// Exported environment variable (`set -gx`).
    pub fn env(name: &str, value: &str) -> Self {
        Self::new(name, value).scope(VarScope::Global).export(true)
    }

    pub fn scope(mut self, s: VarScope) -> Self {
        self.scope = Some(s);
        self
    }

    pub fn export(mut self, v: bool) -> Self {
        self.export = v;
        self
    }

    pub fn path_list(mut self, v: bool) -> Self {
        self.path_list = v;
        self
    }

    /// Mark the variable for erasure (`set --erase`).
    pub fn erase(mut self) -> Self {
        self.erase = true;
        self
    }

    pub fn doc(mut self, d: &str) -> Self {
        self.doc = Some(d.to_string());
        self
    }
}

impl Config for FishVariable {
    fn render(&self, ctx: &RenderContext) -> String {
        let indent = ctx.indent();
        if self.erase {
            let scope_flag = self
                .scope
                .map(|s| format!(" {}", s.flag()))
                .unwrap_or_default();
            return format!("{}set{} --erase {}", indent, scope_flag, self.name);
        }

        let mut flags: Vec<String> = Vec::new();
        if let Some(s) = self.scope {
            flags.push(s.flag().to_string());
        }
        if self.export {
            flags.push("-x".to_string());
        }
        if self.path_list {
            flags.push("--path".to_string());
        }

        let flags_str = if flags.is_empty() {
            String::new()
        } else {
            format!(" {}", flags.join(" "))
        };

        let values_str = self
            .values
            .iter()
            .map(|v| quote_fish_value(v))
            .collect::<Vec<_>>()
            .join(" ");

        format!("{}set{} {} {}", indent, flags_str, self.name, values_str)
    }

    fn doc_comment(&self) -> Option<&str> {
        self.doc.as_deref()
    }
}
