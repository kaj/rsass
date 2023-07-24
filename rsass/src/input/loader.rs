use std::fmt;

/// A file context manages finding and loading files.
///
/// # Example
/// ```
/// use std::collections::HashMap;
/// use rsass::input::{Loader, LoadError};
///
/// #[derive(Clone, Debug)]
/// struct MemoryLoader<'a> {
///     files: HashMap<String, &'a[u8]>,
/// }
///
/// impl<'a> Loader for MemoryLoader<'a> {
///     type File = &'a [u8];
///
///     fn find_file(&self, name: &str) -> Result<Option<Self::File>, LoadError> {
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
    fn find_file(&self, url: &str) -> Result<Option<Self::File>, LoadError>;
}

/// An error loading a file.
#[non_exhaustive]
pub enum LoadError {
    /// Reading {0} failed: {1}
    Input(String, std::io::Error),
    /// {0} is not a css or sass file.
    UnknownFormat(String),
    /// Expected a cargo environment, but none found.
    NotCalledFromCargo,
}
impl std::error::Error for LoadError {}

impl fmt::Display for LoadError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(out, "Error: {self:?}")
    }
}

impl fmt::Debug for LoadError {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoadError::Input(path, err) => {
                write!(out, "Reading {path:?} failed: {err}")
            }
            LoadError::UnknownFormat(name) => {
                write!(out, "{name:?} is not a css or sass file.")
            }
            LoadError::NotCalledFromCargo => {
                write!(out, "Expected a cargo environment, but none found.")
            }
        }
    }
}
