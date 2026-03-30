# Neovim

The `toconfig::neovim` module generates complete Neovim `init.lua` files from typed Rust structs. All nodes implement the `Config` trait and are collected in a `ConfigTree`.

---

## Table of contents

1. [options](#options)
2. [keymap](#keymap)
3. [autocmd](#autocmd)
4. [command](#command)
5. [theme](#theme)
6. [plugins/lazy](#pluginslazy)
7. [plugins/lsp](#pluginslsp)
8. [plugins/telescope](#pluginstelescope)
9. [plugins/cmp](#pluginscmp)
10. [plugins/treesitter](#pluginstreesitter)
11. [profile](#profile)
12. [lua escape hatch](#lua-escape-hatch)
13. [End-to-end example](#end-to-end-example)

---

## options

Set Neovim options via `vim.opt`, `vim.o`, `vim.g`, `vim.b`, or `vim.w`.

```rust
use toconfig::LuaValue;
use toconfig::neovim::options::{OptionNode, OptionsBlock};

// Single option
let node = OptionNode::opt("relativenumber", LuaValue::bool(true));
println!("{}", node.generate());
// → vim.opt.relativenumber = true

// Grouped block with a comment header
let block = OptionsBlock::new()
    .with_comment("UI settings")
    .add(OptionNode::opt("cursorline",    LuaValue::bool(true)))
    .add(OptionNode::opt("scrolloff",     LuaValue::int(8)))
    .add(OptionNode::opt("termguicolors", LuaValue::bool(true)));
```

### Scoped constructors

| Method | Vim table |
|--------|-----------|
| `OptionNode::opt(name, val)` | `vim.opt` |
| `OptionNode::o(name, val)` | `vim.o` |
| `OptionNode::g(name, val)` | `vim.g` (global variables) |
| `OptionNode::bo(name, val)` | `vim.b` (buffer-local) |
| `OptionNode::wo(name, val)` | `vim.w` (window-local) |

### Pre-built preset

```rust
use toconfig::neovim::options::default_editor_options;

let block = default_editor_options();
// Includes: number, relativenumber, tabstop=4, shiftwidth=4, expandtab, etc.
```

---

## keymap

Model `vim.keymap.set(...)` calls with full type safety.

```rust
use toconfig::neovim::keymap::{KeymapNode, MapRhs, KeymapGroup};

// Normal-mode mapping → Lua callback
let map = KeymapNode::n("<leader>ff", MapRhs::lua("require('telescope.builtin').find_files"))
    .desc("Find files");

// Insert-mode mapping → Vim command string
let esc = KeymapNode::i("jk", MapRhs::cmd("<Esc>"))
    .silent(true);

// Multi-mode mapping (Normal + Visual)
let yank = KeymapNode::nv("<leader>y", MapRhs::cmd("\"+y"));

// Group: all children automatically get the prefix prepended
let group = KeymapGroup::new("<leader>g")
    .label("Git")
    .add(KeymapNode::n("s", MapRhs::lua("require('gitsigns').stage_hunk()")))
    .add(KeymapNode::n("r", MapRhs::lua("require('gitsigns').reset_hunk()")));
```

### Mode shortcuts

| Method | Mode(s) |
|--------|---------|
| `.n(lhs, rhs)` | Normal |
| `.i(lhs, rhs)` | Insert |
| `.v(lhs, rhs)` | Visual |
| `.t(lhs, rhs)` | Terminal |
| `.nv(lhs, rhs)` | Normal + Visual |

New keymaps default to `{ silent = true, noremap = true }` via `MapOpts::default_safe()`.

### `MapRhs` variants

| Constructor | Meaning |
|-------------|---------|
| `MapRhs::cmd(s)` | A Vim command string, e.g. `":w<CR>"` |
| `MapRhs::lua(s)` | A Lua expression / callback |

---

## autocmd

```rust
use toconfig::neovim::autocmd::{AutocmdNode, AutocmdAction, Augroup};

// Simple autocmd
let ac = AutocmdNode::on_file_type("rust", AutocmdAction::cmd("setlocal tabstop=4"))
    .desc("Rust tab width");

// Augroup (prevents duplication on re-source)
let group = Augroup::new("MyGroup")
    .add(AutocmdNode::on_buf_write(
        AutocmdAction::callback("function() vim.lsp.buf.format() end"),
    ).desc("Format on save"))
    .add(AutocmdNode::on_vim_enter(AutocmdAction::cmd("checkhealth")));
```

### Event shortcuts

| Constructor | Event |
|-------------|-------|
| `AutocmdNode::on_buf_write(action)` | `BufWritePost` |
| `AutocmdNode::on_buf_enter(action)` | `BufEnter` |
| `AutocmdNode::on_vim_enter(action)` | `VimEnter` |
| `AutocmdNode::on_file_type(ft, action)` | `FileType` with a glob pattern |

### `AutocmdAction` variants

| Constructor | `command` vs `callback` |
|-------------|-------------------------|
| `AutocmdAction::cmd(s)` | `command = 's'` |
| `AutocmdAction::callback(s)` | `callback = <Lua expr>` |

---

## command

```rust
use toconfig::neovim::command::{UserCommand, CmdCompletion};

let cmd = UserCommand::new("Format", "function(opts) vim.lsp.buf.format() end")
    .desc("Format current buffer via LSP")
    .range(true)
    .nargs("?")
    .complete(CmdCompletion::File);

println!("{}", cmd.generate());
// → vim.api.nvim_create_user_command('Format', function(opts) vim.lsp.buf.format() end,
//     { desc = 'Format current buffer via LSP', nargs = '?', range = true, complete = 'file' })
```

---

## theme

```rust
use toconfig::neovim::theme::{ThemeNode, ColorschemeNode, HighlightNode, HighlightAttrs, HexColor};

// Full theme: colorscheme + highlight overrides
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

// Colorscheme only (no overrides)
let cs = ColorschemeNode::new("tokyonight-night");
```

> `HexColor::new` panics at construction if the string is not a valid 7-character `#rrggbb` hex code.

### `HighlightAttrs` setters

`fg`, `bg`, `sp` (special/underline colour), `bold`, `italic`, `underline`, `undercurl`, `strikethrough`, `reverse`, `link`.

---

## plugins/lazy

`Plugin` models a complete [lazy.nvim](https://github.com/folke/lazy.nvim) plugin spec. `LazyManager` wraps the bootstrap snippet + `require('lazy').setup(...)`.

```rust
use toconfig::neovim::plugins::{Plugin, LazyManager};

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
```

### Key `Plugin` setters

`lazy`, `enabled`, `pin`, `build`, `branch`, `tag`, `commit`, `version`, `priority`, `dep`, `event`, `cmd`, `ft`, `keys`, `init`, `config`, `opts`, `main`, `cond`, `submodules`, `dir`, `url`.

---

## plugins/lsp

```rust
use toconfig::neovim::plugins::{LspConfigNode, ServerConfig};

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

The global `on_attach` / `capabilities` are automatically inlined for each server unless the server provides its own override.

---

## plugins/telescope

```rust
use toconfig::neovim::plugins::{TelescopeConfigNode, TelescopeDefaults};

let ts = TelescopeConfigNode::new()
    .defaults(TelescopeDefaults::new()
        .prompt_prefix("🔍 ")
        .border(true)
        .layout_strategy("horizontal")
        .sorting_strategy("ascending"))
    .load_extension("fzf")
    .load_extension("ui-select");
```

---

## plugins/cmp

```rust
use toconfig::neovim::plugins::{CmpConfig, CmpSource};

let cmp = CmpConfig::new()
    .source(CmpSource::new("nvim_lsp").priority(1000))
    .source(CmpSource::new("luasnip").priority(750))
    .source(CmpSource::new("buffer").keyword_length(3))
    .source(CmpSource::new("path"))
    .snippet_engine("{ expand = function(args) require('luasnip').lsp_expand(args.body) end }")
    .mappings("cmp.mapping.preset.insert({ ['<C-Space>'] = cmp.mapping.complete() })");
```

---

## plugins/treesitter

```rust
use toconfig::neovim::plugins::{TreesitterConfig, TreesitterHighlight};

let ts = TreesitterConfig::new()
    .ensure_installed(&["rust", "lua", "python", "typescript", "markdown"])
    .auto_install(true)
    .highlight(TreesitterHighlight::new(true)
        .disable(&["latex"])
        .additional_vim_regex_highlighting(false))
    .indent(true);
```

---

## profile

Pre-built `ConfigTree` bundles you can use as starting points.

```rust
use toconfig::neovim::profile::Profile;

// Minimal: sensible editor options only
let tree = Profile::Minimal.build();

// LazyIde: editor options + IDE extras (split direction, undofile, leaders, etc.)
let tree = Profile::LazyIde.build();
```

---

## lua escape hatch

For any Neovim feature not yet modelled by the library, `RawLua` lets you inline arbitrary Lua as a first-class `Config` node.

```rust
use toconfig::RawLua;

let raw = RawLua::new("vim.loader.enable()")
    .with_doc("Speed up Lua module loading");
```

---

## End-to-end example

```rust
use toconfig::{Config, ConfigTree, LuaValue, RawLua};
use toconfig::neovim::options::{OptionNode, default_editor_options};
use toconfig::neovim::keymap::{KeymapNode, MapRhs};
use toconfig::neovim::autocmd::{Augroup, AutocmdNode, AutocmdAction};
use toconfig::neovim::theme::{ThemeNode, HighlightNode, HighlightAttrs, HexColor};
use toconfig::neovim::plugins::{LazyManager, Plugin, LspConfigNode, ServerConfig};
use toconfig::output::ConfigOutput;

fn main() -> std::io::Result<()> {
    let mut tree = ConfigTree::new().with_comment("init.lua — generated by toconfig");

    // Leader key (must be set before lazy)
    tree.add(OptionNode::g("mapleader", LuaValue::str(" ")));

    // Plugin manager
    tree.add(
        LazyManager::new()
            .plugin(Plugin::new("folke/tokyonight.nvim").priority(1000).lazy(false))
            .plugin(Plugin::new("neovim/nvim-lspconfig"))
            .plugin(Plugin::new("nvim-treesitter/nvim-treesitter").build(":TSUpdate")),
    );

    // Editor options
    tree.add(default_editor_options());

    // Theme with highlight override
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
            .server(ServerConfig::new("lua_ls")),
    );

    tree.validate_all().expect("validation errors");
    ConfigOutput::init_lua().write(&tree)?;
    Ok(())
}
```
