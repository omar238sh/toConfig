use super::core::{HelixConfig, HelixRenderContext, toml_bool, toml_str, toml_str_array};

// ── Helper ────────────────────────────────────────────────────────────────────

fn push_kv(lines: &mut Vec<String>, key: &str, value: String) {
    lines.push(format!("{} = {}", key, value));
}

// ── Enums ─────────────────────────────────────────────────────────────────────

/// Line-number display style.
#[derive(Debug, Clone, PartialEq)]
pub enum LineNumber {
    /// Absolute line numbers.
    Absolute,
    /// Line numbers relative to the cursor.
    Relative,
    /// Relative numbers everywhere except the cursor line, which shows absolute.
    RelativePlus,
}

impl LineNumber {
    pub fn as_str(&self) -> &str {
        match self {
            LineNumber::Absolute => "absolute",
            LineNumber::Relative => "relative",
            LineNumber::RelativePlus => "relative-plus",
        }
    }
}

/// Controls when the bufferline is shown.
#[derive(Debug, Clone, PartialEq)]
pub enum Bufferline {
    /// Never show the bufferline.
    Never,
    /// Always show the bufferline.
    Always,
    /// Only show it when more than one buffer is open.
    Multiple,
}

impl Bufferline {
    pub fn as_str(&self) -> &str {
        match self {
            Bufferline::Never => "never",
            Bufferline::Always => "always",
            Bufferline::Multiple => "multiple",
        }
    }
}

/// Cursor shape for a given mode.
#[derive(Debug, Clone, PartialEq)]
pub enum CursorKind {
    Block,
    Bar,
    Underline,
    Hidden,
}

impl CursorKind {
    pub fn as_str(&self) -> &str {
        match self {
            CursorKind::Block => "block",
            CursorKind::Bar => "bar",
            CursorKind::Underline => "underline",
            CursorKind::Hidden => "hidden",
        }
    }
}

// ── [editor.cursor-shape] ─────────────────────────────────────────────────────

/// Cursor shapes for each editor mode (`[editor.cursor-shape]`).
///
/// # Example
/// ```
/// # use toconfig::helix::HelixConfig;
/// use toconfig::helix::editor::{CursorShapeSection, CursorKind};
/// let sec = CursorShapeSection::new()
///     .normal(CursorKind::Block)
///     .insert(CursorKind::Bar)
///     .select(CursorKind::Underline);
/// let out = sec.generate();
/// assert!(out.contains("[editor.cursor-shape]"));
/// assert!(out.contains("insert = \"bar\""));
/// ```
#[derive(Default)]
pub struct CursorShapeSection {
    pub normal: Option<CursorKind>,
    pub insert: Option<CursorKind>,
    pub select: Option<CursorKind>,
}

impl CursorShapeSection {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn normal(mut self, k: CursorKind) -> Self {
        self.normal = Some(k);
        self
    }

    pub fn insert(mut self, k: CursorKind) -> Self {
        self.insert = Some(k);
        self
    }

    pub fn select(mut self, k: CursorKind) -> Self {
        self.select = Some(k);
        self
    }
}

impl HelixConfig for CursorShapeSection {
    fn render(&self, _ctx: &HelixRenderContext) -> String {
        let mut lines = vec!["[editor.cursor-shape]".to_string()];
        if let Some(ref k) = self.normal {
            push_kv(&mut lines, "normal", toml_str(k.as_str()));
        }
        if let Some(ref k) = self.insert {
            push_kv(&mut lines, "insert", toml_str(k.as_str()));
        }
        if let Some(ref k) = self.select {
            push_kv(&mut lines, "select", toml_str(k.as_str()));
        }
        if lines.len() == 1 {
            return String::new();
        }
        lines.join("\n")
    }
}

// ── [editor.statusline] ───────────────────────────────────────────────────────

/// Elements that can appear in the Helix statusline.
#[derive(Debug, Clone, PartialEq)]
pub enum StatuslineElement {
    Mode,
    Spinner,
    FileName,
    FileBaseName,
    FileModificationIndicator,
    ReadOnlyIndicator,
    FileEncoding,
    FileLineEnding,
    FileType,
    Diagnostics,
    WorkspaceDiagnostics,
    Selections,
    PrimarySelectionLength,
    Position,
    PositionPercentage,
    TotalLineNumbers,
    Separator,
    Spacer,
    VersionControl,
    Register,
}

