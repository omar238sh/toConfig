//! `<match>`, `<test>`, and `<edit>` elements — conditional font substitution rules.

use super::{FontconfigConfig, FontconfigRenderContext};
use super::value::{FontconfigValue, xml_escape};

// ── MatchTarget ───────────────────────────────────────────────────────────────

/// The `target` attribute of a `<match>` element.
///
/// Controls which stage of font lookup the match rule applies to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MatchTarget {
    /// Apply to the initial pattern before font lookup (default).
    #[default]
    Pattern,
    /// Apply after a font has been selected.
    Font,
    /// Apply during the font scanning phase.
    Scan,
}

impl MatchTarget {
    fn as_str(self) -> &'static str {
        match self {
            Self::Pattern => "pattern",
            Self::Font => "font",
            Self::Scan => "scan",
        }
    }
}

// ── TestQual ─────────────────────────────────────────────────────────────────

/// The `qual` attribute of a `<test>` element.
///
/// Determines which elements of a list property are compared.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestQual {
    /// True if *any* element matches (default).
    Any,
    /// True only if *all* elements match.
    All,
    /// True if the *first* element matches.
    First,
    /// True if *no* element matches.
    Not,
}

impl TestQual {
    fn as_str(self) -> &'static str {
        match self {
            Self::Any => "any",
            Self::All => "all",
            Self::First => "first",
            Self::Not => "not",
        }
    }
}

// ── TestCompare ───────────────────────────────────────────────────────────────

/// The `compare` attribute of a `<test>` element.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestCompare {
    /// Property equals value (default).
    Eq,
    /// Property is less than value.
    Less,
    /// Property is less than or equal to value.
    LessEq,
    /// Property is greater than value.
    More,
    /// Property is greater than or equal to value.
    MoreEq,
    /// Property does not equal value.
    NotEq,
    /// Property list contains value.
    Contains,
    /// Property list does not contain value.
    NotContains,
}

impl TestCompare {
    fn as_str(self) -> &'static str {
        match self {
            Self::Eq => "eq",
            Self::Less => "less",
            Self::LessEq => "less_eq",
            Self::More => "more",
            Self::MoreEq => "more_eq",
            Self::NotEq => "not_eq",
            Self::Contains => "contains",
            Self::NotContains => "not_contains",
        }
    }
}

// ── EditMode ─────────────────────────────────────────────────────────────────

/// The `mode` attribute of an `<edit>` element.
///
/// Controls how the new value is combined with any existing property values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditMode {
    /// Replace the value with the new value (default).
    Assign,
    /// Replace the entire list with the new value.
    AssignReplace,
    /// Insert the new value before the first matching value.
    Prepend,
    /// Insert the new value at the beginning of the list.
    PrependFirst,
    /// Append the new value after the last matching value.
    Append,
    /// Append the new value at the end of the list.
    AppendLast,
    /// Delete matching values.
    Delete,
    /// Delete all values.
    DeleteAll,
}

impl EditMode {
    fn as_str(self) -> &'static str {
        match self {
            Self::Assign => "assign",
            Self::AssignReplace => "assign_replace",
            Self::Prepend => "prepend",
            Self::PrependFirst => "prepend_first",
            Self::Append => "append",
            Self::AppendLast => "append_last",
            Self::Delete => "delete",
            Self::DeleteAll => "delete_all",
        }
    }
}

// ── EditBinding ───────────────────────────────────────────────────────────────

/// The `binding` attribute of an `<edit>` element.
///
/// Controls the substitution binding strength when resolving font families.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditBinding {
    /// Weak binding — easily overridden.
    Weak,
    /// Strong binding — not overridden by weaker substitutions.
    Strong,
    /// Preserve the original binding strength.
    Same,
}

impl EditBinding {
    fn as_str(self) -> &'static str {
        match self {
            Self::Weak => "weak",
            Self::Strong => "strong",
            Self::Same => "same",
        }
    }
}

// ── Test ─────────────────────────────────────────────────────────────────────

