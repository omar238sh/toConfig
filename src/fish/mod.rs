use crate::core::{Config, RenderContext};

// ── Utility ───────────────────────────────────────────────────────────────────

/// Quote a fish value with single quotes when needed.
/// Bare values (alphanumeric + safe symbols) are emitted without quotes.
fn quote_fish_value(s: &str) -> String {
    if s.chars()
        .all(|c| c.is_alphanumeric() || matches!(c, '_' | '-' | '.' | '/' | '~' | ':'))
    {
        s.to_string()
    } else {
        format!("'{}'", s.replace('\\', "\\\\").replace('\'', "\\'"))
    }
}

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

// ── Alias ─────────────────────────────────────────────────────────────────────

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

// ── Abbreviation ──────────────────────────────────────────────────────────────

/// Where in the command line an abbreviation expands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbbrPosition {
    /// Only when typed as a command (the default).
    Command,
    /// Anywhere in the command line.
    Anywhere,
}

/// An `abbr --add` abbreviation.
#[derive(Debug, Clone)]
pub struct FishAbbr {
    pub name: String,
    pub expansion: Option<String>,
    pub position: AbbrPosition,
    pub regex: Option<String>,
    /// Name of a fish function that returns the expansion.
    pub function: Option<String>,
    /// Place the cursor at `%` in the expansion.
    pub set_cursor: bool,
    pub doc: Option<String>,
}

impl FishAbbr {
    pub fn new(name: &str, expansion: &str) -> Self {
        Self {
            name: name.to_string(),
            expansion: Some(expansion.to_string()),
            position: AbbrPosition::Command,
            regex: None,
            function: None,
            set_cursor: false,
            doc: None,
        }
    }

    /// Abbreviation whose expansion is computed by a function.
    pub fn with_function(name: &str, function: &str) -> Self {
        Self {
            name: name.to_string(),
            expansion: None,
            position: AbbrPosition::Command,
            regex: None,
            function: Some(function.to_string()),
            set_cursor: false,
            doc: None,
        }
    }

    pub fn position(mut self, p: AbbrPosition) -> Self {
        self.position = p;
        self
    }

    /// Expand the abbreviation anywhere in the command line.
    pub fn anywhere(self) -> Self {
        self.position(AbbrPosition::Anywhere)
    }

    pub fn regex(mut self, r: &str) -> Self {
        self.regex = Some(r.to_string());
        self
    }

    /// Place the cursor at `%` in the expansion string.
    pub fn set_cursor(mut self) -> Self {
        self.set_cursor = true;
        self
    }

    pub fn doc(mut self, d: &str) -> Self {
        self.doc = Some(d.to_string());
        self
    }
}

impl Config for FishAbbr {
    fn render(&self, ctx: &RenderContext) -> String {
        let mut parts = vec!["abbr".to_string(), "--add".to_string()];

        if self.position == AbbrPosition::Anywhere {
            parts.push("--position anywhere".to_string());
        }
        if let Some(ref r) = self.regex {
            parts.push(format!("--regex {}", quote_fish_value(r)));
        }
        if let Some(ref f) = self.function {
            parts.push(format!("--function {}", f));
        }
        if self.set_cursor {
            parts.push("--set-cursor".to_string());
        }

        parts.push(quote_fish_value(&self.name));
        if let Some(ref e) = self.expansion {
            parts.push(quote_fish_value(e));
        }

        format!("{}{}", ctx.indent(), parts.join(" "))
    }

    fn doc_comment(&self) -> Option<&str> {
        self.doc.as_deref()
    }
}

// ── Key Binding ───────────────────────────────────────────────────────────────

/// The bind mode for a fish key binding.
#[derive(Debug, Clone)]
pub enum BindMode {
    Default,
    Insert,
    Visual,
    Custom(String),
}

impl BindMode {
    pub fn as_str(&self) -> &str {
        match self {
            BindMode::Default => "default",
            BindMode::Insert => "insert",
            BindMode::Visual => "visual",
            BindMode::Custom(s) => s.as_str(),
        }
    }
}

/// A `bind` key-binding statement.
#[derive(Debug, Clone)]
pub struct FishBind {
    pub key: String,
    pub command: String,
    pub mode: Option<BindMode>,
    /// Transition to a different mode after the binding fires.
    pub sets_mode: Option<String>,
    pub silent: bool,
    pub doc: Option<String>,
}