impl StatuslineElement {
    pub fn as_str(&self) -> &str {
        match self {
            StatuslineElement::Mode => "mode",
            StatuslineElement::Spinner => "spinner",
            StatuslineElement::FileName => "file-name",
            StatuslineElement::FileBaseName => "file-base-name",
            StatuslineElement::FileModificationIndicator => "file-modification-indicator",
            StatuslineElement::ReadOnlyIndicator => "read-only-indicator",
            StatuslineElement::FileEncoding => "file-encoding",
            StatuslineElement::FileLineEnding => "file-line-ending",
            StatuslineElement::FileType => "file-type",
            StatuslineElement::Diagnostics => "diagnostics",
            StatuslineElement::WorkspaceDiagnostics => "workspace-diagnostics",
            StatuslineElement::Selections => "selections",
            StatuslineElement::PrimarySelectionLength => "primary-selection-length",
            StatuslineElement::Position => "position",
            StatuslineElement::PositionPercentage => "position-percentage",
            StatuslineElement::TotalLineNumbers => "total-line-numbers",
            StatuslineElement::Separator => "separator",
            StatuslineElement::Spacer => "spacer",
            StatuslineElement::VersionControl => "version-control",
            StatuslineElement::Register => "register",
        }
    }
}

/// Statusline configuration (`[editor.statusline]`).
///
/// # Example
/// ```
/// # use toconfig::helix::HelixConfig;
/// use toconfig::helix::editor::{StatuslineSection, StatuslineElement};
/// let sec = StatuslineSection::new()
///     .left(vec![StatuslineElement::Mode, StatuslineElement::Spinner, StatuslineElement::FileName])
///     .right(vec![StatuslineElement::Diagnostics, StatuslineElement::Position, StatuslineElement::FileType])
///     .separator("│");
/// let out = sec.generate();
/// assert!(out.contains("[editor.statusline]"));
/// assert!(out.contains("separator = \"│\""));
/// ```
#[derive(Default)]
pub struct StatuslineSection {
    pub left: Option<Vec<StatuslineElement>>,
    pub center: Option<Vec<StatuslineElement>>,
    pub right: Option<Vec<StatuslineElement>>,
    pub separator: Option<String>,
    /// Label shown in the statusline when in Normal mode.
    pub mode_normal: Option<String>,
    /// Label shown in the statusline when in Insert mode.
    pub mode_insert: Option<String>,
    /// Label shown in the statusline when in Select mode.
    pub mode_select: Option<String>,
}

impl StatuslineSection {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn left(mut self, elements: Vec<StatuslineElement>) -> Self {
        self.left = Some(elements);
        self
    }

    pub fn center(mut self, elements: Vec<StatuslineElement>) -> Self {
        self.center = Some(elements);
        self
    }

    pub fn right(mut self, elements: Vec<StatuslineElement>) -> Self {
        self.right = Some(elements);
        self
    }

    pub fn separator(mut self, sep: impl Into<String>) -> Self {
        self.separator = Some(sep.into());
        self
    }

    pub fn mode_normal(mut self, label: impl Into<String>) -> Self {
        self.mode_normal = Some(label.into());
        self
    }

    pub fn mode_insert(mut self, label: impl Into<String>) -> Self {
        self.mode_insert = Some(label.into());
        self
    }

    pub fn mode_select(mut self, label: impl Into<String>) -> Self {
        self.mode_select = Some(label.into());
        self
    }
}

fn render_elements(elements: &[StatuslineElement]) -> String {
    let strs: Vec<String> = elements.iter().map(|e| e.as_str().to_string()).collect();
    toml_str_array(&strs)
}

impl HelixConfig for StatuslineSection {
    fn render(&self, _ctx: &HelixRenderContext) -> String {
        let mut lines = vec!["[editor.statusline]".to_string()];
        if let Some(ref e) = self.left {
            push_kv(&mut lines, "left", render_elements(e));
        }
        if let Some(ref e) = self.center {
            push_kv(&mut lines, "center", render_elements(e));
        }
        if let Some(ref e) = self.right {
            push_kv(&mut lines, "right", render_elements(e));
        }
        if let Some(ref s) = self.separator {
            push_kv(&mut lines, "separator", toml_str(s));
        }
        if let Some(ref s) = self.mode_normal {
            push_kv(&mut lines, "mode.normal", toml_str(s));
        }
        if let Some(ref s) = self.mode_insert {
            push_kv(&mut lines, "mode.insert", toml_str(s));
        }
        if let Some(ref s) = self.mode_select {
            push_kv(&mut lines, "mode.select", toml_str(s));
        }
        if lines.len() == 1 {
            return String::new();
        }
        lines.join("\n")
    }
}

