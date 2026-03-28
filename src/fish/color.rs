use super::util::quote_fish_value;
use crate::core::{Config, RenderContext};

// ── Color Var ─────────────────────────────────────────────────────────────────

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

// ── Color ─────────────────────────────────────────────────────────────────────

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
