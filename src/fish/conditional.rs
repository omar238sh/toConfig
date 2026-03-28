use super::util::quote_fish_value;
use crate::core::{Config, RenderContext};

// ── Condition ─────────────────────────────────────────────────────────────────

/// A condition expression for a fish `if` or `while` block.
#[derive(Debug, Clone)]
pub enum FishCondition {
    /// `status is-interactive`
    IsInteractive,
    /// `status is-login`
    IsLogin,
    /// `status is-command-substitution`
    IsCommandSubstitution,
    /// `status is-block`
    IsBlock,
    /// Arbitrary expression, e.g. `"command -q nvim"`.
    Raw(String),
}

impl FishCondition {
    pub fn to_expr(&self) -> String {
        match self {
            FishCondition::IsInteractive => "status is-interactive".to_string(),
            FishCondition::IsLogin => "status is-login".to_string(),
            FishCondition::IsCommandSubstitution => "status is-command-substitution".to_string(),
            FishCondition::IsBlock => "status is-block".to_string(),
            FishCondition::Raw(s) => s.clone(),
        }
    }
}

// ── If ────────────────────────────────────────────────────────────────────────

/// A branch inside a `FishIf` block (condition + body).
///
/// Represents an `else if condition` clause.
pub struct FishElseIf {
    pub condition: FishCondition,
    pub body: Vec<Box<dyn Config>>,
}

/// A fish `if … else if … else … end` block.
pub struct FishIf {
    pub condition: FishCondition,
    pub body: Vec<Box<dyn Config>>,
    pub else_if_branches: Vec<FishElseIf>,
    pub else_body: Vec<Box<dyn Config>>,
}

impl FishIf {
    pub fn new(condition: FishCondition) -> Self {
        Self {
            condition,
            body: Vec::new(),
            else_if_branches: Vec::new(),
            else_body: Vec::new(),
        }
    }

    pub fn add<C: Config + 'static>(mut self, node: C) -> Self {
        self.body.push(Box::new(node));
        self
    }

    pub fn else_if(mut self, condition: FishCondition, nodes: Vec<Box<dyn Config>>) -> Self {
        self.else_if_branches.push(FishElseIf {
            condition,
            body: nodes,
        });
        self
    }

    pub fn add_else<C: Config + 'static>(mut self, node: C) -> Self {
        self.else_body.push(Box::new(node));
        self
    }
}

impl Config for FishIf {
    fn render(&self, ctx: &RenderContext) -> String {
        let indent = ctx.indent();
        let inner = ctx.deeper();
        let mut out = Vec::new();

        out.push(format!("{}if {}", indent, self.condition.to_expr()));
        for node in &self.body {
            out.push(node.render(&inner));
        }
        for branch in &self.else_if_branches {
            out.push(format!("{}else if {}", indent, branch.condition.to_expr()));
            for node in &branch.body {
                out.push(node.render(&inner));
            }
        }
        if !self.else_body.is_empty() {
            out.push(format!("{}else", indent));
            for node in &self.else_body {
                out.push(node.render(&inner));
            }
        }
        out.push(format!("{}end", indent));
        out.join("\n")
    }
}

// ── Switch / Case ─────────────────────────────────────────────────────────────

/// A single `case pattern … end` arm inside a `switch`.
pub struct FishCase {
    pub pattern: String,
    pub body: Vec<Box<dyn Config>>,
}

/// A fish `switch value … end` statement.
pub struct FishSwitch {
    pub value: String,
    pub cases: Vec<FishCase>,
}

impl FishSwitch {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
            cases: Vec::new(),
        }
    }

    pub fn case(mut self, pattern: &str, nodes: Vec<Box<dyn Config>>) -> Self {
        self.cases.push(FishCase {
            pattern: pattern.to_string(),
            body: nodes,
        });
        self
    }
}

impl Config for FishSwitch {
    fn render(&self, ctx: &RenderContext) -> String {
        let indent = ctx.indent();
        let inner = ctx.deeper();
        let body_indent = inner.deeper();
        let mut out = Vec::new();

        out.push(format!(
            "{}switch {}",
            indent,
            quote_fish_value(&self.value)
        ));
        for arm in &self.cases {
            out.push(format!("{}case {}", inner.indent(), arm.pattern));
            for node in &arm.body {
                out.push(node.render(&body_indent));
            }
        }
        out.push(format!("{}end", indent));
        out.join("\n")
    }
}

// ── For Loop ──────────────────────────────────────────────────────────────────

/// A fish `for var in items … end` loop.
pub struct FishFor {
    pub variable: String,
    pub items: Vec<String>,
    pub body: Vec<Box<dyn Config>>,
}

impl FishFor {
    pub fn new(variable: &str, items: &[&str]) -> Self {
        Self {
            variable: variable.to_string(),
            items: items.iter().map(|s| s.to_string()).collect(),
            body: Vec::new(),
        }
    }

    pub fn add<C: Config + 'static>(mut self, node: C) -> Self {
        self.body.push(Box::new(node));
        self
    }
}

impl Config for FishFor {
    fn render(&self, ctx: &RenderContext) -> String {
        let indent = ctx.indent();
        let inner = ctx.deeper();
        let items_str = self
            .items
            .iter()
            .map(|i| quote_fish_value(i))
            .collect::<Vec<_>>()
            .join(" ");
        let mut out = Vec::new();
        out.push(format!("{}for {} in {}", indent, self.variable, items_str));
        for node in &self.body {
            out.push(node.render(&inner));
        }
        out.push(format!("{}end", indent));
        out.join("\n")
    }
}

// ── While Loop ────────────────────────────────────────────────────────────────

/// A fish `while condition … end` loop.
pub struct FishWhile {
    pub condition: FishCondition,
    pub body: Vec<Box<dyn Config>>,
}

impl FishWhile {
    pub fn new(condition: FishCondition) -> Self {
        Self {
            condition,
            body: Vec::new(),
        }
    }

    pub fn add<C: Config + 'static>(mut self, node: C) -> Self {
        self.body.push(Box::new(node));
        self
    }
}

impl Config for FishWhile {
    fn render(&self, ctx: &RenderContext) -> String {
        let indent = ctx.indent();
        let inner = ctx.deeper();
        let mut out = Vec::new();
        out.push(format!("{}while {}", indent, self.condition.to_expr()));
        for node in &self.body {
            out.push(node.render(&inner));
        }
        out.push(format!("{}end", indent));
        out.join("\n")
    }
}

// ── Begin Block ───────────────────────────────────────────────────────────────

/// A fish `begin … end` block for grouping commands or redirections.
pub struct FishBegin {
    pub body: Vec<Box<dyn Config>>,
    pub comment: Option<String>,
}

impl Default for FishBegin {
    fn default() -> Self {
        Self {
            body: Vec::new(),
            comment: None,
        }
    }
}

impl FishBegin {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_comment(mut self, c: &str) -> Self {
        self.comment = Some(c.to_string());
        self
    }

    pub fn add<C: Config + 'static>(mut self, node: C) -> Self {
        self.body.push(Box::new(node));
        self
    }
}

impl Config for FishBegin {
    fn render(&self, ctx: &RenderContext) -> String {
        let indent = ctx.indent();
        let inner = ctx.deeper();
        let mut out = Vec::new();
        if let Some(ref c) = self.comment {
            out.push(format!("{}# {}", indent, c));
        }
        out.push(format!("{}begin", indent));
        for node in &self.body {
            out.push(node.render(&inner));
        }
        out.push(format!("{}end", indent));
        out.join("\n")
    }
}
