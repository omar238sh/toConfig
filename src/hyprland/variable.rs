use super::{HyprlandConfig, HyprlandRenderContext};

/// A Hyprland variable declaration: `$name = value`
///
/// Variables are resolved at config-parse time and may be referenced
/// anywhere in the config as `$name`.
///
/// # Example
/// ```
/// # use toconfig::hyprland::HyprlandConfig;
/// use toconfig::hyprland::Variable;
/// let v = Variable::new("terminal", "kitty");
/// assert_eq!(v.generate(), "$terminal = kitty");
/// ```
pub struct Variable {
    pub name: String,
    pub value: String,
}

impl Variable {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

impl HyprlandConfig for Variable {
    fn render(&self, ctx: &HyprlandRenderContext) -> String {
        format!("{}${} = {}", ctx.indent(), self.name, self.value)
    }

    fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Variable name cannot be empty".into());
        }
        if self.name.contains(' ') {
            return Err(format!(
                "Variable name '{}' must not contain spaces",
                self.name
            ));
        }
        Ok(())
    }
}