/// A `<test>` element inside a [`Match`] rule.
///
/// Tests a single font property against a value.  All `<test>` elements in a
/// `<match>` must pass for the `<edit>` actions to be applied.
///
/// # Example
/// ```
/// use toconfig::fontconfig::match_rule::{Test, TestCompare};
/// use toconfig::fontconfig::value::FontconfigValue;
/// use toconfig::fontconfig::FontconfigConfig;
///
/// let t = Test::new("family", FontconfigValue::string("Helvetica"))
///     .compare(TestCompare::Eq);
/// let out = t.generate();
/// assert!(out.contains(r#"name="family""#));
/// assert!(out.contains("<string>Helvetica</string>"));
/// ```
pub struct Test {
    /// Font property name (e.g. `"family"`, `"weight"`, `"slant"`).
    pub name: String,
    /// Value to compare against.
    pub value: FontconfigValue,
    /// Qualification for list properties.
    pub qual: Option<TestQual>,
    /// Comparison operator.
    pub compare: Option<TestCompare>,
    /// Which target the test applies to (overrides the parent `<match>` target).
    pub target: Option<MatchTarget>,
}

impl Test {
    /// Create a basic equality test on `name`.
    pub fn new(name: impl Into<String>, value: FontconfigValue) -> Self {
        Self {
            name: name.into(),
            value,
            qual: None,
            compare: None,
            target: None,
        }
    }

    /// Set the `qual` attribute (consuming builder).
    pub fn qual(mut self, qual: TestQual) -> Self {
        self.qual = Some(qual);
        self
    }

    /// Set the `compare` attribute (consuming builder).
    pub fn compare(mut self, compare: TestCompare) -> Self {
        self.compare = Some(compare);
        self
    }

    /// Set the `target` attribute (consuming builder).
    pub fn target(mut self, target: MatchTarget) -> Self {
        self.target = Some(target);
        self
    }
}

impl FontconfigConfig for Test {
    fn render(&self, ctx: &FontconfigRenderContext) -> String {
        let indent = ctx.indent();
        let inner_ctx = ctx.deeper();

        let mut attrs = format!("name=\"{}\"", xml_escape(&self.name));
        if let Some(q) = self.qual {
            attrs.push_str(&format!(" qual=\"{}\"", q.as_str()));
        }
        if let Some(c) = self.compare {
            attrs.push_str(&format!(" compare=\"{}\"", c.as_str()));
        }
        if let Some(t) = self.target {
            attrs.push_str(&format!(" target=\"{}\"", t.as_str()));
        }

        format!(
            "{}<test {}>\n{}\n{}</test>",
            indent,
            attrs,
            self.value.render_xml(&inner_ctx),
            indent
        )
    }

    fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Test property name must not be empty".into());
        }
        Ok(())
    }
}

// ── Edit ─────────────────────────────────────────────────────────────────────

/// An `<edit>` element inside a [`Match`] rule.
///
/// Modifies a font property when the match rule's tests all pass.
///
/// # Example
/// ```
/// use toconfig::fontconfig::match_rule::{Edit, EditMode, EditBinding};
/// use toconfig::fontconfig::value::FontconfigValue;
/// use toconfig::fontconfig::FontconfigConfig;
///
/// let e = Edit::new("family", FontconfigValue::string("Noto Sans"))
///     .mode(EditMode::Prepend)
///     .binding(EditBinding::Strong);
/// let out = e.generate();
/// assert!(out.contains(r#"name="family""#));
/// assert!(out.contains(r#"mode="prepend""#));
/// assert!(out.contains(r#"binding="strong""#));
/// assert!(out.contains("<string>Noto Sans</string>"));
/// ```
pub struct Edit {
    /// Font property to modify (e.g. `"family"`, `"hinting"`, `"rgba"`).
    pub name: String,
    /// New value to apply.
    pub value: FontconfigValue,
    /// How to combine the new value with existing values.
    pub mode: Option<EditMode>,
    /// Binding strength for the substitution.
    pub binding: Option<EditBinding>,
}

impl Edit {
    /// Create a new edit action on `name`.
    pub fn new(name: impl Into<String>, value: FontconfigValue) -> Self {
        Self {
            name: name.into(),
            value,
            mode: None,
            binding: None,
        }
    }

