# GTK, Qt, and Unified Theme

This page covers three related modules:

- **`toconfig::gtk`** — GTK 3 / GTK 4 `settings.ini` builder
- **`toconfig::qt`** — Qt 5 / Qt 6 qt5ct / qt6ct configuration builder
- **`toconfig::theme`** — Unified `Theme` that drives both backends (and Neovim) from a single definition

All GTK and Qt types implement the `IniConfig` trait and can be written with `IniOutput` (or any generic `ConfigOutput::at_path`).

---

## GTK

### `GtkSettings`

```rust
use toconfig::gtk::{GtkSettings, GtkVersion, HintStyle, RgbaMethod};
use toconfig::ini::IniConfig;

let cfg = GtkSettings::new(GtkVersion::Gtk3)
    .theme("Catppuccin-Mocha-Standard-Blue-Dark")
    .icon_theme("Papirus-Dark")
    .cursor_theme("Bibata-Modern-Ice")
    .cursor_size(24)
    .font("JetBrains Mono 11")          // gtk-font-name
    .prefer_dark(true)                  // gtk-application-prefer-dark-theme
    .antialias(true)                    // Xft.antialias
    .hint_style(HintStyle::Full)        // Xft.hintstyle
    .rgba(RgbaMethod::Rgb);             // Xft.rgba

println!("{}", cfg.generate());
// → [Settings]
// → gtk-theme-name=Catppuccin-Mocha-Standard-Blue-Dark
// → gtk-icon-theme-name=Papirus-Dark
// → ...
```

### `GtkVersion`

| Variant | Config directory | Use case |
|---------|------------------|----------|
| `GtkVersion::Gtk3` | `~/.config/gtk-3.0` | GTK 3 applications |
| `GtkVersion::Gtk4` | `~/.config/gtk-4.0` | GTK 4 applications |

### `HintStyle`

`HintStyle::None`, `HintStyle::Slight`, `HintStyle::Medium`, `HintStyle::Full`.

### `RgbaMethod`

`RgbaMethod::None`, `RgbaMethod::Rgb`, `RgbaMethod::Bgr`, `RgbaMethod::Vrgb`, `RgbaMethod::Vbgr`.

### Writing to disk

```rust
use toconfig::output::ConfigOutput;

let home = std::env::var("HOME").unwrap();
ConfigOutput::at_path(&format!("{}/.config/gtk-3.0/settings.ini", home))
    .write(&GtkSettings::new(GtkVersion::Gtk3).theme("Adwaita-dark"))?;
```

---

## Qt

### `QtConfig`

```rust
use toconfig::qt::{QtConfig, QtVersion, QtStyle, StandardDialogs};
use toconfig::ini::IniConfig;

let cfg = QtConfig::new(QtVersion::Qt5)
    .style(QtStyle::Kvantum)
    .color_scheme("/usr/share/color-schemes/CatppuccinMocha.colors")
    .font_general("Noto Sans, 11")
    .font_fixed("JetBrains Mono, 11")
    .custom_palette(false)
    .standard_dialogs(StandardDialogs::Xdgdesktopportal);

println!("{}", cfg.generate());
// → [Appearance]
// → style=kvantum
// → color_scheme_path=/usr/share/color-schemes/CatppuccinMocha.colors
// → [Fonts]
// → general=Noto Sans, 11
// → fixed=JetBrains Mono, 11
```

### `QtVersion`

| Variant | Config file path |
|---------|-----------------|
| `QtVersion::Qt5` | `~/.config/qt5ct/qt5ct.conf` |
| `QtVersion::Qt6` | `~/.config/qt6ct/qt6ct.conf` |

### `QtStyle`

`QtStyle::Breeze`, `QtStyle::Fusion`, `QtStyle::Kvantum`, `QtStyle::Custom(String)`.

---

## Unified Theme

`Theme` is a single definition that drives GTK, Qt, and Neovim settings from one place — no more duplicating colour names, font sizes, or cursor themes.

### Quick start

```rust
use toconfig::theme::{Theme, ThemePalette, FontConfig, CursorConfig, IconConfig};

let theme = Theme::new("Catppuccin Mocha")
    .palette(ThemePalette::catppuccin_mocha())
    .fonts(FontConfig::new("Noto Sans", 11, "JetBrains Mono", 11))
    .cursor(CursorConfig::new("Bibata-Modern-Ice", 24))
    .icons(IconConfig::new("Papirus-Dark"))
    .gtk_theme("Catppuccin-Mocha-Standard-Blue-Dark")
    .qt_style("kvantum")
    .qt_color_scheme("/usr/share/color-schemes/CatppuccinMocha.colors")
    .neovim_colorscheme("catppuccin");
```

### Generating per-backend configs

```rust
// GTK 3 settings.ini
let gtk3 = theme.apply_gtk(GtkVersion::Gtk3);

// GTK 4 settings.ini
let gtk4 = theme.apply_gtk(GtkVersion::Gtk4);

// Qt5
let qt5 = theme.apply_qt(QtVersion::Qt5);

// Neovim ColorschemeNode
let nvim_cs = theme.apply_neovim();
```

### Built-in palette presets

| Constructor | Description |
|-------------|-------------|
| `ThemePalette::catppuccin_mocha()` | Catppuccin Mocha dark palette |
| `ThemePalette::nord()` | Nord dark palette |
| `ThemePalette::gruvbox_dark()` | Gruvbox dark palette |

### `ThemePalette` colour fields

`base`, `mantle`, `crust`, `surface0`, `surface1`, `surface2`, `overlay0`, `overlay1`, `overlay2`, `subtext0`, `subtext1`, `text`, `rosewater`, `flamingo`, `pink`, `mauve`, `red`, `maroon`, `peach`, `yellow`, `green`, `teal`, `sky`, `sapphire`, `blue`, `lavender`.

---

## End-to-end example

```rust
use toconfig::theme::{Theme, ThemePalette, FontConfig, CursorConfig, IconConfig};
use toconfig::gtk::GtkVersion;
use toconfig::qt::QtVersion;
use toconfig::output::ConfigOutput;

fn main() -> std::io::Result<()> {
    let home = std::env::var("HOME").unwrap();

    let theme = Theme::new("Catppuccin Mocha")
        .palette(ThemePalette::catppuccin_mocha())
        .fonts(FontConfig::new("Noto Sans", 11, "JetBrains Mono", 11))
        .cursor(CursorConfig::new("Bibata-Modern-Ice", 24))
        .icons(IconConfig::new("Papirus-Dark"))
        .gtk_theme("Catppuccin-Mocha-Standard-Blue-Dark")
        .qt_style("kvantum")
        .qt_color_scheme("/usr/share/color-schemes/CatppuccinMocha.colors")
        .neovim_colorscheme("catppuccin");

    // GTK 3
    ConfigOutput::at_path(&format!("{}/.config/gtk-3.0/settings.ini", home))
        .write(&theme.apply_gtk(GtkVersion::Gtk3))?;

    // GTK 4
    ConfigOutput::at_path(&format!("{}/.config/gtk-4.0/settings.ini", home))
        .write(&theme.apply_gtk(GtkVersion::Gtk4))?;

    // Qt 5
    ConfigOutput::at_path(&format!("{}/.config/qt5ct/qt5ct.conf", home))
        .write(&theme.apply_qt(QtVersion::Qt5))?;

    // Qt 6
    ConfigOutput::at_path(&format!("{}/.config/qt6ct/qt6ct.conf", home))
        .write(&theme.apply_qt(QtVersion::Qt6))?;

    Ok(())
}
```
