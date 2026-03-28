# toconfig

A Rust library for **programmatically generating configuration files** for Neovim, Hyprland, and Fish shell.

`toconfig` models configuration as typed Rust structs that implement a single trait. Call `.generate()` on any node to get valid, indented output back. All builder methods use the **fluent / method-chaining** pattern — set only what you need, and fields you leave blank are simply omitted from the generated output.

---

## Table of Contents

1. [Architecture overview](#architecture-overview)
2. [Core concepts](#core-concepts)
   - [The `Config` trait](#the-config-trait)
   - [`RenderContext`](#rendercontext)
   - [`ConfigTree`](#configtree)
   - [`LuaValue`](#luavalue)
3. [Neovim](#neovim)
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
   - [output (Neovim)](#output-neovim)
   - [profile](#profile)
   - [lua (escape hatch)](#lua-escape-hatch)
   - [Neovim end-to-end example](#neovim-end-to-end-example)
4. [Hyprland](#hyprland)
   - [Variables](#variables)
   - [Environment variables](#environment-variables)
   - [Monitor configuration](#monitor-configuration)
   - [Exec / startup commands](#exec--startup-commands)
   - [Keybinds](#keybinds)
   - [Window rules](#window-rules)
   - [Workspace rules](#workspace-rules)
   - [Animations](#animations)
   - [Layouts](#layouts)
   - [Sections (generic blocks)](#sections-generic-blocks)
   - [XWayland](#xwayland)
   - [Permissions](#permissions)
   - [output (Hyprland)](#output-hyprland)
   - [Hyprland end-to-end example](#hyprland-end-to-end-example)
5. [Fish shell](#fish-shell)
   - [Variables](#fish-variables)
   - [Aliases](#aliases)
   - [Abbreviations](#abbreviations)
   - [Key bindings](#key-bindings)
   - [Functions](#functions)
   - [Completions](#completions)
   - [PATH management](#path-management)
   - [Colors](#colors)
   - [Prompts & greeting](#prompts--greeting)
   - [Control flow](#control-flow)
   - [Source / plugins](#source--plugins)
   - [FishConfigTree](#fishconfigtree)
   - [Fish end-to-end example](#fish-end-to-end-example)
6. [Extending the library](#extending-the-library)

---

## Architecture overview

```
toconfig
├── core          — Config trait + RenderContext + ConfigTree
├── lua           — LuaValue serialiser + RawLua escape hatch
├── output        — ConfigOutput: write / preview generated Lua
├── neovim
│   ├── options       — vim.opt / vim.g / vim.o / vim.b / vim.w nodes
│   ├── keymap        — vim.keymap.set nodes + KeymapGroup
│   ├── autocmd       — nvim_create_autocmd / nvim_create_augroup nodes
│   ├── command       — nvim_create_user_command nodes
│   ├── theme         — HexColor / HighlightNode / ThemeNode / ColorschemeNode
│   ├── plugins
│   │   ├── lazy      — lazy.nvim bootstrap + Plugin spec
│   │   ├── lsp       — nvim-lspconfig server nodes
│   │   ├── telescope — telescope.nvim setup node
│   │   ├── cmp       — nvim-cmp setup node
│   │   └── treesitter— nvim-treesitter setup node
│   └── profile       — Pre-built ConfigTree bundles (Minimal, LazyIde)
├── hyprland
│   ├── core          — HyprlandConfig trait + HyprlandConfigTree + HyprlandRenderContext
│   ├── variable      — $name = value declarations
│   ├── environment   — env = KEY,value lines + Wayland/NVIDIA helpers
│   ├── monitor       — monitor= configuration lines
│   ├── exec          — exec-once / exec / plugin / source directives
│   ├── bind          — bind / bindm keybind rules + Dispatcher enum
│   ├── window_rule   — windowrulev2 rules
│   ├── workspace     — workspace rules
│   ├── animation     — animations { } section with Bezier curves
│   ├── layout        — dwindle / master / scroller / monocle layouts
│   ├── section       — generic Section block + RawHyprland escape hatch
│   ├── xwayland      — xwayland { } section
│   ├── permission    — permission = rules
│   └── output        — HyprlandOutput: write / preview generated config
└── fish
    ├── config_tree   — FishConfigTree container
    ├── variable      — set statements + VarScope
    ├── alias         — alias statements
    ├── abbr          — abbr --add abbreviations
    ├── bind          — bind key bindings
    ├── function      — function … end definitions + FishEvent
    ├── completion    — complete statements
    ├── path          — fish_add_path
    ├── color         — fish_color_* / fish_pager_color_* variables
    ├── prompt        — fish_prompt / fish_right_prompt / fish_mode_prompt / fish_greeting
    ├── conditional   — if / switch / for / while / begin blocks
    └── source        — source / fisher install / FishRawLine escape hatch
```

---

## Core concepts

### The `Config` trait

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

Every Neovim and Fish struct in this library implements `Config`. Hyprland nodes implement the parallel `HyprlandConfig` trait (see [Hyprland](#hyprland)).

### `RenderContext`

```rust
pub struct RenderContext {
    pub indent_level: usize,     // current nesting depth
    pub indent_width: usize,     // spaces per level (default: 2)
    pub emit_doc_comments: bool, // prepend doc_comment() as comments
}
```

`RenderContext::default()` starts at level 0 with 2-space indent. Pass `.deeper()` to recursively indent child nodes.

### `ConfigTree`

`ConfigTree` is the top-level container for Neovim and Fish configurations — it holds an ordered list of any `Config` nodes and optionally a section comment header.

```rust
use toconfig::{Config, ConfigTree, LuaValue};
use toconfig::neovim::{OptionNode, KeymapNode, MapRhs};

let mut tree = ConfigTree::new().with_comment("My init.lua");
tree.add(OptionNode::opt("number", LuaValue::bool(true)));
tree.add(KeymapNode::n("<leader>ff", MapRhs::lua("require('telescope.builtin').find_files")));

// Validate everything before writing:
tree.validate_all().expect("config validation failed");

println!("{}", tree.generate());
```

### `LuaValue`

`LuaValue` is the serialisation backbone used by the `options` module (and accessible inside `RawLua`). It covers every Lua primitive:

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

## Neovim

All Neovim builders live in the `toconfig::neovim` namespace. Each sub-module is also re-exported at the `toconfig::neovim` level for convenience.

### options

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
use toconfig::neovim::options::default_editor_options;
let block = default_editor_options(); // number, relativenumber, tabstop=4, etc.
```

---

### keymap

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
use toconfig::neovim::autocmd::{AutocmdNode, AutocmdAction, Augroup};

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
use toconfig::neovim::command::{UserCommand, CmdCompletion};

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

println!("{}", theme.generate());
// → vim.cmd.colorscheme('catppuccin-mocha')
// → vim.api.nvim_set_hl(0, 'Normal', { bg = '#1e1e2e', fg = '#cdd6f4' })
// → vim.api.nvim_set_hl(0, 'Comment', { fg = '#6c7086', italic = true })

// Colorscheme only (no overrides)
let cs = ColorschemeNode::new("tokyonight-night");
println!("{}", cs.generate());
// → vim.cmd.colorscheme('tokyonight-night')
```

`HexColor::new` panics at construction if the string is not a valid 7-character `#rrggbb` hex code — giving compile-time-adjacent colour validation in your tests.

**`HighlightAttrs` setters:** `fg`, `bg`, `sp` (special/underline colour), `bold`, `italic`, `underline`, `undercurl`, `strikethrough`, `reverse`, `link`.

---

### plugins/lazy

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

println!("{}", manager.generate());
// → [lazy.nvim bootstrap code]
// → require('lazy').setup({ ... })
```

**Key `Plugin` setters:** `lazy`, `enabled`, `pin`, `build`, `branch`, `tag`, `commit`, `version`, `priority`, `dep`, `event`, `cmd`, `ft`, `keys`, `init`, `config`, `opts`, `main`, `cond`, `submodules`, `dir`, `url`.

---

### plugins/lsp

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

The global `on_attach` / `capabilities` Lua references are automatically inlined for each server unless the server provides its own override.

---

### plugins/telescope

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

println!("{}", ts.generate());
// → require('telescope').setup({ defaults = { ... } })
// → require('telescope').load_extension('fzf')
// → require('telescope').load_extension('ui-select')
```

---

### plugins/cmp

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

### plugins/treesitter

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

### output (Neovim)

`ConfigOutput` renders a `Config` node and writes the result to a file (or returns it as a `String`). It performs a **diff-check** — if the file already contains identical content, the write is skipped.

```rust
use toconfig::output::{ConfigOutput, WriteMode};

let out = ConfigOutput::init_lua()    // targets ~/.config/nvim/init.lua
    .emit_ldoc(true);                 // prepend doc_comment() strings as comments

let written = out.write(&my_tree)?;  // returns Ok(true) if file was updated
let preview = out.preview(&my_tree); // returns String, no I/O

// Custom path
let out2 = ConfigOutput::at_path("/tmp/test_init.lua")
    .mode(WriteMode::Append);
```

---

### profile

Pre-built `ConfigTree` bundles you can use as starting points:

```rust
use toconfig::neovim::profile::Profile;

// Minimal: just sensible editor options
let tree = Profile::Minimal.build();

// LazyIde: editor options + IDE extras (split direction, undofile, leaders, etc.)
let tree = Profile::LazyIde.build();
```

---

### lua (escape hatch)

For any Neovim feature not yet modelled by the library, `RawLua` lets you inline arbitrary Lua as a first-class `Config` node — including doc-commenting and indentation.

```rust
use toconfig::RawLua;

let raw = RawLua::new("vim.loader.enable()")
    .with_doc("Speed up Lua module loading");
```

`LuaValue::raw(expr)` serves the same purpose inside `OptionNode` values.

---

### Neovim end-to-end example

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

## Hyprland

All Hyprland builders live in the `toconfig::hyprland` namespace. Hyprland nodes implement the **`HyprlandConfig`** trait, which is intentionally separate from the Neovim `Config` trait — the Rust compiler prevents accidentally mixing Neovim and Hyprland nodes in the same tree.

```rust
pub trait HyprlandConfig {
    fn render(&self, ctx: &HyprlandRenderContext) -> String;
    fn generate(&self) -> String { ... }         // render with default context
    fn validate(&self) -> Result<(), String> { Ok(()) }
}
```

`HyprlandRenderContext` defaults to 0-level, 4-space indentation.

### Variables

```rust
use toconfig::hyprland::{HyprlandConfig, Variable};

let v = Variable::new("terminal", "kitty");
println!("{}", v.generate());
// → $terminal = kitty
```

### Environment variables

```rust
use toconfig::hyprland::{HyprlandConfig, EnvVar};

// Arbitrary variable
let e = EnvVar::new("MY_VAR", "hello");

// Convenience constructors for common Wayland / NVIDIA setups:
EnvVar::xcursor_size(24)
EnvVar::xcursor_theme("Adwaita")
EnvVar::qt_wayland()         // QT_QPA_PLATFORM=wayland
EnvVar::qt_no_csd()
EnvVar::xdg_current_desktop("Hyprland")
EnvVar::xdg_session_wayland()
EnvVar::gdk_wayland()
EnvVar::sdl_wayland()
EnvVar::preferred_gpu("/dev/dri/card1:/dev/dri/card0") // multi-GPU
EnvVar::nvidia_libva()
EnvVar::nvidia_gbm()
EnvVar::nvidia_glx()
EnvVar::nvidia_explicit_sync()

println!("{}", EnvVar::xcursor_size(24).generate());
// → env = XCURSOR_SIZE,24
```

### Monitor configuration

```rust
use toconfig::hyprland::{HyprlandConfig, MonitorConfig};

// Named monitor at 1080p 60 Hz, 1× scaling
let m = MonitorConfig::new("eDP-1", "1920x1080@60", "0x0", 1.0);
// → monitor=eDP-1,1920x1080@60,0x0,1

// Catch-all fallback
let fallback = MonitorConfig::auto();
// → monitor=,preferred,auto,1

// HiDPI with 90° rotation and VRR
let hidpi = MonitorConfig::new("DP-1", "3840x2160@60", "1920x0", 2.0)
    .transform(1)
    .vrr(1);
```

**Optional setters:** `transform` (0–7), `mirror`, `bitdepth`, `vrr` (0–2).

---

### Exec / startup commands

```rust
use toconfig::hyprland::exec::{ExecOnce, Exec, PluginLoad, Source};

ExecOnce::new("waybar")                      // exec-once = waybar
Exec::new("swww img ~/wallpaper.png")        // exec = swww img ~/wallpaper.png
PluginLoad::new("/usr/lib/hyprland/plugin.so") // plugin = ...
Source::new("~/.config/hypr/keybinds.conf") // source = ...
ExecOnce::hyprctl_dispatch("workspace 1")   // exec-once = hyprctl dispatch workspace 1
```

---

### Keybinds

```rust
use toconfig::hyprland::{HyprlandConfig, Bind, Dispatcher};

// Launch terminal
let b = Bind::new("SUPER", "Return", Dispatcher::Exec("$terminal".into()));
// → bind = SUPER, Return, exec, $terminal

// Kill window
let k = Bind::new("SUPER", "Q", Dispatcher::KillActive);
// → bind = SUPER, Q, killactive

// Switch workspace
let ws = Bind::new("SUPER", "1", Dispatcher::Workspace("1".into()));

// Mouse bind (bindm)
let m = Bind::new("SUPER", "mouse:272", Dispatcher::Custom("movewindow".into(), None))
    .mouse();
// → bindm = SUPER, mouse:272, movewindow
```

**`Bind` flag modifiers:** `.locked()` (`bindl`), `.release()` (`bindr`), `.repeat()` (`binde`), `.non_consuming()` (`bindn`), `.mouse()` (`bindm`).

**Common `Dispatcher` variants:** `Exec`, `KillActive`, `ForceCloseActive`, `ToggleFloating`, `FullScreen(u8)`, `TogglePseudo`, `ToggleSplit`, `MoveFocus`, `SwapWindow`, `MoveWindow`, `ResizeActive`, `Workspace`, `MoveToWorkspace`, `MoveToWorkspaceSilent`, `ToggleSpecialWorkspace`, `CycleNext`, `CyclePrev`, `Pin`, `SplitRatio`, `Custom`.

---

### Window rules

```rust
use toconfig::hyprland::{HyprlandConfig, WindowRule};

// Float pavucontrol
let r = WindowRule::new("float", "class:^(pavucontrol)$");
// → windowrulev2 = float, class:^(pavucontrol)$

// Multiple matchers (AND logic)
let r2 = WindowRule::new("float", "class:^(kitty)$")
    .and("title:^(float)$");

// Allow tearing for games
let tearing = WindowRule::new("immediate", "class:^(game_binary)$");
```

---

### Workspace rules

```rust
use toconfig::hyprland::workspace::WorkspaceRule;

let w = WorkspaceRule::new("1")
    .monitor("eDP-1")
    .default();
// → workspace = 1, monitor:eDP-1, default:true

// Special workspace (scratchpad) with auto-launch
WorkspaceRule::new("special:magic")
    .on_created_empty("kitty");
```

**Other setters:** `persistent()`, `gaps_in(px)`, `gaps_out(px)`, `rule(key, value)`.

---

### Animations

```rust
use toconfig::hyprland::{HyprlandConfig, AnimationsSection, Bezier, Animation};

let sec = AnimationsSection::new()
    .bezier(Bezier::ease_out_back("myBezier"))
    .bezier(Bezier::ease_in_out("smoothBezier"))
    .animation(Animation::new("windows", 7.0, "myBezier").style("slide"))
    .animation(Animation::new("workspaces", 6.0, "smoothBezier"));
```

**Bezier presets:** `ease_in_out`, `ease_out_back`, `ease_out`, `linear`. Custom: `Bezier::new(name, p1x, p1y, p2x, p2y)`.

**`Animation::new(name, speed, curve)`** — optional `.style("slide"|"popin"|"fade")` and `.disabled()`.

---

### Layouts

```rust
use toconfig::hyprland::layout::{DwindleLayout, MasterLayout, ScrollingLayout, MonocleLayout};

// Dwindle (default)
DwindleLayout::new()
    .pseudotile(true)
    .preserve_split(true)
    .force_split(2);

// Master
MasterLayout::new()
    .mfact(0.55)
    .new_status("master")
    .orientation("left");

// Scrolling (hyprscroller plugin required)
ScrollingLayout::new()
    .column_default_width("onehalf")
    .focus_wrap(true);

// Monocle (hyprmonocle plugin required)
MonocleLayout::new().center(true);
```

---

### Sections (generic blocks)

`Section` covers every named block in Hyprland: `general`, `input`, `decoration`, `misc`, `binds`, `cursor`, `render`, `opengl`, `debug`, `group`, `group:groupbar`, and any plugin-defined section.

```rust
use toconfig::hyprland::section::{Section, RawHyprland};

let general = Section::new("general")
    .pair("gaps_in", "5")
    .pair("gaps_out", "20")
    .pair("border_size", "2")
    .pair("allow_tearing", "false");

// Nested sub-section
let input = Section::new("input")
    .pair("kb_layout", "us")
    .nested(
        Section::new("touchpad")
            .pair("natural_scroll", "true")
    );

// Raw escape hatch
let raw = RawHyprland::new("misc {\n    disable_hyprland_logo = true\n}");
```

---

### XWayland

```rust
use toconfig::hyprland::{HyprlandConfig, XWaylandSection};

XWaylandSection::new()
    .force_zero_scaling(true)   // recommended for HiDPI
    .use_nearest_neighbor(false);
```

---

### Permissions

```rust
use toconfig::hyprland::permission::{Permission, PermissionType, PermissionAction};

// Allow OBS to record the screen
Permission::allow_screencopy("/usr/bin/obs");
// → permission = /usr/bin/obs, screencopy, allow

// Deny everything else
Permission::deny_screencopy("/usr/bin/.*");

// Custom
Permission::new("/usr/bin/cheese", PermissionType::Camera, PermissionAction::Ask);
```

---

### output (Hyprland)

```rust
use toconfig::hyprland::output::HyprlandOutput;

// Write to ~/.config/hypr/hyprland.conf
HyprlandOutput::hyprland_conf().write(&tree)?;

// Custom path or preview
HyprlandOutput::at_path("/tmp/test.conf").write(&tree)?;
let preview = HyprlandOutput::hyprland_conf().preview(&tree);
```

Performs a diff-check and skips the write if content is unchanged. Returns `Ok(true)` when written, `Ok(false)` when skipped.

---

### Hyprland end-to-end example

```rust
use toconfig::hyprland::{
    HyprlandConfig, HyprlandConfigTree, Variable, EnvVar, MonitorConfig,
    Bind, Dispatcher, WindowRule, AnimationsSection, Bezier, Animation,
    DwindleLayout, XWaylandSection,
};
use toconfig::hyprland::exec::ExecOnce;
use toconfig::hyprland::output::HyprlandOutput;
use toconfig::hyprland::section::Section;
use toconfig::hyprland::workspace::WorkspaceRule;

fn main() -> std::io::Result<()> {
    let mut tree = HyprlandConfigTree::new()
        .with_comment("hyprland.conf — generated by toconfig");

    tree.add(Variable::new("terminal", "kitty"))
        .add(Variable::new("browser", "firefox"))
        .add(MonitorConfig::new("eDP-1", "1920x1080@60", "0x0", 1.0))
        .add(EnvVar::xcursor_size(24))
        .add(EnvVar::xdg_session_wayland())
        .add(ExecOnce::new("waybar"))
        .add(ExecOnce::new("dunst"))
        .add(
            Section::new("general")
                .pair("gaps_in", "5")
                .pair("gaps_out", "20")
                .pair("border_size", "2"),
        )
        .add(
            Section::new("input")
                .pair("kb_layout", "us")
                .pair("follow_mouse", "1"),
        )
        .add(Bind::new("SUPER", "Return", Dispatcher::Exec("$terminal".into())))
        .add(Bind::new("SUPER", "Q", Dispatcher::KillActive))
        .add(Bind::new("SUPER", "1", Dispatcher::Workspace("1".into())))
        .add(WindowRule::new("float", "class:^(pavucontrol)$"))
        .add(WorkspaceRule::new("1").monitor("eDP-1").default())
        .add(
            AnimationsSection::new()
                .bezier(Bezier::ease_out_back("myBezier"))
                .animation(Animation::new("windows", 7.0, "myBezier").style("slide")),
        )
        .add(DwindleLayout::new().pseudotile(true).preserve_split(true))
        .add(XWaylandSection::new().force_zero_scaling(true));

    tree.validate_all().expect("validation errors");
    HyprlandOutput::hyprland_conf().write(&tree)?;
    Ok(())
}
```

---

## Fish shell

All Fish builders live in the `toconfig::fish` namespace. They implement the standard `Config` trait and are collected in a `FishConfigTree`.

### Fish variables

```rust
use toconfig::fish::variable::{FishVariable, VarScope};

// Scoped constructors
FishVariable::global("EDITOR", "nvim")     // set -g EDITOR nvim
FishVariable::local("x", "42")             // set -l x 42
FishVariable::universal("theme", "dark")   // set -U theme dark
FishVariable::env("PATH", "/usr/local/bin") // set -gx PATH /usr/local/bin

// Multi-value list
FishVariable::with_values("colors", &["red", "green", "blue"])

// Builder methods: .scope(VarScope::Global), .export(true), .path_list(true), .erase()
```

---

### Aliases

```rust
use toconfig::fish::alias::FishAlias;

FishAlias::new("ll", "ls -la")
// → alias ll 'ls -la'
```

---

### Abbreviations

```rust
use toconfig::fish::abbr::{FishAbbr, AbbrPosition};

// Basic
FishAbbr::new("gco", "git checkout")
// → abbr --add gco 'git checkout'

// Expand anywhere in the command line
FishAbbr::new("gs", "git status").anywhere()

// Function-based expansion
FishAbbr::with_function("date", "current_date_fn")
// → abbr --add --function current_date_fn date

// With regex trigger
FishAbbr::new("md", "mkdir -p").regex(r"^mk?d")
```

---

### Key bindings

```rust
use toconfig::fish::bind::{FishBind, BindMode};

FishBind::new("\\cc", "kill_job")
// → bind \cc kill_job

FishBind::new("\\e[A", "up-or-search")
    .mode(BindMode::Insert)
    .silent()
// → bind --mode insert --silent \e[A up-or-search
```

---

### Functions

```rust
use toconfig::fish::function::{FishFunction, FishEvent};

FishFunction::new("mkcd", &[
    "mkdir -p $argv[1]",
    "and cd $argv[1]",
])
.description("Make a directory and cd into it")
.argument_names(&["dir"])

// Event-triggered function
FishFunction::new("__on_pwd_change", &["echo PWD changed to $PWD"])
    .on_event(FishEvent::Variable("PWD".into()))

// Wrap another command's completions
FishFunction::new("g", &["git $argv"]).wrap("git")
```

**`FishEvent` variants:** `Event(name)`, `Variable(name)`, `ProcessExit(pid)`, `Signal(name)`, `JobExit(pid)`.

---

### Completions

```rust
use toconfig::fish::completion::FishCompletion;

FishCompletion::new("myapp")
    .short("v")
    .long("verbose")
    .description("Enable verbose output")
    .no_argument()
// → complete -c myapp -s v -l verbose -f -d 'Enable verbose output'

FishCompletion::new("myapp")
    .long("output")
    .requires_argument()
    .condition("__fish_myapp_needs_file")
    .arguments("(__fish_complete_path)")
```

---

### PATH management

```rust
use toconfig::fish::path::FishAddPath;

FishAddPath::new(&["$HOME/.local/bin", "/opt/homebrew/bin"])
// → fish_add_path $HOME/.local/bin /opt/homebrew/bin

FishAddPath::new(&["/opt/bin"]).prepend().global()
// → fish_add_path --prepend --global /opt/bin
```

---

### Colors

```rust
use toconfig::fish::color::{FishColor, FishColorVar};

FishColor::new(FishColorVar::Command, "brblue")
// → set -g fish_color_command brblue

FishColor::new(FishColorVar::Comment, "555 --italics")
FishColor::new(FishColorVar::Autosuggestion, "#6c7086")
```

All `fish_color_*` and `fish_pager_color_*` variables are covered by the `FishColorVar` enum.

---

### Prompts & greeting

```rust
use toconfig::fish::prompt::{FishPrompt, FishRightPrompt, FishModePrompt, FishGreeting};

// Custom left prompt
FishPrompt::new(&[
    "set_color brblue",
    "echo -n (prompt_pwd)",
    "set_color normal",
    "echo -n '> '",
])

// Right prompt
FishRightPrompt::new(&["echo -n (date '+%H:%M'"])

// Vi-mode indicator
FishModePrompt::new(&["echo -n \"[$fish_bind_mode] \""])

// Greeting variants
FishGreeting::disabled()              // set -g fish_greeting
FishGreeting::message("Hello!")       // set -g fish_greeting 'Hello!'
FishGreeting::function(&["echo Hi"]) // function fish_greeting ... end
```

---

### Control flow

```rust
use toconfig::fish::conditional::{FishIf, FishCondition, FishSwitch, FishFor, FishWhile, FishBegin};
use toconfig::fish::variable::FishVariable;

// if / else if / else
FishIf::new(FishCondition::IsInteractive)
    .add(FishVariable::global("EDITOR", "nvim"))
    .add_else(FishVariable::global("EDITOR", "vi"))

// switch / case
FishSwitch::new("$os")
    .case("Linux",  vec![Box::new(FishVariable::global("PLATFORM", "linux"))])
    .case("Darwin", vec![Box::new(FishVariable::global("PLATFORM", "macos"))])

// for loop
FishFor::new("f", &["a.txt", "b.txt"])
    .add(/* ... */)

// while loop
FishWhile::new(FishCondition::Raw("test -f /tmp/lock".into()))
    .add(/* ... */)

// begin block
FishBegin::new()
    .with_comment("Grouped block")
    .add(/* ... */)
```

**`FishCondition` variants:** `IsInteractive`, `IsLogin`, `IsCommandSubstitution`, `IsBlock`, `Raw(expr)`.

---

### Source / plugins

```rust
use toconfig::fish::source::{FishSource, FishPlugin, FishRawLine};

FishSource::new("~/.config/fish/functions.fish")
// → source ~/.config/fish/functions.fish

FishPlugin::new("jorgebucaran/autopair.fish")
// → fisher install jorgebucaran/autopair.fish

// Escape hatch: emit arbitrary Fish code verbatim
FishRawLine::new("zoxide init fish | source")
```

---

### FishConfigTree

```rust
use toconfig::fish::config_tree::FishConfigTree;
use toconfig::Config;

let tree = FishConfigTree::new()
    .with_comment("config.fish — generated by toconfig")
    .add(/* any Config node */)
    .add(/* ... */);

tree.validate_all().expect("validation errors");
println!("{}", tree.generate());
```

Both consuming-builder (`.add`) and mutable-borrow (`.push_node`) patterns are supported.

---

### Fish end-to-end example

```rust
use toconfig::Config;
use toconfig::fish::config_tree::FishConfigTree;
use toconfig::fish::variable::FishVariable;
use toconfig::fish::alias::FishAlias;
use toconfig::fish::abbr::FishAbbr;
use toconfig::fish::path::FishAddPath;
use toconfig::fish::prompt::FishGreeting;
use toconfig::fish::source::FishRawLine;
use toconfig::fish::conditional::{FishIf, FishCondition};
use toconfig::output::ConfigOutput;

fn main() -> std::io::Result<()> {
    let tree = FishConfigTree::new()
        .with_comment("config.fish — generated by toconfig")
        .add(FishGreeting::disabled())
        .add(FishVariable::env("EDITOR", "nvim"))
        .add(FishVariable::env("BROWSER", "firefox"))
        .add(FishAddPath::new(&["$HOME/.local/bin", "$HOME/.cargo/bin"]))
        .add(FishAlias::new("ll",  "ls -la"))
        .add(FishAlias::new("gs",  "git status"))
        .add(FishAbbr::new("gco", "git checkout"))
        .add(FishAbbr::new("gp",  "git push"))
        .add(
            FishIf::new(FishCondition::IsInteractive)
                .add(FishRawLine::new("zoxide init fish | source"))
                .add(FishRawLine::new("starship init fish | source"))
        );

    let out = ConfigOutput::at_path(
        &format!("{}/.config/fish/config.fish", std::env::var("HOME").unwrap())
    );
    out.write(&tree)?;
    Ok(())
}
```

---

## Extending the library

Implement `Config` for any struct to participate in the Neovim / Fish rendering pipeline:

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

// Now use it anywhere in a ConfigTree:
// tree.add(VimLoaderNode);
```

For Hyprland, implement `HyprlandConfig` instead:

```rust
use toconfig::hyprland::core::{HyprlandConfig, HyprlandRenderContext};

pub struct MyHyprNode;

impl HyprlandConfig for MyHyprNode {
    fn render(&self, ctx: &HyprlandRenderContext) -> String {
        format!("{}# custom directive", ctx.indent())
    }
}
```
