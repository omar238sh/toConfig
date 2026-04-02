use super::core::{HelixConfig, HelixRenderContext};

/// The editor mode a keybinding applies to.
#[derive(Debug, Clone, PartialEq)]
pub enum KeyMode {
    Normal,
    Insert,
    Select,
}

impl KeyMode {
    pub fn as_str(&self) -> &str {
        match self {
            KeyMode::Normal => "normal",
            KeyMode::Insert => "insert",
            KeyMode::Select => "select",
        }
    }
}

/// The action bound to a key.
///
/// A Helix key binding can be:
/// - A single command string (e.g. `"move_char_right"` or `":write"`)
/// - A sequence of commands rendered as a TOML array
/// - A nested keymap rendered as a TOML inline table (`{ key = "cmd", … }`)
#[derive(Debug, Clone)]
pub enum KeyAction {
    /// Single command: `"move_char_right"`
    Command(String),
    /// Sequence of commands: `["cmd1", "cmd2"]`
    Sequence(Vec<String>),
    /// Nested keymap: `{ <key> = <action>, … }` (one level deep).
    ///
    /// The pairs are `(key_string, command_string)`.
    Nested(Vec<(String, String)>),
}

impl KeyAction {
    /// Convenience: create a single-command action.
    pub fn cmd(s: impl Into<String>) -> Self {
        KeyAction::Command(s.into())
    }

    /// Convenience: create a sequence action.
    pub fn seq(cmds: Vec<impl Into<String>>) -> Self {
        KeyAction::Sequence(cmds.into_iter().map(|s| s.into()).collect())
    }

    /// Convenience: create a nested keymap action.
    pub fn nested(pairs: Vec<(impl Into<String>, impl Into<String>)>) -> Self {
        KeyAction::Nested(
            pairs
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        )
    }

    fn to_toml(&self) -> String {
        match self {
            KeyAction::Command(cmd) => format!("\"{}\"", cmd),
            KeyAction::Sequence(cmds) => {
                let inner: Vec<String> = cmds.iter().map(|c| format!("\"{}\"", c)).collect();
                format!("[{}]", inner.join(", "))
            }
            KeyAction::Nested(pairs) => {
                let inner: Vec<String> = pairs
                    .iter()
                    .map(|(k, v)| format!("{} = \"{}\"", k, v))
                    .collect();
                format!("{{ {} }}", inner.join(", "))
            }
        }
    }
}

/// A single key binding entry: `"key" = action`.
#[derive(Debug, Clone)]
pub struct KeyBinding {
    pub key: String,
    pub action: KeyAction,
}

impl KeyBinding {
    pub fn new(key: impl Into<String>, action: KeyAction) -> Self {
        Self {
            key: key.into(),
            action,
        }
    }
}

/// A keybinding section for one editor mode (`[keys.normal]`, `[keys.insert]`,
/// or `[keys.select]`).
///
/// # Example
/// ```
/// # use toconfig::helix::HelixConfig;
/// use toconfig::helix::keys::{KeysSection, KeyMode, KeyBinding, KeyAction};
///
/// let sec = KeysSection::new(KeyMode::Normal)
///     .bind(KeyBinding::new("C-s", KeyAction::cmd(":write")))
///     .bind(KeyBinding::new("space", KeyAction::nested(vec![
///         ("f", ":format"),
///         ("w", ":write"),
///     ])));
///
/// let out = sec.generate();
/// assert!(out.contains("[keys.normal]"));
/// assert!(out.contains("C-s = \":write\""));
/// ```
pub struct KeysSection {
    pub mode: KeyMode,
    pub bindings: Vec<KeyBinding>,
}

impl KeysSection {
    pub fn new(mode: KeyMode) -> Self {
        Self {
            mode,
            bindings: Vec::new(),
        }
    }

    /// Add a key binding (consuming builder).
    pub fn bind(mut self, binding: KeyBinding) -> Self {
        self.bindings.push(binding);
        self
    }

    /// Add a key binding (mutable borrow).
    pub fn add_bind(&mut self, binding: KeyBinding) -> &mut Self {
        self.bindings.push(binding);
        self
    }

    /// Shorthand: add a simple command binding.
    ///
    /// ```
    /// # use toconfig::helix::keys::{KeysSection, KeyMode};
    /// let sec = KeysSection::new(KeyMode::Insert)
    ///     .cmd("j", "{ j = \"normal_mode\" }");
    /// // Note: for actual nested maps, prefer KeyAction::nested.
    /// ```
    pub fn cmd(mut self, key: impl Into<String>, command: impl Into<String>) -> Self {
        self.bindings.push(KeyBinding::new(
            key.into(),
            KeyAction::Command(command.into()),
        ));
        self
    }
}

impl HelixConfig for KeysSection {
    fn render(&self, _ctx: &HelixRenderContext) -> String {
        if self.bindings.is_empty() {
            return String::new();
        }
        let mut lines = vec![format!("[keys.{}]", self.mode.as_str())];
        for binding in &self.bindings {
            lines.push(format!("{} = {}", binding.key, binding.action.to_toml()));
        }
        lines.join("\n")
    }

    fn validate(&self) -> Result<(), String> {
        for binding in &self.bindings {
            if binding.key.is_empty() {
                return Err("KeysSection: key string cannot be empty".into());
            }
        }
        Ok(())
    }
}
