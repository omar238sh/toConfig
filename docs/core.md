# Core Concepts

All modules share a common set of primitives defined in `toconfig::core` and `toconfig::lua`. Understanding these is the key to using every module effectively.

---

## The `Config` trait

```rust
pub trait Config {
    /// Render this node into a string.
    fn render(&self, ctx: &RenderContext) -> String;

    /// Convenience: render with default context (indent = 0).
    fn generate(&self) -> String { self.render(&RenderContext::default()) }

    /// Optional human-readable comment emitted above this node
    /// when `RenderContext::emit_doc_comments` is true.
    fn doc_comment(&self) -> Option<&str> { None }

    /// Pre-render validation hook. Return `Err(msg)` to abort.
    fn validate(&self) -> Result<(), String> { Ok(()) }
}
```

Every Neovim and Fish node implements `Config`. Call `.generate()` to render a node to a `String` with default settings, or `.render(ctx)` to pass a custom `RenderContext`.

> **Note:** Hyprland uses the parallel `HyprlandConfig` trait; GTK/Qt use `IniConfig`; Fontconfig uses `FontconfigConfig`. Each is structurally identical but intentionally separate so the Rust compiler prevents accidentally mixing nodes from different tools.

---

## `RenderContext`

```rust
pub struct RenderContext {
    pub indent_level: usize,     // current nesting depth
    pub indent_width: usize,     // spaces per level (default: 2)
    pub emit_doc_comments: bool, // prepend doc_comment() as comments
}
```

- `RenderContext::default()` starts at level 0 with 2-space indentation.
- Call `.deeper()` inside a node's `render` to increase the indent for child nodes.
- Set `emit_doc_comments = true` to have every node's `doc_comment()` emitted as a Lua / shell comment line above the node.

```rust
let ctx = RenderContext {
    indent_level: 0,
    indent_width: 4,       // 4-space indent
    emit_doc_comments: true,
};
let output = my_node.render(&ctx);
```

---

## `ConfigTree`

`ConfigTree` is the top-level container for Neovim and Fish configurations. It holds an ordered list of any `Config` nodes and an optional section comment header.

```rust
use toconfig::{Config, ConfigTree, LuaValue};
use toconfig::neovim::{OptionNode, KeymapNode, MapRhs};

let mut tree = ConfigTree::new().with_comment("My init.lua");
tree.add(OptionNode::opt("number", LuaValue::bool(true)));
tree.add(KeymapNode::n("<leader>ff", MapRhs::lua("require('telescope.builtin').find_files")));

// Validate all nodes before writing:
tree.validate_all().expect("config validation failed");

println!("{}", tree.generate());
```

Key methods:

| Method | Description |
|--------|-------------|
| `ConfigTree::new()` | Create an empty tree |
| `.with_comment(s)` | Set the header comment |
| `.add(node)` | Append any `impl Config` node (consuming builder) |
| `.push_node(&mut node)` | Append via mutable reference |
| `.validate_all()` | Call `validate()` on every node; returns `Err` on the first failure |
| `.generate()` | Render all nodes in order |

---

## `LuaValue`

`LuaValue` is the serialisation backbone used by the Neovim `options` module and accessible inside `RawLua`. It covers every Lua primitive:

| Variant | Constructor | Lua output |
|---------|-------------|------------|
| `Nil` | — | `nil` |
| `Boolean(b)` | `LuaValue::bool(b)` | `true` / `false` |
| `Integer(n)` | `LuaValue::int(n)` | `42` |
| `Float(f)` | `LuaValue::float(f)` | `3.14` |
| `Str(s)` | `LuaValue::str(s)` | `'hello'` |
| `Raw(s)` | `LuaValue::raw(s)` | verbatim expression |
| `List(v)` | `LuaValue::list(v)` | `{ ... }` array |
| `Table(pairs)` | `LuaValue::table(pairs)` | `{ key = val, ... }` |

Examples:

```rust
use toconfig::LuaValue;

// Primitives
LuaValue::bool(true)          // → true
LuaValue::int(8)              // → 8
LuaValue::float(0.5)          // → 0.5
LuaValue::str("hello")        // → 'hello'
LuaValue::raw("vim.fn.stdpath('data')") // → verbatim

// Collections
LuaValue::list(vec![
    LuaValue::str("rust"),
    LuaValue::str("lua"),
])
// → { 'rust', 'lua' }

LuaValue::table(vec![
    ("timeout".into(), LuaValue::int(300)),
    ("silent".into(),  LuaValue::bool(true)),
])
// → { timeout = 300, silent = true }
```

---

## `RawLua` — escape hatch

For any Lua expression not yet modelled by the library, `RawLua` lets you inline arbitrary Lua as a first-class `Config` node — it participates fully in the tree's indentation and doc-comment machinery.

```rust
use toconfig::RawLua;

let node = RawLua::new("vim.loader.enable()")
    .with_doc("Speed up Lua module loading");

println!("{}", node.generate());
// → vim.loader.enable()
```

`LuaValue::raw(expr)` serves the same purpose inside `OptionNode` values when you need a Lua expression as an option value rather than a standalone line.

---

## Parallel traits

Each tool has its own trait family that mirrors `Config` exactly but is kept separate to avoid mixing incompatible node types in the same tree:

| Tool | Trait | Context | Tree |
|------|-------|---------|------|
| Neovim, Fish, Helix, Systemd, Fstab | `Config` | `RenderContext` | `ConfigTree` / tool-specific tree |
| Hyprland | `HyprlandConfig` | `HyprlandRenderContext` | `HyprlandConfigTree` |
| GTK, Qt | `IniConfig` | `IniRenderContext` | `IniFile` |
| Fontconfig | `FontconfigConfig` | `FontconfigRenderContext` | `FontconfigDocument` |
| Waybar | `WaybarConfig` | `WaybarRenderContext` | `WaybarConfigTree` |