// ── [editor.lsp] ──────────────────────────────────────────────────────────────

/// LSP integration settings (`[editor.lsp]`).
///
/// # Example
/// ```
/// # use toconfig::helix::HelixConfig;
/// use toconfig::helix::editor::LspSection;
/// let sec = LspSection::new()
///     .display_inlay_hints(true)
///     .display_messages(true);
/// let out = sec.generate();
/// assert!(out.contains("[editor.lsp]"));
/// assert!(out.contains("display-inlay-hints = true"));
/// ```
#[derive(Default)]
pub struct LspSection {
    pub enable: Option<bool>,
    pub display_messages: Option<bool>,
    pub auto_signature_help: Option<bool>,
    pub display_inlay_hints: Option<bool>,
    pub display_signature_help_docs: Option<bool>,
    pub snippets: Option<bool>,
    pub goto_reference_include_declaration: Option<bool>,
}

impl LspSection {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enable(mut self, v: bool) -> Self {
        self.enable = Some(v);
        self
    }

    pub fn display_messages(mut self, v: bool) -> Self {
        self.display_messages = Some(v);
        self
    }

    pub fn auto_signature_help(mut self, v: bool) -> Self {
        self.auto_signature_help = Some(v);
        self
    }

    pub fn display_inlay_hints(mut self, v: bool) -> Self {
        self.display_inlay_hints = Some(v);
        self
    }

    pub fn display_signature_help_docs(mut self, v: bool) -> Self {
        self.display_signature_help_docs = Some(v);
        self
    }

    pub fn snippets(mut self, v: bool) -> Self {
        self.snippets = Some(v);
        self
    }

    pub fn goto_reference_include_declaration(mut self, v: bool) -> Self {
        self.goto_reference_include_declaration = Some(v);
        self
    }
}

impl HelixConfig for LspSection {
    fn render(&self, _ctx: &HelixRenderContext) -> String {
        let mut lines = vec!["[editor.lsp]".to_string()];
        if let Some(v) = self.enable {
            push_kv(&mut lines, "enable", toml_bool(v).to_string());
        }
        if let Some(v) = self.display_messages {
            push_kv(&mut lines, "display-messages", toml_bool(v).to_string());
        }
        if let Some(v) = self.auto_signature_help {
            push_kv(
                &mut lines,
                "auto-signature-help",
                toml_bool(v).to_string(),
            );
        }
        if let Some(v) = self.display_inlay_hints {
            push_kv(
                &mut lines,
                "display-inlay-hints",
                toml_bool(v).to_string(),
            );
        }
        if let Some(v) = self.display_signature_help_docs {
            push_kv(
                &mut lines,
                "display-signature-help-docs",
                toml_bool(v).to_string(),
            );
        }
        if let Some(v) = self.snippets {
            push_kv(&mut lines, "snippets", toml_bool(v).to_string());
        }
        if let Some(v) = self.goto_reference_include_declaration {
            push_kv(
                &mut lines,
                "goto-reference-include-declaration",
                toml_bool(v).to_string(),
            );
        }
        if lines.len() == 1 {
            return String::new();
        }
        lines.join("\n")
    }
}

// ── [editor.file-picker] ──────────────────────────────────────────────────────

/// File-picker settings (`[editor.file-picker]`).
#[derive(Default)]
pub struct FilePickerSection {
    pub hidden: Option<bool>,
    pub follow_symlinks: Option<bool>,
    pub deduplicate_links: Option<bool>,
    pub parents: Option<bool>,
    pub ignore: Option<bool>,
    pub git_ignore: Option<bool>,
    pub git_global: Option<bool>,
    pub git_exclude: Option<bool>,
    pub max_depth: Option<u32>,
}

impl FilePickerSection {
    pub fn new() -> Self {
        Self::default()
    }

    /// Show hidden files (`true` = show, `false` = hide).
    pub fn hidden(mut self, v: bool) -> Self {
        self.hidden = Some(v);
        self
    }

    pub fn follow_symlinks(mut self, v: bool) -> Self {
        self.follow_symlinks = Some(v);
        self
    }

    pub fn deduplicate_links(mut self, v: bool) -> Self {
        self.deduplicate_links = Some(v);
        self
    }

    pub fn parents(mut self, v: bool) -> Self {
        self.parents = Some(v);
        self
    }

