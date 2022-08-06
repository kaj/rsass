use super::{LoadError, Loader, SourceFile, SourceName};
use std::path::{Path, PathBuf};

/// A [`Loader`] that loads files from the filesystem.
#[derive(Debug)]
pub struct FsLoader {
    path: Vec<PathBuf>,
}

impl FsLoader {
    /// Create a new FsFileContext.
    ///
    /// Files will be resolved from the current working directory.
    pub fn for_cwd() -> Self {
        Self {
            path: vec![PathBuf::new()],
        }
    }

    /// Add a path to search for files.
    pub fn push_path(&mut self, path: &Path) {
        self.path.push(path.into());
    }

    /// Create a Loader and a SourceFile from a given Path.
    pub fn for_path(path: &Path) -> Result<(Self, SourceFile), LoadError> {
        let mut f = std::fs::File::open(&path)
            .map_err(|e| LoadError::Input(path.display().to_string(), e))?;
        let (path, name) = if let Some(base) = path.parent() {
            (vec![base.to_path_buf()], path.strip_prefix(base).unwrap())
        } else {
            (vec![PathBuf::new()], path)
        };
        let loader = Self { path };
        let source = SourceName::root(name.display().to_string());
        let source = SourceFile::read(&mut f, source)?;
        Ok((loader, source))
    }
}

impl Loader for FsLoader {
    type File = std::fs::File;

    fn find_file(&self, url: &str) -> Result<Option<Self::File>, LoadError> {
        if !url.is_empty() {
            for base in &self.path {
                let full = base.join(url);
                if full.is_file() {
                    tracing::debug!(?full, "opening file");
                    return Self::File::open(&full)
                        .map_err(|e| {
                            LoadError::Input(full.display().to_string(), e)
                        })
                        .map(Some);
                }
                tracing::trace!(?full, "Not found");
            }
        }
        Ok(None)
    }
}
