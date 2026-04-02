use crate::core::{Config, RenderContext};
use crate::ini::{IniConfig, IniRenderContext};
use std::path::PathBuf;

/// Controls how output is written.
#[derive(Debug, Clone)]
pub enum WriteMode {
    Overwrite,
    Append,
}

/// Manages writing the generated Lua to a file.
pub struct ConfigOutput {
    pub target: PathBuf,
    pub mode: WriteMode,
    pub ldoc: bool, // emit LDoc comments
}

impl ConfigOutput {
    pub fn init_lua() -> Self {
        Self {
            target: PathBuf::from(
                std::env::var("HOME").unwrap_or_else(|_| "/root".into()) + "/.config/nvim/init.lua",
            ),
            mode: WriteMode::Overwrite,
            ldoc: false,
        }
    }

    pub fn at_path(path: &str) -> Self {
        Self {
            target: PathBuf::from(path),
            mode: WriteMode::Overwrite,
            ldoc: false,
        }
    }

    pub fn mode(mut self, m: WriteMode) -> Self {
        self.mode = m;
        self
    }
    pub fn emit_ldoc(mut self, v: bool) -> Self {
        self.ldoc = v;
        self
    }

    /// Render the config tree and write it to the target file.
    /// Performs a diff-check: skips the write if content is unchanged.
    pub fn write<C: Config>(&self, config: &C) -> std::io::Result<bool> {
        let mut ctx = RenderContext::default();
        ctx.emit_doc_comments = self.ldoc;
        let content = config.render(&ctx);

        // Diff check — skip write if unchanged
        if self.target.exists() {
            let existing = std::fs::read_to_string(&self.target)?;
            if existing == content {
                return Ok(false); // no write needed
            }
        }

        if let Some(parent) = self.target.parent() {
            std::fs::create_dir_all(parent)?;
        }

        match self.mode {
            WriteMode::Overwrite => std::fs::write(&self.target, &content)?,
            WriteMode::Append => {
                use std::io::Write;
                let mut f = std::fs::OpenOptions::new()
                    .append(true)
                    .open(&self.target)?;
                writeln!(f, "{}", content)?;
            }
        }
        Ok(true)
    }

    /// Return the generated string without writing (preview mode).
    pub fn preview<C: Config>(&self, config: &C) -> String {
        let mut ctx = RenderContext::default();
        ctx.emit_doc_comments = self.ldoc;
        config.render(&ctx)
    }
}

// ── IniOutput ─────────────────────────────────────────────────────────────────

/// Manages diff-aware writing of INI-format configs (GTK, Qt) to disk.
///
/// # Example
/// ```no_run
/// use toconfig::output::IniOutput;
/// use toconfig::gtk::{GtkSettings, GtkVersion};
///
/// let cfg = GtkSettings::new(GtkVersion::Gtk3).theme("Adwaita");
/// IniOutput::at_path("/tmp/gtk-3.0/settings.ini")
///     .write(&cfg)
///     .unwrap();
/// ```
pub struct IniOutput {
    pub target: PathBuf,
    pub mode: WriteMode,
}

impl IniOutput {
    pub fn at_path(path: &str) -> Self {
        Self {
            target: PathBuf::from(path),
            mode: WriteMode::Overwrite,
        }
    }

    pub fn mode(mut self, m: WriteMode) -> Self {
        self.mode = m;
        self
    }

    /// Render `config` and write it to the target file.
    ///
    /// Performs a diff-check: returns `Ok(false)` and skips the write if the
    /// file already contains identical content.
    pub fn write<C: IniConfig>(&self, config: &C) -> std::io::Result<bool> {
        let content = config.render(&IniRenderContext::default());

        if self.target.exists() {
            let existing = std::fs::read_to_string(&self.target)?;
            if existing == content {
                return Ok(false);
            }
        }

        if let Some(parent) = self.target.parent() {
            std::fs::create_dir_all(parent)?;
        }

        match self.mode {
            WriteMode::Overwrite => std::fs::write(&self.target, &content)?,
            WriteMode::Append => {
                use std::io::Write;
                let mut f = std::fs::OpenOptions::new()
                    .append(true)
                    .open(&self.target)?;
                writeln!(f, "{}", content)?;
            }
        }
        Ok(true)
    }

    /// Return the generated string without writing (preview mode).
    pub fn preview<C: IniConfig>(&self, config: &C) -> String {
        config.render(&IniRenderContext::default())
    }
}
