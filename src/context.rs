use anyhow::{bail, Result};
use ignore::WalkBuilder;
use serde::Serialize;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct Sandbox {
    root: PathBuf,
}

impl Sandbox {
    pub fn new(root: PathBuf) -> Self { Self { root } }
    pub fn root(&self) -> &Path { &self.root }

    pub fn assert_inside(&self, p: &Path) -> Result<()> {
        let abs = if p.is_absolute() { p.to_path_buf() } else { self.root.join(p) };
        let abs = dunce::canonicalize(abs).unwrap_or_else(|_| self.root.join(p));
        let root = dunce::canonicalize(&self.root).unwrap_or(self.root.clone());
        if !abs.starts_with(&root) { bail!("path escapes sandbox: {}", abs.display()); }
        Ok(())
    }

    pub fn build_walker(&self) -> WalkBuilder {
        let mut w = WalkBuilder::new(&self.root);
        w.hidden(true).git_ignore(true).git_global(true).git_exclude(true);
        // Respect .rooignore if present (same semantics as .gitignore)
        if self.root.join(".rooignore").exists() {
            w.add_custom_ignore_filename(".rooignore");
        }
        w
    }
}

#[derive(Clone, Copy)]
pub struct Approval { pub auto_yes: bool }

impl Approval {
    pub fn confirm(&self, reason: &str) -> Result<bool> {
        if self.auto_yes { return Ok(true); }
        println!("Confirm: {} [y/N] ", reason);
        print!("> "); io::stdout().flush().ok();
        let mut ans = String::new();
        io::stdin().read_line(&mut ans)?;
        Ok(matches!(ans.trim().to_ascii_lowercase().as_str(), "y" | "yes"))
    }
}

#[derive(Debug, Serialize)]
pub struct FileHit { pub path: String }

// Small helpers
pub fn read_text(path: &Path) -> Result<String> { Ok(fs::read_to_string(path)?) }
pub fn write_text(path: &Path, content: &str) -> Result<()> { Ok(fs::write(path, content)?) }