impl FishBind {
    pub fn new(key: &str, command: &str) -> Self {
        Self {
            key: key.to_string(),
            command: command.to_string(),
            mode: None,
            sets_mode: None,
            silent: false,
            doc: None,
        }
    }

    pub fn mode(mut self, m: BindMode) -> Self {
        self.mode = Some(m);
        self
    }

    pub fn sets_mode(mut self, m: &str) -> Self {
        self.sets_mode = Some(m.to_string());
        self
    }

    pub fn silent(mut self) -> Self {
        self.silent = true;
        self
    }

    pub fn doc(mut self, d: &str) -> Self {
        self.doc = Some(d.to_string());
        self
    }
}

impl Config for FishBind {
    fn render(&self, ctx: &RenderContext) -> String {
        let mut parts = vec!["bind".to_string()];
        if let Some(ref m) = self.mode {
            parts.push(format!("--mode {}", m.as_str()));
        }
        if let Some(ref m) = self.sets_mode {
            parts.push(format!("--sets-mode {}", m));
        }
        if self.silent {
            parts.push("--silent".to_string());
        }
        parts.push(self.key.clone());
        parts.push(self.command.clone());
        format!("{}{}", ctx.indent(), parts.join(" "))
    }

    fn doc_comment(&self) -> Option<&str> {
        self.doc.as_deref()
    }
}

// ── Add Path ──────────────────────────────────────────────────────────────────

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

// ── Color ─────────────────────────────────────────────────────────────────────

/// Identifies a `fish_color_*` / `fish_pager_color_*` variable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FishColorVar {
    Normal,
    Command,
    Keyword,
    Quote,
    Redirection,
    EndOfCommand,
    Error,
    Param,
    Option,
    Comment,
    Selection,
    Operator,
    Escape,
    Autosuggestion,
    MatchingParens,
    HistoryCurrent,
    SearchMatch,
    ValidPath,
    Cancel,
    // Pager colors
    PagerProgress,
    PagerBackground,
    PagerSecondaryBackground,
    PagerSelectedBackground,
    PagerPrefix,
    PagerCompletion,
    PagerDescription,
    PagerSecondaryPrefix,
    PagerSecondaryCompletion,
    PagerSecondaryDescription,
    PagerSelectedPrefix,
    PagerSelectedCompletion,
    PagerSelectedDescription,
}

impl FishColorVar {
    pub fn var_name(self) -> &'static str {
        match self {
            FishColorVar::Normal => "fish_color_normal",
            FishColorVar::Command => "fish_color_command",
            FishColorVar::Keyword => "fish_color_keyword",
            FishColorVar::Quote => "fish_color_quote",
            FishColorVar::Redirection => "fish_color_redirection",
            FishColorVar::EndOfCommand => "fish_color_end",
            FishColorVar::Error => "fish_color_error",
            FishColorVar::Param => "fish_color_param",
            FishColorVar::Option => "fish_color_option",
            FishColorVar::Comment => "fish_color_comment",
            FishColorVar::Selection => "fish_color_selection",
            FishColorVar::Operator => "fish_color_operator",
            FishColorVar::Escape => "fish_color_escape",
            FishColorVar::Autosuggestion => "fish_color_autosuggestion",
            FishColorVar::MatchingParens => "fish_color_matching_paren",
            FishColorVar::HistoryCurrent => "fish_color_history_current",
            FishColorVar::SearchMatch => "fish_color_search_match",
            FishColorVar::ValidPath => "fish_color_valid_path",
            FishColorVar::Cancel => "fish_color_cancel",
            FishColorVar::PagerProgress => "fish_pager_color_progress",
            FishColorVar::PagerBackground => "fish_pager_color_background",
            FishColorVar::PagerSecondaryBackground => "fish_pager_color_secondary_background",
            FishColorVar::PagerSelectedBackground => "fish_pager_color_selected_background",
            FishColorVar::PagerPrefix => "fish_pager_color_prefix",
            FishColorVar::PagerCompletion => "fish_pager_color_completion",
            FishColorVar::PagerDescription => "fish_pager_color_description",
            FishColorVar::PagerSecondaryPrefix => "fish_pager_color_secondary_prefix",
            FishColorVar::PagerSecondaryCompletion => "fish_pager_color_secondary_completion",
            FishColorVar::PagerSecondaryDescription => "fish_pager_color_secondary_description",
            FishColorVar::PagerSelectedPrefix => "fish_pager_color_selected_prefix",
            FishColorVar::PagerSelectedCompletion => "fish_pager_color_selected_completion",
            FishColorVar::PagerSelectedDescription => "fish_pager_color_selected_description",
        }
    }
}