    /// Set the `mode` attribute (consuming builder).
    pub fn mode(mut self, mode: EditMode) -> Self {
        self.mode = Some(mode);
        self
    }

    /// Set the `binding` attribute (consuming builder).
    pub fn binding(mut self, binding: EditBinding) -> Self {
        self.binding = Some(binding);
        self
    }
}

impl FontconfigConfig for Edit {
    fn render(&self, ctx: &FontconfigRenderContext) -> String {
        let indent = ctx.indent();
        let inner_ctx = ctx.deeper();

        let mut attrs = format!("name=\"{}\"", xml_escape(&self.name));
        if let Some(m) = self.mode {
            attrs.push_str(&format!(" mode=\"{}\"", m.as_str()));
        }
        if let Some(b) = self.binding {
            attrs.push_str(&format!(" binding=\"{}\"", b.as_str()));
        }

        format!(
            "{}<edit {}>\n{}\n{}</edit>",
            indent,
            attrs,
            self.value.render_xml(&inner_ctx),
            indent
        )
    }

    fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Edit property name must not be empty".into());
        }
        Ok(())
    }
}

// ── Match ─────────────────────────────────────────────────────────────────────

/// A `<match>` element — conditional font substitution rule.
///
/// When all [`Test`]s pass, all [`Edit`]s are applied.
///
/// # Example
/// ```
/// use toconfig::fontconfig::FontconfigConfig;
/// use toconfig::fontconfig::match_rule::{Match, MatchTarget, Test, Edit, EditMode, EditBinding};
/// use toconfig::fontconfig::value::FontconfigValue;
///
/// let rule = Match::new()
///     .target(MatchTarget::Pattern)
///     .test(Test::new("family", FontconfigValue::string("Helvetica")))
///     .edit(
///         Edit::new("family", FontconfigValue::string("Helvetica Neue"))
///             .mode(EditMode::Prepend)
///             .binding(EditBinding::Strong),
///     );
///
/// let out = rule.generate();
/// assert!(out.contains(r#"<match target="pattern">"#));
/// assert!(out.contains("<test"));
/// assert!(out.contains("<edit"));
/// ```
pub struct Match {
    /// Which stage the rule applies to.
    pub target: Option<MatchTarget>,
    /// Conditions that must all pass.
    pub tests: Vec<Test>,
    /// Modifications to apply when all tests pass.
    pub edits: Vec<Edit>,
}

impl Default for Match {
    fn default() -> Self {
        Self::new()
    }
}

impl Match {
    pub fn new() -> Self {
        Self {
            target: None,
            tests: Vec::new(),
            edits: Vec::new(),
        }
    }

    /// Set the `target` attribute (consuming builder).
    pub fn target(mut self, target: MatchTarget) -> Self {
        self.target = Some(target);
        self
    }

    /// Append a `<test>` element (consuming builder).
    pub fn test(mut self, test: Test) -> Self {
        self.tests.push(test);
        self
    }

    /// Append an `<edit>` element (consuming builder).
    pub fn edit(mut self, edit: Edit) -> Self {
        self.edits.push(edit);
        self
    }

    /// Append a `<test>` element (mutable borrow).
    pub fn add_test(&mut self, test: Test) -> &mut Self {
        self.tests.push(test);
        self
    }

    /// Append an `<edit>` element (mutable borrow).
    pub fn add_edit(&mut self, edit: Edit) -> &mut Self {
        self.edits.push(edit);
        self
    }
}

impl FontconfigConfig for Match {
    fn render(&self, ctx: &FontconfigRenderContext) -> String {
        let indent = ctx.indent();
        let inner_ctx = ctx.deeper();

        let open_tag = if let Some(t) = self.target {
            format!(r#"{}<match target="{}">"#, indent, t.as_str())
        } else {
            format!("{}<match>", indent)
        };

        let mut lines = vec![open_tag];
        for test in &self.tests {
            lines.push(test.render(&inner_ctx));
        }
        for edit in &self.edits {
            lines.push(edit.render(&inner_ctx));
        }
        lines.push(format!("{}</match>", indent));
        lines.join("\n")
    }
}
