# Extending the Library

`toconfig` is designed to be extended. If a feature is not yet modelled by the built-in structs, you can implement the relevant trait yourself and participate fully in the rendering pipeline — including indentation, doc-comments, and validation.

---

## Implementing `Config` (Neovim / Fish / generic Lua-text)

```rust
use toconfig::{Config, RenderContext};

/// Enables the fast Lua module loader available in Neovim 0.9+.
pub struct VimLoaderNode;

impl Config for VimLoaderNode {
    fn render(&self, ctx: &RenderContext) -> String {
        format!("{}vim.loader.enable()", ctx.indent())
    }

    fn doc_comment(&self) -> Option<&str> {
        Some("Speed up Lua module loading (Neovim 0.9+)")
    }
}
```

Then add it to any `ConfigTree` or `FishConfigTree`:

```rust
let mut tree = ConfigTree::new();
tree.add(VimLoaderNode);
println!("{}", tree.generate());
// → vim.loader.enable()
```

---

## Implementing `HyprlandConfig`

```rust
use toconfig::hyprland::{HyprlandConfig, HyprlandRenderContext};

/// A raw Hyprland plugin declaration.
pub struct HyprlandPlugin {
    path: String,
}

impl HyprlandPlugin {
    pub fn new(path: &str) -> Self {
        Self { path: path.to_string() }
    }
}

impl HyprlandConfig for HyprlandPlugin {
    fn render(&self, _ctx: &HyprlandRenderContext) -> String {
        format!("plugin = {}", self.path)
    }
}
```

---

## Implementing `IniConfig` (GTK / Qt / INI-format)

```rust
use toconfig::ini::{IniConfig, IniRenderContext};

pub struct CustomIniSection {
    name: String,
    entries: Vec<(String, String)>,
}

impl CustomIniSection {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string(), entries: Vec::new() }
    }

    pub fn set(mut self, key: &str, value: &str) -> Self {
        self.entries.push((key.to_string(), value.to_string()));
        self
    }
}

impl IniConfig for CustomIniSection {
    fn render(&self, _ctx: &IniRenderContext) -> String {
        let mut lines = vec![format!("[{}]", self.name)];
        for (k, v) in &self.entries {
            lines.push(format!("{}={}", k, v));
        }
        lines.join("\n")
    }
}
```

---

## Implementing `FontconfigConfig`

```rust
use toconfig::fontconfig::{FontconfigConfig, FontconfigRenderContext};

pub struct FontconfigComment {
    text: String,
}

impl FontconfigComment {
    pub fn new(text: &str) -> Self {
        Self { text: text.to_string() }
    }
}

impl FontconfigConfig for FontconfigComment {
    fn render(&self, ctx: &FontconfigRenderContext) -> String {
        format!("{}<!-- {} -->", ctx.indent(), self.text)
    }
}
```

---

## Validation

Add a `validate` implementation to catch configuration errors before rendering:

```rust
impl Config for MyNode {
    fn render(&self, ctx: &RenderContext) -> String { /* ... */ }

    fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("name must not be empty".into());
        }
        Ok(())
    }
}
```

`ConfigTree::validate_all()` calls `validate()` on every node and returns the first error, so callers get a clear message before any I/O happens.

---

## Using `RenderContext::indent()`

Inside `render`, call `ctx.indent()` to get the current indentation prefix string. Use `ctx.deeper()` to create a child context with one additional indent level for nested rendering:

```rust
fn render(&self, ctx: &RenderContext) -> String {
    let indent = ctx.indent();
    let inner_ctx = ctx.deeper();
    let children: String = self.children
        .iter()
        .map(|c| c.render(&inner_ctx))
        .collect::<Vec<_>>()
        .join("\n");
    format!("{indent}outer {{\n{children}\n{indent}}}")
}
```

---

## Tips

- Keep each custom node in its own file / module — it keeps the tree additions readable.
- Use `RawLua`, `RawHyprland`, `RawToml`, or `RawSystemd` for one-off escapes rather than a full struct.
- If you build a node family that would be useful upstream, consider opening a PR to the `toconfig` repository.
