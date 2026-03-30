# toconfig — Documentation

`toconfig` is a Rust library for **programmatically generating configuration files** for a wide range of Linux tools. It models every configuration as typed Rust structs, uses a fluent / method-chaining API, and produces valid, indented output from a single `.generate()` call.

---

## Guides

| Guide | Description |
|-------|-------------|
| [Getting started](getting-started.md) | Installation, crate setup, and your first generated config |
| [Core concepts](core.md) | `Config` trait, `RenderContext`, `ConfigTree`, `LuaValue` |
| [Output](output.md) | Writing files, diff-checking, preview mode |
| [Extending the library](extending.md) | Implementing custom `Config` nodes |

## Module reference

| Module | Doc page | What it generates |
|--------|----------|-------------------|
| `neovim` | [neovim.md](neovim.md) | `~/.config/nvim/init.lua` |
| `hyprland` | [hyprland.md](hyprland.md) | `~/.config/hypr/hyprland.conf` |
| `fish` | [fish.md](fish.md) | `~/.config/fish/config.fish` |
| `helix` | [helix.md](helix.md) | `~/.config/helix/config.toml` |
| `waybar` | [waybar.md](waybar.md) | `~/.config/waybar/config` |
| `systemd` | [systemd.md](systemd.md) | `.service`, `.timer`, `.socket`, … unit files |
| `fstab` | [fstab.md](fstab.md) | `/etc/fstab` |
| `gtk` / `qt` / `theme` | [gtk-qt-theme.md](gtk-qt-theme.md) | GTK/Qt `settings.ini`, unified theme |
| `fontconfig` | [fontconfig.md](fontconfig.md) | `~/.config/fontconfig/fonts.conf` |

---

## Quick-look architecture

```
toconfig
├── core          — Config trait + RenderContext + ConfigTree
├── lua           — LuaValue serialiser + RawLua escape hatch
├── output        — ConfigOutput / IniOutput: write / preview / diff-check
├── ini           — IniConfig trait + IniSection / IniFile
├── neovim        — options, keymap, autocmd, command, theme, plugins, profile
├── hyprland      — variable, env, monitor, exec, bind, window_rule, workspace,
│                   animation, layout, section, xwayland, permission, output
├── fish          — variable, alias, abbr, bind, function, completion, path,
│                   color, prompt, conditional, source, config_tree
├── helix         — editor, keys, section, theme
├── waybar        — bar, clock, battery, cpu, memory, network, pulseaudio,
│                   backlight
├── systemd       — unit_section, service, socket, timer, mount, automount,
│                   path, swap, install, drop_in
├── fstab         — core (FstabEntry, Fstab), options
├── gtk           — GtkSettings
├── qt            — QtConfig
├── theme         — Theme, ThemePalette, FontConfig, CursorConfig, IconConfig
└── fontconfig    — alias, match_rule, dir, select, value, description
```
