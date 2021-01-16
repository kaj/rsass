use crate::error::Error;
use std::path::{Path, PathBuf};


/// A file context manages finding and loading files.
///
/// # Example
/// ```
/// use std::collections::HashMap;
/// use rsass::{FileContext, Error};
///
/// #[derive(Clone, Debug)]
/// struct StaticFileContext<'a> {
///     files: HashMap<String, &'a[u8]>,
/// }
///
/// impl<'a> FileContext for StaticFileContext<'a> {
///     type File = &'a [u8];
///
///     fn find_file(
///         &self, name: &str
///     ) -> Result<Option<(Self, String, Self::File)>, Error> {
///         if let Some(file) = self.files.get(name).map(|data| *data) {
///             Ok(Some((self.clone(), name.to_string(), file)))
///         } else {
///             Ok(None)
///         }
///     }
/// }
/// ```
pub trait FileContext: Sized + std::fmt::Debug {
    type File: std::io::Read;

    /// Find a file.
    ///
    /// If the file is imported from another file,
    /// the argument is the exact string specified in the import declaration.
    ///
    /// The official Sass spec prescribes that files are loaded by
    /// url instead of by path to ensure universal compatibility of style sheets.
    /// This effectively mandates the use of forward slashes on all platforms.
    fn find_file(
        &self, url: &str
    ) -> Result<Option<(Self, String, Self::File)>, Error>;
}


/// A filesystem file context specifies where to find local files.
///
/// When opening an included file, an extended file context is
/// created, to find further included files relative to the file they
/// are inlcuded from.
///
/// # Example
/// ```
/// use rsass::FsFileContext;
/// use std::path::PathBuf;
///
/// let base = FsFileContext::new();
/// let (base, file1) =
///     base.file(&PathBuf::from("some").join("dir").join("file.scss"));
/// // base is now a relative to file1, usefull to open files
/// // by paths mentioned in file1.
/// let (base, file2) = base.file("some/other.scss".as_ref());
/// assert_eq!(file1, PathBuf::from("some").join("dir").join("file.scss"));
/// assert_eq!(file2, PathBuf::from("some").join("dir").join("some/other.scss"));
/// ```
#[derive(Clone, Debug)]
pub struct FsFileContext {
    path: Vec<PathBuf>,
}

impl FsFileContext {
    /// Create a new FsFileContext.
    ///
    /// Files will be resolved from the current working directory.
    pub fn new() -> Self {
        Self {
            path: vec![PathBuf::new()],
        }
    }

    pub fn push_path(&mut self, path: &Path) {
        self.path.push(path.into());
    }

    /// Get a file from this context.
    ///
    /// Get a path and a FsFileContext from this FsFileContext and a path.
    pub fn file(&self, file: &Path) -> (Self, PathBuf) {
        let t = self.path[0].join(file);
        let mut path = vec![];
        if let Some(dir) = t.parent() {
            path.push(PathBuf::from(dir));
        }
        path.extend_from_slice(&self.path);
        (Self { path }, t)
    }
}

impl FileContext for FsFileContext {
    type File = std::fs::File;

    fn find_file(
        &self, name: &str
    ) -> Result<Option<(Self, String, Self::File)>, crate::Error> {
        // TODO Check docs what expansions should be tried!
        // Files with .sass extension needs another parser.
        let name = Path::new(name);
        let parent = name.parent();
        if let Some(name) = name.file_name().and_then(|n| n.to_str()) {
            for base in &self.path {
                for name in &[
                    name,
                    &format!("{}.scss", name),
                    &format!("_{}.scss", name),
                    &format!("{}/index.scss", name),
                    &format!("{}/_index.scss", name),
                ] {
                    let full = if let Some(parent) = parent {
                        base.join(parent).join(name)
                    } else {
                        base.join(name)
                    };
                    if full.is_file() {
                        let c = if let Some(parent) = parent {
                            let mut path = vec![];
                            path.push(PathBuf::from(parent));
                            path.extend_from_slice(&self.path);
                            Self { path }
                        } else {
                            self.clone()
                        };
                        let path = full.display().to_string();
                        let file = Self::File::open(full.clone())?;
                        return Ok(Some((c, path, file)));
                    }
                }
            }
        }
        Ok(None)
    }
}
