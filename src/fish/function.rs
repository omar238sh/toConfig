use super::util::quote_fish_value;
use crate::core::{Config, RenderContext};

// ── Event ─────────────────────────────────────────────────────────────────────

/// Event trigger for a fish function.
#[derive(Debug, Clone)]
pub enum FishEvent {
    /// `--on-event event_name` – fires when `emit event_name` is called.
    Event(String),
    /// `--on-variable var_name` – fires when the variable changes.
    Variable(String),
    /// `--on-process-exit pid_or_cmd` – fires when a process exits.
    ProcessExit(String),
    /// `--on-signal SIGNAL` – fires on the given signal (e.g. `SIGINT`).
    Signal(String),
    /// `--on-job-exit pid_or_cmd` – fires when a job exits.
    JobExit(String),
}

impl FishEvent {
    pub fn to_flag(&self) -> String {
        match self {
            FishEvent::Event(s) => format!("--on-event {}", s),
            FishEvent::Variable(s) => format!("--on-variable {}", s),
            FishEvent::ProcessExit(s) => format!("--on-process-exit {}", s),
            FishEvent::Signal(s) => format!("--on-signal {}", s),
            FishEvent::JobExit(s) => format!("--on-job-exit {}", s),
        }
    }
}

// ── Function ──────────────────────────────────────────────────────────────────

/// A `function ... end` definition.
#[derive(Debug, Clone)]
pub struct FishFunction {
    pub name: String,
    pub body: Vec<String>,
    pub description: Option<String>,
    pub events: Vec<FishEvent>,
    pub argument_names: Vec<String>,
    pub inherit_variable: Vec<String>,
    /// `--wraps cmd` – copies completions from another command.
    pub wrap: Option<String>,
    pub doc: Option<String>,
}

impl FishFunction {
    pub fn new(name: &str, body: &[&str]) -> Self {
        Self {
            name: name.to_string(),
            body: body.iter().map(|s| s.to_string()).collect(),
            description: None,
            events: Vec::new(),
            argument_names: Vec::new(),
            inherit_variable: Vec::new(),
            wrap: None,
            doc: None,
        }
    }

    pub fn description(mut self, d: &str) -> Self {
        self.description = Some(d.to_string());
        self
    }

    pub fn on_event(mut self, event: FishEvent) -> Self {
        self.events.push(event);
        self
    }

    pub fn argument_names(mut self, names: &[&str]) -> Self {
        self.argument_names = names.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn inherit_variable(mut self, vars: &[&str]) -> Self {
        self.inherit_variable = vars.iter().map(|s| s.to_string()).collect();
        self
    }

    /// Copy completions from `cmd` via `--wraps`.
    pub fn wrap(mut self, cmd: &str) -> Self {
        self.wrap = Some(cmd.to_string());
        self
    }

    pub fn doc(mut self, d: &str) -> Self {
        self.doc = Some(d.to_string());
        self
    }
}

impl Config for FishFunction {
    fn render(&self, ctx: &RenderContext) -> String {
        let indent = ctx.indent();
        let inner = ctx.deeper();

        let mut header = format!("function {}", self.name);
        if let Some(ref d) = self.description {
            header.push_str(&format!(" --description {}", quote_fish_value(d)));
        }
        for var in &self.inherit_variable {
            header.push_str(&format!(" --inherit-variable {}", var));
        }
        if !self.argument_names.is_empty() {
            header.push_str(&format!(
                " --argument-names {}",
                self.argument_names.join(" ")
            ));
        }
        if let Some(ref w) = self.wrap {
            header.push_str(&format!(" --wraps {}", w));
        }
        for event in &self.events {
            header.push_str(&format!(" {}", event.to_flag()));
        }

        let mut out = Vec::new();
        out.push(format!("{}{}", indent, header));
        for line in &self.body {
            out.push(format!("{}{}", inner.indent(), line));
        }
        out.push(format!("{}end", indent));
        out.join("\n")
    }

    fn doc_comment(&self) -> Option<&str> {
        self.doc.as_deref()
    }
}
