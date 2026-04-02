# Fish Shell

The `toconfig::fish` module generates Fish shell configuration files from typed Rust structs. All nodes implement the standard `Config` trait and are collected in a `FishConfigTree`.

---

## Table of contents

1. [Variables](#variables)
2. [Aliases](#aliases)
3. [Abbreviations](#abbreviations)
4. [Key bindings](#key-bindings)
5. [Functions](#functions)
6. [Completions](#completions)
7. [PATH management](#path-management)
8. [Colors](#colors)
9. [Prompts & greeting](#prompts--greeting)
10. [Control flow](#control-flow)
11. [Source / plugins](#source--plugins)
12. [FishConfigTree](#fishconfigtree)
13. [End-to-end example](#end-to-end-example)

---

## Variables

```rust
use toconfig::fish::variable::{FishVariable, VarScope};

// Scoped constructors
FishVariable::global("EDITOR", "nvim")       // set -g EDITOR nvim
FishVariable::local("x", "42")               // set -l x 42
FishVariable::universal("theme", "dark")     // set -U theme dark
FishVariable::env("PATH", "/usr/local/bin")  // set -gx PATH /usr/local/bin

// Multi-value list
FishVariable::with_values("colors", &["red", "green", "blue"])
// → set -g colors red green blue
```

Builder methods: `.scope(VarScope::Global)`, `.export(true)`, `.path_list(true)`, `.erase()`.

---

## Aliases

```rust
use toconfig::fish::alias::FishAlias;

FishAlias::new("ll", "ls -la")
// → alias ll 'ls -la'
```

---

## Abbreviations

```rust
use toconfig::fish::abbr::{FishAbbr, AbbrPosition};

// Basic abbreviation
FishAbbr::new("gco", "git checkout")
// → abbr --add gco 'git checkout'

// Expand anywhere in the command line (not just first word)
FishAbbr::new("gs", "git status").anywhere()

// Function-based expansion
FishAbbr::with_function("date", "current_date_fn")
// → abbr --add --function current_date_fn date

// With regex trigger
FishAbbr::new("md", "mkdir -p").regex(r"^mk?d")
```

---

## Key bindings

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

## Functions

```rust
use toconfig::fish::function::{FishFunction, FishEvent};

// Simple function
FishFunction::new("mkcd", &[
    "mkdir -p $argv[1]",
    "and cd $argv[1]",
])
.description("Make a directory and cd into it")
.argument_names(&["dir"]);

// Event-triggered function
FishFunction::new("__on_pwd_change", &["echo PWD changed to $PWD"])
    .on_event(FishEvent::Variable("PWD".into()));

// Wrap another command's completions
FishFunction::new("g", &["git $argv"]).wrap("git");
```

### `FishEvent` variants

| Variant | Trigger |
|---------|---------|
| `Event(name)` | Custom event with name |
| `Variable(name)` | Variable change |
| `ProcessExit(pid)` | Process exits |
| `Signal(name)` | OS signal received |
| `JobExit(pid)` | Job exits |

---

## Completions

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

## PATH management

```rust
use toconfig::fish::path::FishAddPath;

FishAddPath::new(&["$HOME/.local/bin", "/opt/homebrew/bin"])
// → fish_add_path $HOME/.local/bin /opt/homebrew/bin

FishAddPath::new(&["/opt/bin"]).prepend().global()
// → fish_add_path --prepend --global /opt/bin
```

---

## Colors

```rust
use toconfig::fish::color::{FishColor, FishColorVar};

FishColor::new(FishColorVar::Command, "brblue")
// → set -g fish_color_command brblue

FishColor::new(FishColorVar::Comment,        "555 --italics")
FishColor::new(FishColorVar::Autosuggestion, "#6c7086")
```

All `fish_color_*` and `fish_pager_color_*` variables are covered by the `FishColorVar` enum.

---

## Prompts & greeting

```rust
use toconfig::fish::prompt::{FishPrompt, FishRightPrompt, FishModePrompt, FishGreeting};

// Custom left prompt
FishPrompt::new(&[
    "set_color brblue",
    "echo -n (prompt_pwd)",
    "set_color normal",
    "echo -n '> '",
]);

// Right prompt
FishRightPrompt::new(&["echo -n (date '+%H:%M')"]);

// Vi-mode indicator
FishModePrompt::new(&["echo -n \"[$fish_bind_mode] \""]);

// Greeting variants
FishGreeting::disabled()              // set -g fish_greeting (empty)
FishGreeting::message("Hello!")       // set -g fish_greeting 'Hello!'
FishGreeting::function(&["echo Hi"]) // function fish_greeting ... end
```

---

## Control flow

```rust
use toconfig::fish::conditional::{
    FishIf, FishCondition, FishSwitch, FishFor, FishWhile, FishBegin
};
use toconfig::fish::variable::FishVariable;

// if / else if / else
FishIf::new(FishCondition::IsInteractive)
    .add(FishVariable::global("EDITOR", "nvim"))
    .add_else(FishVariable::global("EDITOR", "vi"));

// switch / case
FishSwitch::new("$os")
    .case("Linux",  vec![Box::new(FishVariable::global("PLATFORM", "linux"))])
    .case("Darwin", vec![Box::new(FishVariable::global("PLATFORM", "macos"))]);

// for loop
FishFor::new("f", &["a.txt", "b.txt"])
    .add(/* any Config node */);

// while loop
FishWhile::new(FishCondition::Raw("test -f /tmp/lock".into()))
    .add(/* any Config node */);

// begin block (grouped execution)
FishBegin::new()
    .with_comment("Grouped block")
    .add(/* any Config node */);
```

### `FishCondition` variants

`IsInteractive`, `IsLogin`, `IsCommandSubstitution`, `IsBlock`, `Raw(expr)`.

---

## Source / plugins

```rust
use toconfig::fish::source::{FishSource, FishPlugin, FishRawLine};

// Source a local file
FishSource::new("~/.config/fish/functions.fish")
// → source ~/.config/fish/functions.fish

// Install a plugin via fisher
FishPlugin::new("jorgebucaran/autopair.fish")
// → fisher install jorgebucaran/autopair.fish

// Escape hatch: emit arbitrary Fish code verbatim
FishRawLine::new("zoxide init fish | source")
```

---

## FishConfigTree

```rust
use toconfig::fish::config_tree::FishConfigTree;

let tree = FishConfigTree::new()
    .with_comment("config.fish — generated by toconfig")
    .add(/* any Config node */)
    .add(/* ... */);

tree.validate_all().expect("validation errors");
println!("{}", tree.generate());
```

Both the consuming-builder pattern (`.add`) and the mutable-reference pattern (`.push_node`) are supported.

---

## End-to-end example

```rust
use toconfig::Config;
use toconfig::fish::config_tree::FishConfigTree;
use toconfig::fish::variable::FishVariable;
use toconfig::fish::alias::FishAlias;
use toconfig::fish::abbr::FishAbbr;
use toconfig::fish::path::FishAddPath;
use toconfig::fish::prompt::FishGreeting;
use toconfig::fish::source::FishRawLine;
use toconfig::fish::function::FishFunction;
use toconfig::fish::conditional::{FishIf, FishCondition};
use toconfig::output::ConfigOutput;

fn main() -> std::io::Result<()> {
    let tree = FishConfigTree::new()
        .with_comment("config.fish — generated by toconfig")
        // Greeting
        .add(FishGreeting::disabled())
        // Environment
        .add(FishVariable::env("EDITOR", "nvim"))
        .add(FishVariable::env("BROWSER", "firefox"))
        // PATH
        .add(FishAddPath::new(&["$HOME/.local/bin", "$HOME/.cargo/bin"]))
        // Aliases
        .add(FishAlias::new("ll",  "ls -la"))
        .add(FishAlias::new("gs",  "git status"))
        .add(FishAlias::new("vim", "nvim"))
        // Abbreviations
        .add(FishAbbr::new("gco", "git checkout"))
        .add(FishAbbr::new("gp",  "git push"))
        .add(FishAbbr::new("gpl", "git pull"))
        // Helper function
        .add(
            FishFunction::new("mkcd", &["mkdir -p $argv[1]", "and cd $argv[1]"])
                .description("Make a directory and cd into it"),
        )
        // Interactive-only tools
        .add(
            FishIf::new(FishCondition::IsInteractive)
                .add(FishRawLine::new("zoxide init fish | source"))
                .add(FishRawLine::new("starship init fish | source")),
        );

    let fish_path = format!("{}/.config/fish/config.fish", std::env::var("HOME").unwrap());
    ConfigOutput::at_path(&fish_path).write(&tree)?;
    Ok(())
}
```