    pub fn ignore(mut self, v: bool) -> Self {
        self.ignore = Some(v);
        self
    }

    pub fn git_ignore(mut self, v: bool) -> Self {
        self.git_ignore = Some(v);
        self
    }

    pub fn git_global(mut self, v: bool) -> Self {
        self.git_global = Some(v);
        self
    }

    pub fn git_exclude(mut self, v: bool) -> Self {
        self.git_exclude = Some(v);
        self
    }

    pub fn max_depth(mut self, d: u32) -> Self {
        self.max_depth = Some(d);
        self
    }
}

impl HelixConfig for FilePickerSection {
    fn render(&self, _ctx: &HelixRenderContext) -> String {
        let mut lines = vec!["[editor.file-picker]".to_string()];
        if let Some(v) = self.hidden {
            push_kv(&mut lines, "hidden", toml_bool(v).to_string());
        }
        if let Some(v) = self.follow_symlinks {
            push_kv(&mut lines, "follow-symlinks", toml_bool(v).to_string());
        }
        if let Some(v) = self.deduplicate_links {
            push_kv(&mut lines, "deduplicate-links", toml_bool(v).to_string());
        }
        if let Some(v) = self.parents {
            push_kv(&mut lines, "parents", toml_bool(v).to_string());
        }
        if let Some(v) = self.ignore {
            push_kv(&mut lines, "ignore", toml_bool(v).to_string());
        }
        if let Some(v) = self.git_ignore {
            push_kv(&mut lines, "git-ignore", toml_bool(v).to_string());
        }
        if let Some(v) = self.git_global {
            push_kv(&mut lines, "git-global", toml_bool(v).to_string());
        }
        if let Some(v) = self.git_exclude {
            push_kv(&mut lines, "git-exclude", toml_bool(v).to_string());
        }
        if let Some(v) = self.max_depth {
            push_kv(&mut lines, "max-depth", v.to_string());
        }
        if lines.len() == 1 {
            return String::new();
        }
        lines.join("\n")
    }
}

// ── [editor.search] ───────────────────────────────────────────────────────────

/// Search settings (`[editor.search]`).
#[derive(Default)]
pub struct SearchSection {
    pub smart_case: Option<bool>,
    pub wrap_around: Option<bool>,
}

impl SearchSection {
    pub fn new() -> Self {
        Self::default()
    }

    /// Use smart-case matching (insensitive unless the query contains uppercase).
    pub fn smart_case(mut self, v: bool) -> Self {
        self.smart_case = Some(v);
        self
    }

    /// Wrap around end/start of file when searching.
    pub fn wrap_around(mut self, v: bool) -> Self {
        self.wrap_around = Some(v);
        self
    }
}

impl HelixConfig for SearchSection {
    fn render(&self, _ctx: &HelixRenderContext) -> String {
        let mut lines = vec!["[editor.search]".to_string()];
        if let Some(v) = self.smart_case {
            push_kv(&mut lines, "smart-case", toml_bool(v).to_string());
        }
        if let Some(v) = self.wrap_around {
            push_kv(&mut lines, "wrap-around", toml_bool(v).to_string());
        }
        if lines.len() == 1 {
            return String::new();
        }
        lines.join("\n")
    }
}

// ── [editor.indent-guides] ────────────────────────────────────────────────────

/// Indent-guide rendering settings (`[editor.indent-guides]`).
///
/// # Example
/// ```
/// # use toconfig::helix::HelixConfig;
/// use toconfig::helix::editor::IndentGuidesSection;
/// let sec = IndentGuidesSection::new().render(true).character("╎").skip_levels(1);
/// let out = sec.generate();
/// assert!(out.contains("[editor.indent-guides]"));
/// assert!(out.contains("character = \"╎\""));
/// ```
#[derive(Default)]
pub struct IndentGuidesSection {
    pub render: Option<bool>,
    pub character: Option<String>,
    pub skip_levels: Option<u32>,
}

impl IndentGuidesSection {
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable or disable indent guides.
    pub fn render(mut self, v: bool) -> Self {
        self.render = Some(v);
        self
    }

    /// The character used to draw indent guides (e.g. `"│"`, `"╎"`, `"┆"`).
    pub fn character(mut self, c: impl Into<String>) -> Self {
        self.character = Some(c.into());
        self
    }

    /// Number of indent levels to skip before drawing guides.
    pub fn skip_levels(mut self, n: u32) -> Self {
        self.skip_levels = Some(n);
        self
    }
}

