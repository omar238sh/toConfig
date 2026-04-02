# Output

The `toconfig::output` module provides helpers for writing generated configuration to disk (or just previewing it as a string) with a built-in **diff-check** that skips the write when the file content is unchanged.

---

## `ConfigOutput` (Lua / text)

Used with any node that implements `Config` (Neovim, Fish, etc.).

### Constructors

| Constructor | Target path |
|-------------|-------------|
| `ConfigOutput::init_lua()` | `~/.config/nvim/init.lua` |
| `ConfigOutput::at_path(p)` | Any custom absolute or `~`-expanded path |

```rust
use toconfig::output::{ConfigOutput, WriteMode};

// Write to the default Neovim init.lua location
let out = ConfigOutput::init_lua();

// Write to a custom path
let out = ConfigOutput::at_path("/tmp/test_init.lua");
```

### Options

| Method | Description |
|--------|-------------|
| `.emit_ldoc(true)` | Prepend `doc_comment()` strings as comments above each node |
| `.mode(WriteMode::Append)` | Append instead of overwriting (default: `WriteMode::Overwrite`) |

### Writing and previewing

```rust
// Write to file; returns Ok(true) if the file was updated, Ok(false) if skipped (no change)
let was_updated: bool = out.write(&my_tree)?;

// Preview: render to a String without any I/O
let preview: String = out.preview(&my_tree);
```

### Full example

```rust
use toconfig::output::ConfigOutput;

let written = ConfigOutput::init_lua()
    .emit_ldoc(true)
    .write(&tree)
    .expect("failed to write init.lua");

if written {
    println!("init.lua updated");
} else {
    println!("init.lua unchanged, skipping write");
}
```

---

## `HyprlandOutput`

Used with `HyprlandConfigTree` nodes.

```rust
use toconfig::hyprland::output::HyprlandOutput;

// Write to ~/.config/hypr/hyprland.conf (default)
HyprlandOutput::hyprland_conf().write(&hypr_tree)?;

// Custom path
HyprlandOutput::at_path("/tmp/test.conf").write(&hypr_tree)?;

// Preview only
let text = HyprlandOutput::hyprland_conf().preview(&hypr_tree);
```

---

## `IniOutput` (GTK / Qt)

Used with any node that implements `IniConfig`.

```rust
use toconfig::output::IniOutput;

// Write GTK settings.ini
IniOutput::at_path("~/.config/gtk-3.0/settings.ini")
    .write(&gtk_settings)?;

// Preview
let text = IniOutput::at_path("~/.config/qt5ct/qt5ct.conf")
    .preview(&qt_config);
```

---

## Diff-check behaviour

All output helpers check whether the destination file already exists and contains content byte-for-byte identical to the new output. If so:

- The file is **not** touched (preserves mtime, avoids unnecessary re-runs of tools that watch the file).
- `write()` returns `Ok(false)`.

This makes `toconfig` safe to call on every application start or from a systemd timer without causing unnecessary churn.
