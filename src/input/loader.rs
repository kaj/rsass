use super::{SourceFile, SourceName};
use crate::Error;
use std::path::{Path, PathBuf};

/// A file context manages finding and loading files.
///
/// # Example
/// ```
/// use std::collections::HashMap;
/// use rsass::{input::Loader, Error};
///
/// #[derive(Clone, Debug)]
/// struct MemoryLoader<'a> {
///     files: HashMap<String, &'a[u8]>,
/// }
///
/// impl<'a> Loader for MemoryLoader<'a> {
///     type File = &'a [u8];
///
///     fn find_file(&self, name: &str) -> Result<Option<Self::File>, Error> {
///         Ok(self.files.get(name).map(|data| *data))
///     }
/// }
/// ```
pub trait Loader: Sized + std::fmt::Debug {
    /// Anything that can be read can be a File in an implementation.
    type File: std::io::Read;

    /// Find a file.
    ///
    /// If a file named `base/input.scss` uses a file named `module`, the
    /// name is converted to `base/module.scss` and variants by
    /// [`Context::find_file`][crate::input::Context::find_file], and
    /// this method is called for each variant to check if it exists.
    ///
    /// Note that if a file with the given name does not exist, that is not
    /// an error.
    /// In that case, `find_file` is expected to return `Ok(None)`.
    /// Things like illegal file names (for the given backend) or lacking
    /// permissions, are handled as errors.
    ///
    /// The official Sass specification prescribes that files are loaded by
    /// url instead of by path to ensure universal compatibility of style sheets.
    /// This effectively mandates the use of forward slashes on all platforms.
    fn find_file(&self, url: &str) -> Result<Option<Self::File>, Error>;
}

/// A [`Loader`] that loads files from the filesystem.
#[derive(Clone, Debug)]
pub struct FsLoader {
    path: Vec<PathBuf>,
}

impl FsLoader {
    /// Create a new FsFileContext.
    ///
    /// Files will be resolved from the current working directory.
    #[allow(clippy::new_without_default)]
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
    pub fn for_path(path: &Path) -> Result<(Self, SourceFile), Error> {
        let mut f = std::fs::File::open(&path)
            .map_err(|e| Error::Input(path.display().to_string(), e))?;
        let (path, name) = if let Some(base) = path.parent() {
            (
                vec![base.to_path_buf(), PathBuf::new()],
                path.strip_prefix(base).unwrap(),
            )
        } else {
            (vec![PathBuf::new()], path)
        };
        let ctx = Self { path };
        let source = SourceName::root(name.display().to_string());
        let source = SourceFile::read(&mut f, source)?;
        Ok((ctx, source))
    }
}

impl Loader for FsLoader {
    type File = std::fs::File;

    fn find_file(&self, name: &str) -> Result<Option<Self::File>, Error> {
        if !name.is_empty() {
            for base in &self.path {
                let full = base.join(name);
                if full.is_file() {
                    tracing::debug!(?full, "opening file");
                    return match Self::File::open(&full) {
                        Ok(file) => Ok(Some(file)),
                        Err(e) => {
                            Err(Error::Input(full.display().to_string(), e))
                        }
                    };
                }
                tracing::trace!(?full, "Not found");
            }
        }
        Ok(None)
    }
}