impl HelixConfig for IndentGuidesSection {
    fn render(&self, _ctx: &HelixRenderContext) -> String {
        let mut lines = vec!["[editor.indent-guides]".to_string()];
        if let Some(v) = self.render {
            push_kv(&mut lines, "render", toml_bool(v).to_string());
        }
        if let Some(ref c) = self.character {
            push_kv(&mut lines, "character", toml_str(c));
        }
        if let Some(n) = self.skip_levels {
            push_kv(&mut lines, "skip-levels", n.to_string());
        }
        if lines.len() == 1 {
            return String::new();
        }
        lines.join("\n")
    }
}

// ── [editor.soft-wrap] ────────────────────────────────────────────────────────

/// Soft-wrap settings (`[editor.soft-wrap]`).
#[derive(Default)]
pub struct SoftWrapSection {
    pub enable: Option<bool>,
    pub max_wrap: Option<u32>,
    pub max_indent_retain: Option<u32>,
    pub wrap_indicator: Option<String>,
    pub wrap_at_text_width: Option<bool>,
}

impl SoftWrapSection {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enable(mut self, v: bool) -> Self {
        self.enable = Some(v);
        self
    }

    /// Maximum number of characters a line can be wrapped by (0 = no limit).
    pub fn max_wrap(mut self, n: u32) -> Self {
        self.max_wrap = Some(n);
        self
    }

    /// Maximum number of indent levels to retain when wrapping.
    pub fn max_indent_retain(mut self, n: u32) -> Self {
        self.max_indent_retain = Some(n);
        self
    }

    /// String prepended to soft-wrapped continuation lines (e.g. `"↪ "`).
    pub fn wrap_indicator(mut self, s: impl Into<String>) -> Self {
        self.wrap_indicator = Some(s.into());
        self
    }

    /// Wrap at `text-width` rather than the window width.
    pub fn wrap_at_text_width(mut self, v: bool) -> Self {
        self.wrap_at_text_width = Some(v);
        self
    }
}

impl HelixConfig for SoftWrapSection {
    fn render(&self, _ctx: &HelixRenderContext) -> String {
        let mut lines = vec!["[editor.soft-wrap]".to_string()];
        if let Some(v) = self.enable {
            push_kv(&mut lines, "enable", toml_bool(v).to_string());
        }
        if let Some(n) = self.max_wrap {
            push_kv(&mut lines, "max-wrap", n.to_string());
        }
        if let Some(n) = self.max_indent_retain {
            push_kv(&mut lines, "max-indent-retain", n.to_string());
        }
        if let Some(ref s) = self.wrap_indicator {
            push_kv(&mut lines, "wrap-indicator", toml_str(s));
        }
        if let Some(v) = self.wrap_at_text_width {
            push_kv(&mut lines, "wrap-at-text-width", toml_bool(v).to_string());
        }
        if lines.len() == 1 {
            return String::new();
        }
        lines.join("\n")
    }
}

// ── [editor] (main) ───────────────────────────────────────────────────────────