/// Sets a single `fish_color_*` or `fish_pager_color_*` variable.
#[derive(Debug, Clone)]
pub struct FishColor {
    pub color_var: FishColorVar,
    /// Color value, e.g. `"brblue"`, `"#af87ff"`, `"normal --bold"`.
    pub value: String,
}

impl FishColor {
    pub fn new(color_var: FishColorVar, value: &str) -> Self {
        Self {
            color_var,
            value: value.to_string(),
        }
    }
}

impl Config for FishColor {
    fn render(&self, ctx: &RenderContext) -> String {
        format!(
            "{}set -g {} {}",
            ctx.indent(),
            self.color_var.var_name(),
            quote_fish_value(&self.value)
        )
    }
}

// ── Completion ────────────────────────────────────────────────────────────────

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

// ── Conditional ───────────────────────────────────────────────────────────────

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

/// A branch inside a `FishIf` block (condition + body).
///
/// Represents an `else if condition` clause. Build one by collecting
/// `Config` nodes and pass them to [`FishIf::else_if`].
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
        self.else_if_branches
            .push(FishElseIf { condition, body: nodes });
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
///
/// The `pattern` may contain globs (e.g. `"*.fish"`). Build arms with
/// [`FishSwitch::case`] rather than constructing this directly.
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

        out.push(format!("{}switch {}", indent, quote_fish_value(&self.value)));
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

// ── Greeting ──────────────────────────────────────────────────────────────────

/// Sets the fish greeting shown at session start.
#[derive(Debug, Clone)]
pub enum FishGreeting {
    /// `set -g fish_greeting "message"` – a plain string.
    Message(String),
    /// Custom `fish_greeting` function with body lines.
    Function(Vec<String>),
    /// `set -g fish_greeting` (empty) – disables the greeting.
    Disabled,
}

impl FishGreeting {
    pub fn message(msg: &str) -> Self {
        FishGreeting::Message(msg.to_string())
    }

    pub fn function(body: &[&str]) -> Self {
        FishGreeting::Function(body.iter().map(|s| s.to_string()).collect())
    }

    pub fn disabled() -> Self {
        FishGreeting::Disabled
    }
}

impl Config for FishGreeting {
    fn render(&self, ctx: &RenderContext) -> String {
        match self {
            FishGreeting::Disabled => format!("{}set -g fish_greeting", ctx.indent()),
            FishGreeting::Message(msg) => {
                format!(
                    "{}set -g fish_greeting {}",
                    ctx.indent(),
                    quote_fish_value(msg)
                )
            }
            FishGreeting::Function(body) => {
                let inner = ctx.deeper();
                let mut out = Vec::new();
                out.push(format!("{}function fish_greeting", ctx.indent()));
                for line in body {
                    out.push(format!("{}{}", inner.indent(), line));
                }
                out.push(format!("{}end", ctx.indent()));
                out.join("\n")
            }
        }
    }
}

// ── Prompt ────────────────────────────────────────────────────────────────────

/// The `fish_prompt` function (left prompt).
#[derive(Debug, Clone)]
pub struct FishPrompt {
    pub body: Vec<String>,
    pub doc: Option<String>,
}

impl FishPrompt {
    pub fn new(body: &[&str]) -> Self {
        Self {
            body: body.iter().map(|s| s.to_string()).collect(),
            doc: None,
        }
    }

    pub fn doc(mut self, d: &str) -> Self {
        self.doc = Some(d.to_string());
        self
    }
}

