# Fontconfig

The `toconfig::fontconfig` module generates `~/.config/fontconfig/fonts.conf` (or any other fontconfig XML file) from typed Rust structs. Nodes implement `FontconfigConfig` and are collected in a `FontconfigDocument`, which renders a complete, well-formed XML file with the `<?xml …?>` declaration and `<fontconfig>` root element.

The module is **intentionally separate** from the Neovim `Config` and Hyprland `HyprlandConfig` trait families — nodes cannot be accidentally mixed.

---

## Table of contents

1. [FontconfigDocument](#fontconfigdocument)
2. [Description](#description)
3. [Dir, CacheDir, Include](#dir-cachedir-include)
4. [Alias](#alias)
5. [Match rules](#match-rules)
6. [SelectFont](#selectfont)
7. [End-to-end example](#end-to-end-example)

---

## FontconfigDocument

```rust
use toconfig::fontconfig::{FontconfigConfig, FontconfigDocument};

let doc = FontconfigDocument::new()
    .push(/* any FontconfigConfig node */)
    .push(/* ... */);

let xml = doc.generate();
// → <?xml version="1.0"?>
// → <!DOCTYPE fontconfig SYSTEM "fonts.dtd">
// → <fontconfig>
// →   ...
// → </fontconfig>
```

---

## Description

```rust
use toconfig::fontconfig::description::Description;

Description::new("Personal font preferences")
// → <description>Personal font preferences</description>
```

---

## Dir, CacheDir, Include

```rust
use toconfig::fontconfig::dir::{Dir, CacheDir, Include};

// Font search directory
Dir::new("~/.local/share/fonts")
// → <dir>~/.local/share/fonts</dir>

// XDG-relative directory (prefix="xdg")
Dir::new("fonts").prefix("xdg")
// → <dir prefix="xdg">fonts</dir>

// Cache location
CacheDir::new("~/.cache/fontconfig")
// → <cachedir>~/.cache/fontconfig</cachedir>

// Include another conf file or directory
Include::new("conf.d").ignore_missing(true)
// → <include ignore_missing="yes">conf.d</include>
```

---

## Alias

`Alias` defines family substitution rules — which fonts to prefer or accept as fallbacks for a given generic family.

```rust
use toconfig::fontconfig::alias::Alias;

Alias::new("sans-serif")
    .prefer(["Noto Sans"])
    .accept(["DejaVu Sans"])
// → <alias>
// →   <family>sans-serif</family>
// →   <prefer><family>Noto Sans</family></prefer>
// →   <accept><family>DejaVu Sans</family></accept>
// → </alias>

Alias::new("monospace")
    .prefer(["JetBrains Mono", "Noto Mono"])
```

---

## Match rules

`Match` is the most powerful fontconfig construct — it performs conditional edits on font patterns or font sets.

```rust
use toconfig::fontconfig::match_rule::{Match, MatchTarget, Test, Edit, EditMode, EditBinding};
use toconfig::fontconfig::value::FontconfigValue;

// Substitute Helvetica → Noto Sans
Match::new()
    .target(MatchTarget::Pattern)
    .test(Test::new("family", FontconfigValue::string("Helvetica")))
    .edit(
        Edit::new("family", FontconfigValue::string("Noto Sans"))
            .mode(EditMode::Prepend)
            .binding(EditBinding::Strong),
    )
```

### `MatchTarget` variants

| Variant | Description |
|---------|-------------|
| `MatchTarget::Pattern` | Applied to the font request pattern |
| `MatchTarget::Font` | Applied to each matching font in the font list |
| `MatchTarget::Scan` | Applied while scanning font files |

### `EditMode` variants

`Prepend`, `Append`, `Replace`, `PrependFirst`, `AppendLast`, `Delete`, `DeleteAll`.

### `EditBinding` variants

`Strong`, `Weak`, `Same`.

### `FontconfigValue` constructors

| Constructor | XML output |
|-------------|------------|
| `FontconfigValue::string(s)` | `<string>s</string>` |
| `FontconfigValue::integer(n)` | `<integer>n</integer>` |
| `FontconfigValue::double(f)` | `<double>f</double>` |
| `FontconfigValue::bool(b)` | `<bool>true/false</bool>` |
| `FontconfigValue::constant(s)` | `<const>s</const>` |

---

## SelectFont

`SelectFont` accepts or rejects fonts from the font list based on pattern or glob tests.

```rust
use toconfig::fontconfig::select::{SelectFont, SelectAction, Glob};

// Reject bitmap (non-scalable) fonts in misc directory
SelectFont::new().block(
    SelectAction::Reject,
    vec![],                                               // pattern tests
    vec![Glob::new("/usr/share/fonts/misc/*")],           // glob tests
)
// → <selectfont>
// →   <rejectfont>
// →     <glob>/usr/share/fonts/misc/*</glob>
// →   </rejectfont>
// → </selectfont>

// Accept only specific paths
SelectFont::new().block(
    SelectAction::Accept,
    vec![],
    vec![Glob::new("~/.local/share/fonts/*")],
)
```

---

## End-to-end example

```rust
use toconfig::fontconfig::{FontconfigConfig, FontconfigDocument};
use toconfig::fontconfig::description::Description;
use toconfig::fontconfig::dir::{Dir, CacheDir, Include};
use toconfig::fontconfig::alias::Alias;
use toconfig::fontconfig::match_rule::{Match, MatchTarget, Test, Edit, EditMode, EditBinding};
use toconfig::fontconfig::value::FontconfigValue;
use toconfig::fontconfig::select::{SelectFont, SelectAction, Glob};
use toconfig::output::ConfigOutput;

fn main() -> std::io::Result<()> {
    let doc = FontconfigDocument::new()
        .push(Description::new("Personal font preferences"))
        // Font search paths
        .push(Dir::new("~/.local/share/fonts"))
        .push(Dir::new("fonts").prefix("xdg"))
        // Cache location
        .push(CacheDir::new("~/.cache/fontconfig"))
        // Include distro defaults
        .push(Include::new("conf.d").ignore_missing(true))
        // Alias sans-serif to Noto Sans with a DejaVu fallback
        .push(
            Alias::new("sans-serif")
                .prefer(["Noto Sans"])
                .accept(["DejaVu Sans"]),
        )
        .push(
            Alias::new("monospace")
                .prefer(["JetBrains Mono", "Noto Mono"]),
        )
        // Substitute Helvetica → Noto Sans
        .push(
            Match::new()
                .target(MatchTarget::Pattern)
                .test(Test::new("family", FontconfigValue::string("Helvetica")))
                .edit(
                    Edit::new("family", FontconfigValue::string("Noto Sans"))
                        .mode(EditMode::Prepend)
                        .binding(EditBinding::Strong),
                ),
        )
        // Reject bitmap fonts
        .push(
            SelectFont::new().block(
                SelectAction::Reject,
                vec![],
                vec![Glob::new("/usr/share/fonts/misc/*")],
            ),
        );

    let home = std::env::var("HOME").unwrap();
    ConfigOutput::at_path(&format!("{}/.config/fontconfig/fonts.conf", home))
        .write(&doc)?;
    Ok(())
}
```