/// The main `[editor]` configuration block.
///
/// Contains all top-level editor settings as well as optional sub-sections
/// (`cursor-shape`, `statusline`, `lsp`, `file-picker`, `search`,
/// `indent-guides`, `soft-wrap`).
///
/// # Example
/// ```
/// # use toconfig::helix::HelixConfig;
/// use toconfig::helix::editor::{EditorSection, LineNumber, Bufferline,
///     CursorShapeSection, CursorKind, StatuslineSection, StatuslineElement,
///     LspSection, SearchSection, IndentGuidesSection, SoftWrapSection};
///
/// let editor = EditorSection::new()
///     .line_number("relative")
///     .mouse(false)
///     .scrolloff(5)
///     .color_modes(true)
///     .bufferline(Bufferline::Always)
///     .cursor_shape(
///         CursorShapeSection::new()
///             .normal(CursorKind::Block)
///             .insert(CursorKind::Bar)
///             .select(CursorKind::Underline),
///     )
///     .statusline(
///         StatuslineSection::new()
///             .left(vec![StatuslineElement::Mode, StatuslineElement::Spinner])
///             .right(vec![StatuslineElement::Position, StatuslineElement::FileType]),
///     )
///     .lsp(LspSection::new().display_inlay_hints(true))
///     .search(SearchSection::new().smart_case(true).wrap_around(true))
///     .indent_guides(IndentGuidesSection::new().render(true).character("╎"))
///     .soft_wrap(SoftWrapSection::new().enable(false));
///
/// let out = editor.generate();
/// assert!(out.contains("[editor]"));
/// assert!(out.contains("mouse = false"));
/// assert!(out.contains("[editor.cursor-shape]"));
/// assert!(out.contains("[editor.statusline]"));
/// ```
#[derive(Default)]
pub struct EditorSection {
    // ── display ──────────────────────────────────────────────────────────────
    pub line_number: Option<String>,
    pub cursorline: Option<bool>,
    pub cursorcolumn: Option<bool>,
    pub color_modes: Option<bool>,
    pub bufferline: Option<Bufferline>,
    pub true_color: Option<bool>,
    pub undercurl: Option<bool>,
    pub rulers: Option<Vec<u32>>,
    // ── behaviour ────────────────────────────────────────────────────────────
    pub mouse: Option<bool>,
    pub middle_click_paste: Option<bool>,
    pub scrolloff: Option<u32>,
    pub scroll_lines: Option<u32>,
    pub shell: Option<Vec<String>>,
    pub text_width: Option<u32>,
    pub default_line_ending: Option<String>,
    pub insert_final_newline: Option<bool>,
    pub trim_trailing_whitespace: Option<bool>,
    // ── completion / auto-features ───────────────────────────────────────────
    pub auto_completion: Option<bool>,
    pub auto_format: Option<bool>,
    pub auto_save: Option<bool>,
    pub idle_timeout: Option<u64>,
    pub completion_timeout: Option<u64>,
    pub preview_completion_insert: Option<bool>,
    pub completion_trigger_len: Option<u32>,
    pub completion_replace: Option<bool>,
    pub auto_info: Option<bool>,
    // ── workspace ────────────────────────────────────────────────────────────
    pub workspace_lsp_roots: Option<Vec<String>>,
    // ── sub-sections ─────────────────────────────────────────────────────────
    pub cursor_shape: Option<CursorShapeSection>,
    pub statusline: Option<StatuslineSection>,
    pub lsp: Option<LspSection>,
    pub file_picker: Option<FilePickerSection>,
    pub search: Option<SearchSection>,
    pub indent_guides: Option<IndentGuidesSection>,
    pub soft_wrap: Option<SoftWrapSection>,
}

impl EditorSection {
    pub fn new() -> Self {
        Self::default()
    }

    // ── display setters ──────────────────────────────────────────────────────

    /// Line-number style: `"absolute"`, `"relative"`, or `"relative-plus"`.
    ///
    /// You can also pass a [`LineNumber`] via `.line_number_enum(LineNumber::Relative)`.
    pub fn line_number(mut self, s: impl Into<String>) -> Self {
        self.line_number = Some(s.into());
        self
    }

    /// Line-number style via the typed [`LineNumber`] enum.
    pub fn line_number_enum(mut self, ln: LineNumber) -> Self {
        self.line_number = Some(ln.as_str().to_string());
        self
    }

    pub fn cursorline(mut self, v: bool) -> Self {
        self.cursorline = Some(v);
        self
    }

    pub fn cursorcolumn(mut self, v: bool) -> Self {
        self.cursorcolumn = Some(v);
        self
    }

    pub fn color_modes(mut self, v: bool) -> Self {
        self.color_modes = Some(v);
        self
    }

    pub fn bufferline(mut self, b: Bufferline) -> Self {
        self.bufferline = Some(b);
        self
    }

    pub fn true_color(mut self, v: bool) -> Self {
        self.true_color = Some(v);
        self
    }

    pub fn undercurl(mut self, v: bool) -> Self {
        self.undercurl = Some(v);
        self
    }

    /// Column rulers.  Each value is a column number.
    pub fn rulers(mut self, cols: Vec<u32>) -> Self {
        self.rulers = Some(cols);
        self
    }

    // ── behaviour setters ────────────────────────────────────────────────────

    pub fn mouse(mut self, v: bool) -> Self {
        self.mouse = Some(v);
        self
    }

    pub fn middle_click_paste(mut self, v: bool) -> Self {
        self.middle_click_paste = Some(v);
        self
    }

    /// Minimum number of lines to keep visible around the cursor.
    pub fn scrolloff(mut self, n: u32) -> Self {
        self.scrolloff = Some(n);
        self
    }

    /// Number of lines to scroll per mouse-wheel event.
    pub fn scroll_lines(mut self, n: u32) -> Self {
        self.scroll_lines = Some(n);
        self
    }

