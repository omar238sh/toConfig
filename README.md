# toconfig

A Rust library for **programmatically generating Neovim `init.lua` files**.

`toconfig` models every major Neovim configuration knob — options, keymaps, autocommands, user commands, theming, and a curated set of popular plugins — as typed Rust structs that implement a single `Config` trait. Call `.generate()` on any node to get valid, indented Lua code back.

---

## Table of Contents

1. [Architecture overview](#architecture-overview)
2. [Core concepts](#core-concepts)
   - [The `Config` trait](#the-config-trait)
   - [`RenderContext`](#rendercontext)
   - [`LuaValue`](#luavalue)
3. [Modules](#modules)
   - [options](#options)
   - [keymap](#keymap)
   - [autocmd](#autocmd)
   - [command](#command)
   - [theme](#theme)
   - [plugins/lazy](#pluginslazy)
   - [plugins/lsp](#pluginslsp)
   - [plugins/telescope](#pluginstelescope)
   - [plugins/cmp](#pluginscmp)
   - [plugins/treesitter](#pluginstreesitter)
   - [output](#output)
   - [profile](#profile)
   - [lua (escape hatch)](#lua-escape-hatch)
4. [End-to-end example](#end-to-end-example)
5. [Extending the library](#extending-the-library)

---

## Architecture overview

```
toconfig
├── core          — Config trait + RenderContext + ConfigTree
├── lua           — LuaValue serialiser + RawLua escape hatch
├── options       — vim.opt / vim.g / vim.o / vim.b / vim.w nodes
├── keymap        — vim.keymap.set nodes + KeymapGroup
├── autocmd       — nvim_create_autocmd / nvim_create_augroup nodes
├── command       — nvim_create_user_command nodes
├── theme         — HexColor / HighlightNode / ThemeNode
├── plugins
│   ├── lazy      — lazy.nvim bootstrap + Plugin spec
│   ├── lsp       — nvim-lspconfig server nodes
│   ├── telescope — telescope.nvim setup node
│   ├── cmp       — nvim-cmp setup node
│   └── treesitter— nvim-treesitter setup node
├── output        — ConfigOutput: write / preview generated Lua
└── profile       — Pre-built ConfigTree bundles (Minimal, LazyIde)
```

Every node is a plain Rust struct. All builder methods use the **fluent / method-chaining** pattern — set only what you need, and opts you leave blank are simply omitted from the generated Lua.

---

## Core concepts

### The `Config` trait

```rust
pub trait Config {
    /// Render this node into a Lua string.
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

Every struct in this library implements `Config`. You can also implement it yourself to add custom nodes — see [Extending the library](#extending-the-library).

### `RenderContext`

```rust
pub struct RenderContext {
    pub indent_level: usize,   // current nesting depth
    pub indent_width: usize,   // spaces per level (default: 2)
    pub emit_doc_comments: bool, // prepend doc_comment() as Lua comments
}
```

`RenderContext::default()` starts at level 0 with 2-space indent. Pass `.deeper()` to recursively indent child nodes.

### `ConfigTree`

`ConfigTree` is the top-level container — it holds an ordered list of any `Config` nodes and optionally a section comment header.

```rust
let mut tree = ConfigTree::new().with_comment("My init.lua");
tree.add(OptionNode::opt("number", LuaValue::bool(true)));
tree.add(KeymapNode::n("<leader>ff", MapRhs::lua("require('telescope.builtin').find_files")));

// Validate everything before writing:
tree.validate_all().expect("config validation failed");

println!("{}", tree.generate());
```

### `LuaValue`

`LuaValue` is the serialisation backbone used by the `options` module (and accessible for use in `RawLua`). It covers every Lua primitive:

| Variant | Constructor | Lua output |
|---------|------------|------------|
| `Nil` | — | `nil` |
| `Boolean(b)` | `LuaValue::bool(b)` | `true` / `false` |
| `Integer(n)` | `LuaValue::int(n)` | `42` |
| `Float(f)` | `LuaValue::float(f)` | `3.14` |
| `Str(s)` | `LuaValue::str(s)` | `'hello'` |
| `Raw(s)` | `LuaValue::raw(s)` | verbatim expression |
| `List(v)` | `LuaValue::list(v)` | `{ ... }` array |
| `Table(pairs)` | `LuaValue::table(pairs)` | `{ key = val, ... }` |

---

## Modules

### options

Set Neovim options via `vim.opt`, `vim.o`, `vim.g`, `vim.b`, or `vim.w`.

```rust
use toconfig::{LuaValue};
use toconfig::options::{OptionNode, OptionsBlock};

// Single option
let node = OptionNode::opt("relativenumber", LuaValue::bool(true));
println!("{}", node.generate());
// → vim.opt.relativenumber = true

// Grouped block with a comment header
let block = OptionsBlock::new()
    .with_comment("UI settings")
    .add(OptionNode::opt("cursorline",   LuaValue::bool(true)))
    .add(OptionNode::opt("scrolloff",    LuaValue::int(8)))
    .add(OptionNode::opt("termguicolors",LuaValue::bool(true)));
```

**Scoped constructors:**

| Method | Vim table |
|--------|-----------|
| `OptionNode::opt(name, val)` | `vim.opt` |
| `OptionNode::o(name, val)` | `vim.o` |
| `OptionNode::g(name, val)` | `vim.g` (global variables) |
| `OptionNode::bo(name, val)` | `vim.b` (buffer-local) |
| `OptionNode::wo(name, val)` | `vim.w` (window-local) |

**Pre-built preset:**

```rust
use toconfig::options::default_editor_options;
let block = default_editor_options(); // number, relativenumber, tabstop=4, etc.
```

---

### keymap

Model `vim.keymap.set(...)` calls with full type safety.

```rust
use toconfig::keymap::{KeymapNode, MapRhs, MapOpts, KeymapGroup};

// Normal-mode mapping → Lua callback
let map = KeymapNode::n("<leader>ff", MapRhs::lua("require('telescope.builtin').find_files"))
    .desc("Find files");

// Insert-mode mapping → Vim command string
let esc = KeymapNode::i("jk", MapRhs::cmd("<Esc>"))
    .silent(true);

// Multi-mode mapping (Normal + Visual)
let yank = KeymapNode::nv("<leader>y", MapRhs::cmd("\"+y"));

// Group: all children get the prefix prepended automatically
let group = KeymapGroup::new("<leader>g")
    .label("Git")
    .add(KeymapNode::n("s", MapRhs::lua("require('gitsigns').stage_hunk()")))
    .add(KeymapNode::n("r", MapRhs::lua("require('gitsigns').reset_hunk()")));

println!("{}", map.generate());
// → vim.keymap.set('n', '<leader>ff', require('telescope.builtin').find_files, { silent = true, noremap = true, desc = 'Find files' })
```

**Mode shortcuts:**

| Method | Mode(s) |
|--------|---------|
| `.n(lhs, rhs)` | Normal |
| `.i(lhs, rhs)` | Insert |
| `.v(lhs, rhs)` | Visual |
| `.t(lhs, rhs)` | Terminal |
| `.nv(lhs, rhs)` | Normal + Visual |

New keymaps default to `{ silent = true, noremap = true }` via `MapOpts::default_safe()`.

---

### autocmd

```rust
use toconfig::autocmd::{AutocmdNode, AutocmdAction, AutocmdPattern, Augroup};

// Simple autocmd
let ac = AutocmdNode::on_file_type("rust", AutocmdAction::cmd("setlocal tabstop=4"))
    .desc("Rust tab width");

// Augroup (prevents duplication on re-source)
let group = Augroup::new("MyGroup")
    .add(AutocmdNode::on_buf_write(AutocmdAction::callback("function() vim.lsp.buf.format() end"))
        .desc("Format on save"))
    .add(AutocmdNode::on_vim_enter(AutocmdAction::cmd("checkhealth")));

println!("{}", group.generate());
// → local MyGroup = vim.api.nvim_create_augroup('MyGroup', { clear = true })
// → vim.api.nvim_create_autocmd('BufWritePost', { group = 'MyGroup', callback = function() ... end, desc = 'Format on save' })
// → vim.api.nvim_create_autocmd('VimEnter', { group = 'MyGroup', command = 'checkhealth' })
```

**Event shortcuts:**

| Constructor | Event |
|-------------|-------|
| `AutocmdNode::on_buf_write(action)` | `BufWritePost` |
| `AutocmdNode::on_buf_enter(action)` | `BufEnter` |
| `AutocmdNode::on_vim_enter(action)` | `VimEnter` |
| `AutocmdNode::on_file_type(ft, action)` | `FileType` with a glob pattern |

---

### command

```rust
use toconfig::command::{UserCommand, CmdCompletion};

let cmd = UserCommand::new("Format", "function(opts) vim.lsp.buf.format() end")
    .desc("Format current buffer via LSP")
    .range(true)
    .nargs("?")
    .complete(CmdCompletion::File);

println!("{}", cmd.generate());
// → vim.api.nvim_create_user_command('Format', function(opts) vim.lsp.buf.format() end, { desc = 'Format current buffer via LSP', nargs = '?', range = true, complete = 'file' })
```

---

### theme

```rust
use toconfig::theme::{ThemeNode, HighlightNode, HighlightAttrs, HexColor};

let theme = ThemeNode::new("catppuccin-mocha")
    .override_hl(
        HighlightNode::new("Normal", HighlightAttrs::new()
            .bg(HexColor::new("#1e1e2e"))
            .fg(HexColor::new("#cdd6f4")))
    )
    .override_hl(
        HighlightNode::new("Comment", HighlightAttrs::new()
            .italic(true)
            .fg(HexColor::new("#6c7086")))
    );

println!("{}", theme.generate());
// → vim.cmd.colorscheme('catppuccin-mocha')
// → vim.api.nvim_set_hl(0, 'Normal', { bg = '#1e1e2e', fg = '#cdd6f4' })
// → vim.api.nvim_set_hl(0, 'Comment', { fg = '#6c7086', italic = true })
```

`HexColor::new` panics at construction if the string is not a valid 7-character `#rrggbb` hex code — giving compile-time-adjacent colour validation in your tests.

---

### plugins/lazy

`Plugin` models a complete [lazy.nvim](https://github.com/folke/lazy.nvim) plugin spec. `LazyManager` wraps the bootstrap snippet + `require('lazy').setup(...)`.

```rust
use toconfig::plugins::lazy::{Plugin, LazyManager};

let manager = LazyManager::new()
    .plugin(Plugin::new("nvim-treesitter/nvim-treesitter")
        .lazy(false)
        .build(":TSUpdate")
        .event("BufReadPost"))
    .plugin(Plugin::new("neovim/nvim-lspconfig")
        .dep(Plugin::new("williamboman/mason.nvim")))
    .plugin(Plugin::new("folke/which-key.nvim")
        .priority(1000)
        .lazy(false));

println!("{}", manager.generate());
// → [lazy.nvim bootstrap code]
// → require('lazy').setup({ ... })
```

**Key `Plugin` setters:** `lazy`, `enabled`, `pin`, `build`, `branch`, `tag`, `commit`, `version`, `priority`, `dep`, `event`, `cmd`, `ft`, `keys`, `init`, `config`, `opts`, `main`, `cond`, `submodules`, `dir`, `url`.

---

### plugins/lsp

```rust
use toconfig::plugins::lsp::{LspConfigNode, ServerConfig};

let lsp = LspConfigNode::new()
    .capabilities("require('cmp_nvim_lsp').default_capabilities()")
    .on_attach("function(client, bufnr) vim.keymap.set('n', 'K', vim.lsp.buf.hover, { buffer = bufnr }) end")
    .server(ServerConfig::new("rust_analyzer")
        .filetypes(&["rust"])
        .root_markers(&["Cargo.toml"])
        .settings("{ ['rust-analyzer'] = { checkOnSave = { command = 'clippy' } } }"))
    .server(ServerConfig::new("lua_ls")
        .single_file_support(true));
```

The global `on_attach` / `capabilities` Lua references are automatically inlined for each server unless the server provides its own override.

---

### plugins/telescope

```rust
use toconfig::plugins::telescope::{TelescopeConfigNode, TelescopeDefaults};

let ts = TelescopeConfigNode::new()
    .defaults(TelescopeDefaults::new()
        .prompt_prefix("🔍 ")
        .border(true)
        .layout_strategy("horizontal")
        .sorting_strategy("ascending"))
    .load_extension("fzf")
    .load_extension("ui-select");

println!("{}", ts.generate());
// → require('telescope').setup({ defaults = { ... } })
// → require('telescope').load_extension('fzf')
// → require('telescope').load_extension('ui-select')
```

---

### plugins/cmp

```rust
use toconfig::plugins::cmp::{CmpConfig, CmpSource};

let cmp = CmpConfig::new()
    .source(CmpSource::new("nvim_lsp").priority(1000))
    .source(CmpSource::new("luasnip").priority(750))
    .source(CmpSource::new("buffer").keyword_length(3))
    .source(CmpSource::new("path"))
    .snippet_engine("{ expand = function(args) require('luasnip').lsp_expand(args.body) end }")
    .mappings("cmp.mapping.preset.insert({ ['<C-Space>'] = cmp.mapping.complete() })");
```

---

### plugins/treesitter

```rust
use toconfig::plugins::treesitter::{TreesitterConfig, TreesitterHighlight};

let ts = TreesitterConfig::new()
    .ensure_installed(&["rust", "lua", "python", "typescript", "markdown"])
    .auto_install(true)
    .highlight(TreesitterHighlight::new(true)
        .disable(&["latex"])
        .additional_vim_regex_highlighting(false))
    .indent(true);
```

---

### output

`ConfigOutput` renders a `Config` node and writes the result to a file (or returns it as a `String`). It performs a **diff-check** — if the file already contains identical content, the write is skipped.

```rust
use toconfig::output::{ConfigOutput, WriteMode};

let out = ConfigOutput::init_lua()       // targets ~/.config/nvim/init.lua
    .emit_ldoc(true);                    // prepend doc_comment() strings as comments

let written = out.write(&my_tree)?;     // returns Ok(true) if file was updated
let preview = out.preview(&my_tree);    // returns String, no I/O

// Custom path
let out2 = ConfigOutput::at_path("/tmp/test_init.lua")
    .mode(WriteMode::Append);
```

---

### profile

Pre-built `ConfigTree` bundles you can use as starting points:

```rust
use toconfig::profile::Profile;

// Minimal: just sensible editor options
let tree = Profile::Minimal.build();

// LazyIde: editor options + IDE extras (split direction, undofile, leaders, etc.)
let tree = Profile::LazyIde.build();
```

---

### lua (escape hatch)

For any Neovim feature not yet modelled by the library, `RawLua` lets you inline arbitrary Lua as a first-class `Config` node — including doc-commenting and indentation.

```rust
use toconfig::lua::RawLua;

let raw = RawLua::new("vim.loader.enable()")
    .with_doc("Speed up Lua module loading");
```

`LuaValue::raw(expr)` serves the same purpose inside `OptionNode` values.

---

## End-to-end example

```rust
use toconfig::{Config, ConfigTree, LuaValue};
use toconfig::options::{OptionNode, default_editor_options};
use toconfig::keymap::{KeymapNode, MapRhs};
use toconfig::autocmd::{Augroup, AutocmdNode, AutocmdAction};
use toconfig::theme::{ThemeNode, HighlightNode, HighlightAttrs, HexColor};
use toconfig::plugins::lazy::{LazyManager, Plugin};
use toconfig::plugins::lsp::{LspConfigNode, ServerConfig};
use toconfig::output::ConfigOutput;

fn main() -> std::io::Result<()> {
    let mut tree = ConfigTree::new().with_comment("init.lua — generated by toconfig");

    // Global leader key (must be set before lazy)
    tree.add(OptionNode::g("mapleader", LuaValue::str(" ")));

    // Plugin manager
    tree.add(
        LazyManager::new()
            .plugin(Plugin::new("folke/tokyonight.nvim").priority(1000).lazy(false))
            .plugin(Plugin::new("neovim/nvim-lspconfig"))
            .plugin(Plugin::new("nvim-treesitter/nvim-treesitter").build(":TSUpdate"))
    );

    // Editor options
    tree.add(default_editor_options());

    // Theme
    tree.add(
        ThemeNode::new("tokyonight-night")
            .override_hl(HighlightNode::new(
                "Comment",
                HighlightAttrs::new().italic(true).fg(HexColor::new("#565f89")),
            ))
    );

    // Keymaps
    tree.add(KeymapNode::n("<leader>w", MapRhs::cmd(":w<CR>")).desc("Save file"));
    tree.add(KeymapNode::n("<leader>q", MapRhs::cmd(":q<CR>")).desc("Quit"));

    // Autocommand group
    tree.add(
        Augroup::new("UserGroup")
            .add(AutocmdNode::on_buf_write(
                AutocmdAction::callback("function() vim.lsp.buf.format({ async = true }) end"),
            ).desc("Format on save"))
    );

    // LSP
    tree.add(
        LspConfigNode::new()
            .server(ServerConfig::new("rust_analyzer").filetypes(&["rust"]))
            .server(ServerConfig::new("lua_ls"))
    );

    // Validate and write
    tree.validate_all().expect("validation errors");
    ConfigOutput::init_lua().write(&tree)?;
    Ok(())
}
```

---

## Extending the library

Implement `Config` for any struct to participate in the rendering pipeline:

```rust
use toconfig::{Config, RenderContext};

pub struct VimLoaderNode;

impl Config for VimLoaderNode {
    fn render(&self, ctx: &RenderContext) -> String {
        format!("{}vim.loader.enable()", ctx.indent())
    }
    fn doc_comment(&self) -> Option<&str> {
        Some("Enable the experimental Lua module loader cache")
    }
}

// Now use it anywhere:
// tree.add(VimLoaderNode);
```

Plugin-specific schemas can additionally implement a **specialized trait** (like `LspConfig`) to enforce structural contracts beyond what `Config` alone tracks.