impl Config for FishPrompt {
    fn render(&self, ctx: &RenderContext) -> String {
        let indent = ctx.indent();
        let inner = ctx.deeper();
        let mut out = Vec::new();
        out.push(format!("{}function fish_prompt", indent));
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

/// The `fish_right_prompt` function (right side of prompt line).
#[derive(Debug, Clone)]
pub struct FishRightPrompt {
    pub body: Vec<String>,
    pub doc: Option<String>,
}

impl FishRightPrompt {
    pub fn new(body: &[&str]) -> Self {
        Self {
            body: body.iter().map(|s| s.to_string()).collect(),
            doc: None,
        }
    }

    pub fn doc(mut self, d: &str) -> Self {
        self.doc = Some(d.to_string());
        self
    }
}

impl Config for FishRightPrompt {
    fn render(&self, ctx: &RenderContext) -> String {
        let indent = ctx.indent();
        let inner = ctx.deeper();
        let mut out = Vec::new();
        out.push(format!("{}function fish_right_prompt", indent));
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

/// The `fish_mode_prompt` function (shows vi-mode indicator).
#[derive(Debug, Clone)]
pub struct FishModePrompt {
    pub body: Vec<String>,
}

impl FishModePrompt {
    pub fn new(body: &[&str]) -> Self {
        Self {
            body: body.iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl Config for FishModePrompt {
    fn render(&self, ctx: &RenderContext) -> String {
        let indent = ctx.indent();
        let inner = ctx.deeper();
        let mut out = Vec::new();
        out.push(format!("{}function fish_mode_prompt", indent));
        for line in &self.body {
            out.push(format!("{}{}", inner.indent(), line));
        }
        out.push(format!("{}end", indent));
        out.join("\n")
    }
}

// ── Source ────────────────────────────────────────────────────────────────────

/// A `source path` statement.
#[derive(Debug, Clone)]
pub struct FishSource {
    pub path: String,
}

impl FishSource {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }
}

impl Config for FishSource {
    fn render(&self, ctx: &RenderContext) -> String {
        format!("{}source {}", ctx.indent(), quote_fish_value(&self.path))
    }
}

// ── Plugin (Fisher) ───────────────────────────────────────────────────────────

/// A Fisher plugin installation entry (`fisher install owner/repo`).
#[derive(Debug, Clone)]
pub struct FishPlugin {
    pub repo: String,
    pub doc: Option<String>,
}

impl FishPlugin {
    pub fn new(repo: &str) -> Self {
        Self {
            repo: repo.to_string(),
            doc: None,
        }
    }

    pub fn doc(mut self, d: &str) -> Self {
        self.doc = Some(d.to_string());
        self
    }
}

impl Config for FishPlugin {
    fn render(&self, ctx: &RenderContext) -> String {
        format!("{}fisher install {}", ctx.indent(), self.repo)
    }

    fn doc_comment(&self) -> Option<&str> {
        self.doc.as_deref()
    }
}

// ── Raw Line ──────────────────────────────────────────────────────────────────

/// An escape hatch: emit an arbitrary fish shell line verbatim.
#[derive(Debug, Clone)]
pub struct FishRawLine {
    pub code: String,
    pub doc: Option<String>,
}

impl FishRawLine {
    pub fn new(code: &str) -> Self {
        Self {
            code: code.to_string(),
            doc: None,
        }
    }

    pub fn doc(mut self, d: &str) -> Self {
        self.doc = Some(d.to_string());
        self
    }
}

impl Config for FishRawLine {
    fn render(&self, ctx: &RenderContext) -> String {
        format!("{}{}", ctx.indent(), self.code)
    }

    fn doc_comment(&self) -> Option<&str> {
        self.doc.as_deref()
    }
}

// ── Config Tree ───────────────────────────────────────────────────────────────

/// A composite node that holds an ordered list of fish config nodes.
pub struct FishConfigTree {
    pub nodes: Vec<Box<dyn Config>>,
    pub comment: Option<String>,
}

impl Default for FishConfigTree {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            comment: None,
        }
    }
}

impl FishConfigTree {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_comment(mut self, c: &str) -> Self {
        self.comment = Some(c.to_string());
        self
    }

    pub fn add<C: Config + 'static>(mut self, node: C) -> Self {
        self.nodes.push(Box::new(node));
        self
    }

    pub fn push_node<C: Config + 'static>(&mut self, node: C) -> &mut Self {
        self.nodes.push(Box::new(node));
        self
    }

    /// Validate all children before rendering.
    pub fn validate_all(&self) -> Result<(), Vec<String>> {
        let errors: Vec<String> = self
            .nodes
            .iter()
            .filter_map(|n| n.validate().err())
            .collect();
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl Config for FishConfigTree {
    fn render(&self, ctx: &RenderContext) -> String {
        let mut parts = Vec::new();
        if let Some(ref c) = self.comment {
            parts.push(format!("{}# {}", ctx.indent(), c));
        }
        for node in &self.nodes {
            if ctx.emit_doc_comments {
                if let Some(doc) = node.doc_comment() {
                    parts.push(format!("{}# {}", ctx.indent(), doc));
                }
            }
            parts.push(node.render(ctx));
        }
        parts.join("\n")
    }
}