    /// Shell command used by `:sh`.  E.g. `vec!["bash", "-c"]`.
    pub fn shell(mut self, args: Vec<impl Into<String>>) -> Self {
        self.shell = Some(args.into_iter().map(|s| s.into()).collect());
        self
    }

    /// Maximum number of columns for hard text wrapping.
    pub fn text_width(mut self, n: u32) -> Self {
        self.text_width = Some(n);
        self
    }

    /// Default line ending: `"lf"`, `"crlf"`, `"cr"`, `"ff"`, `"nel"`, `"native"`.
    pub fn default_line_ending(mut self, s: impl Into<String>) -> Self {
        self.default_line_ending = Some(s.into());
        self
    }

    pub fn insert_final_newline(mut self, v: bool) -> Self {
        self.insert_final_newline = Some(v);
        self
    }

    pub fn trim_trailing_whitespace(mut self, v: bool) -> Self {
        self.trim_trailing_whitespace = Some(v);
        self
    }

    // ── auto-feature setters ─────────────────────────────────────────────────

    pub fn auto_completion(mut self, v: bool) -> Self {
        self.auto_completion = Some(v);
        self
    }

    pub fn auto_format(mut self, v: bool) -> Self {
        self.auto_format = Some(v);
        self
    }

    pub fn auto_save(mut self, v: bool) -> Self {
        self.auto_save = Some(v);
        self
    }

    pub fn idle_timeout(mut self, ms: u64) -> Self {
        self.idle_timeout = Some(ms);
        self
    }

    pub fn completion_timeout(mut self, ms: u64) -> Self {
        self.completion_timeout = Some(ms);
        self
    }

    pub fn preview_completion_insert(mut self, v: bool) -> Self {
        self.preview_completion_insert = Some(v);
        self
    }

    pub fn completion_trigger_len(mut self, n: u32) -> Self {
        self.completion_trigger_len = Some(n);
        self
    }

    pub fn completion_replace(mut self, v: bool) -> Self {
        self.completion_replace = Some(v);
        self
    }

    pub fn auto_info(mut self, v: bool) -> Self {
        self.auto_info = Some(v);
        self
    }

    // ── workspace ────────────────────────────────────────────────────────────

    pub fn workspace_lsp_roots(mut self, roots: Vec<impl Into<String>>) -> Self {
        self.workspace_lsp_roots = Some(roots.into_iter().map(|s| s.into()).collect());
        self
    }

    // ── sub-section setters ──────────────────────────────────────────────────

    pub fn cursor_shape(mut self, s: CursorShapeSection) -> Self {
        self.cursor_shape = Some(s);
        self
    }

    pub fn statusline(mut self, s: StatuslineSection) -> Self {
        self.statusline = Some(s);
        self
    }

    pub fn lsp(mut self, s: LspSection) -> Self {
        self.lsp = Some(s);
        self
    }

    pub fn file_picker(mut self, s: FilePickerSection) -> Self {
        self.file_picker = Some(s);
        self
    }

    pub fn search(mut self, s: SearchSection) -> Self {
        self.search = Some(s);
        self
    }

    pub fn indent_guides(mut self, s: IndentGuidesSection) -> Self {
        self.indent_guides = Some(s);
        self
    }

    pub fn soft_wrap(mut self, s: SoftWrapSection) -> Self {
        self.soft_wrap = Some(s);
        self
    }
}

