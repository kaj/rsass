use super::{LoadError, Loader, SourceFile, SourceName};
use std::path::{Path, PathBuf};

/// A [`Loader`] for when calling rsass from a `build.rs` script.
///
/// This is very similar to a [`FsLoader`][super::FsLoader], but has a
/// `for_crate` constructor that uses the `CARGO_MANIFEST_DIR`
/// environment variable instead of the current working directory, and
/// it prints `cargo:rerun-if-changed` messages for each path that it
/// loads.
#[derive(Debug)]
pub struct CargoLoader {
    path: Vec<PathBuf>,
}

impl CargoLoader {
    /// Create a new `FsFileContext`.
    ///
    /// Files will be resolved from the directory containing the
    /// manifest of your package.
    /// This assumes the program is called by `cargo`, so the
    /// `CARGO_MANIFEST_DIR` environment variable is set.
    pub fn for_crate() -> Result<Self, LoadError> {
        Ok(Self {
            path: vec![get_pkg_base()?],
        })
    }

    /// Add a path to search for files.
    ///
    /// The path can be relative to the crate manifest directory, or
    /// absolute.
    pub fn push_path(&mut self, path: &Path) -> Result<(), LoadError> {
        self.path.push(if path.is_absolute() {
            path.into()
        } else {
            get_pkg_base()?.join(path)
        });
        Ok(())
    }

    /// Create a loader and a `SourceFile` from a given `Path`.
    ///
    /// The path can be relative to the crate manifest directory, or
    /// absolute.
    pub fn for_path(path: &Path) -> Result<(Self, SourceFile), LoadError> {
        let path = if path.is_absolute() {
            path.into()
        } else {
            get_pkg_base()?.join(path)
        };
        let mut f = std::fs::File::open(&path)
            .map_err(|e| LoadError::Input(path.display().to_string(), e))?;
        cargo_watch(&path);
        let (path, name) = if let Some(base) = path.parent() {
            (vec![base.to_path_buf()], path.strip_prefix(base).unwrap())
        } else {
            (vec![get_pkg_base()?], path.as_ref())
        };
        let loader = Self { path };
        let source = SourceName::root(name.display().to_string());
        let source = SourceFile::read(&mut f, source)?;
        Ok((loader, source))
    }
}

impl Loader for CargoLoader {
    type File = std::fs::File;

    fn find_file(&self, url: &str) -> Result<Option<Self::File>, LoadError> {
        if !url.is_empty() {
            for base in &self.path {
                let full = base.join(url);
                if full.is_file() {
                    tracing::debug!(?full, "opening file");
                    let file = Self::File::open(&full).map_err(|e| {
                        LoadError::Input(full.display().to_string(), e)
                    })?;
                    cargo_watch(&full);
                    return Ok(Some(file));
                }
                tracing::trace!(?full, "Not found");
            }
        }
        Ok(None)
    }
}

/// Tell cargo to recompile if the file on `path` changes.
fn cargo_watch(path: &Path) {
    println!("cargo:rerun-if-changed={}", path.display());
}

/// Get the package base dir.
///
/// This returns the `CARGO_MANIFEST_DIR` environment variable as a path.
/// If the env is not set, an error is returned.
fn get_pkg_base() -> Result<PathBuf, LoadError> {
    std::env::var_os("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .ok_or(LoadError::NotCalledFromCargo)
}