impl HelixConfig for EditorSection {
    fn render(&self, ctx: &HelixRenderContext) -> String {
        let mut parts: Vec<String> = Vec::new();

        // ── [editor] key-values ───────────────────────────────────────────────
        let mut kv: Vec<String> = vec!["[editor]".to_string()];

        if let Some(ref s) = self.line_number {
            push_kv(&mut kv, "line-number", toml_str(s));
        }
        if let Some(v) = self.cursorline {
            push_kv(&mut kv, "cursorline", toml_bool(v).to_string());
        }
        if let Some(v) = self.cursorcolumn {
            push_kv(&mut kv, "cursorcolumn", toml_bool(v).to_string());
        }
        if let Some(v) = self.color_modes {
            push_kv(&mut kv, "color-modes", toml_bool(v).to_string());
        }
        if let Some(ref b) = self.bufferline {
            push_kv(&mut kv, "bufferline", toml_str(b.as_str()));
        }
        if let Some(v) = self.true_color {
            push_kv(&mut kv, "true-color", toml_bool(v).to_string());
        }
        if let Some(v) = self.undercurl {
            push_kv(&mut kv, "undercurl", toml_bool(v).to_string());
        }
        if let Some(ref cols) = self.rulers {
            let arr: Vec<String> = cols.iter().map(|c| c.to_string()).collect();
            push_kv(&mut kv, "rulers", format!("[{}]", arr.join(", ")));
        }
        if let Some(v) = self.mouse {
            push_kv(&mut kv, "mouse", toml_bool(v).to_string());
        }
        if let Some(v) = self.middle_click_paste {
            push_kv(&mut kv, "middle-click-paste", toml_bool(v).to_string());
        }
        if let Some(n) = self.scrolloff {
            push_kv(&mut kv, "scrolloff", n.to_string());
        }
        if let Some(n) = self.scroll_lines {
            push_kv(&mut kv, "scroll-lines", n.to_string());
        }
        if let Some(ref args) = self.shell {
            push_kv(&mut kv, "shell", toml_str_array(args));
        }
        if let Some(n) = self.text_width {
            push_kv(&mut kv, "text-width", n.to_string());
        }
        if let Some(ref s) = self.default_line_ending {
            push_kv(&mut kv, "default-line-ending", toml_str(s));
        }
        if let Some(v) = self.insert_final_newline {
            push_kv(&mut kv, "insert-final-newline", toml_bool(v).to_string());
        }
        if let Some(v) = self.trim_trailing_whitespace {
            push_kv(
                &mut kv,
                "trim-trailing-whitespace",
                toml_bool(v).to_string(),
            );
        }
        if let Some(v) = self.auto_completion {
            push_kv(&mut kv, "auto-completion", toml_bool(v).to_string());
        }
        if let Some(v) = self.auto_format {
            push_kv(&mut kv, "auto-format", toml_bool(v).to_string());
        }
        if let Some(v) = self.auto_save {
            push_kv(&mut kv, "auto-save", toml_bool(v).to_string());
        }
        if let Some(ms) = self.idle_timeout {
            push_kv(&mut kv, "idle-timeout", ms.to_string());
        }
        if let Some(ms) = self.completion_timeout {
            push_kv(&mut kv, "completion-timeout", ms.to_string());
        }
        if let Some(v) = self.preview_completion_insert {
            push_kv(
                &mut kv,
                "preview-completion-insert",
                toml_bool(v).to_string(),
            );
        }
        if let Some(n) = self.completion_trigger_len {
            push_kv(&mut kv, "completion-trigger-len", n.to_string());
        }
        if let Some(v) = self.completion_replace {
            push_kv(&mut kv, "completion-replace", toml_bool(v).to_string());
        }
        if let Some(v) = self.auto_info {
            push_kv(&mut kv, "auto-info", toml_bool(v).to_string());
        }
        if let Some(ref roots) = self.workspace_lsp_roots {
            push_kv(&mut kv, "workspace-lsp-roots", toml_str_array(roots));
        }

        if kv.len() > 1 {
            parts.push(kv.join("\n"));
        } else {
            // Always emit the [editor] header even if no keys are set
            parts.push("[editor]".to_string());
        }

        // ── sub-sections (each rendered as its own TOML table header) ─────────
        if let Some(ref s) = self.cursor_shape {
            let rendered = s.render(ctx);
            if !rendered.is_empty() {
                parts.push(rendered);
            }
        }
        if let Some(ref s) = self.statusline {
            let rendered = s.render(ctx);
            if !rendered.is_empty() {
                parts.push(rendered);
            }
        }
        if let Some(ref s) = self.lsp {
            let rendered = s.render(ctx);
            if !rendered.is_empty() {
                parts.push(rendered);
            }
        }
        if let Some(ref s) = self.file_picker {
            let rendered = s.render(ctx);
            if !rendered.is_empty() {
                parts.push(rendered);
            }
        }
        if let Some(ref s) = self.search {
            let rendered = s.render(ctx);
            if !rendered.is_empty() {
                parts.push(rendered);
            }
        }
        if let Some(ref s) = self.indent_guides {
            let rendered = s.render(ctx);
            if !rendered.is_empty() {
                parts.push(rendered);
            }
        }
        if let Some(ref s) = self.soft_wrap {
            let rendered = s.render(ctx);
            if !rendered.is_empty() {
                parts.push(rendered);
            }
        }

        parts.join("\n\n")
    }
}
