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
    /// Anything that can be read can be a File in an implementation.
    type File: std::io::Read;

    /// Find a file for `@import`
    ///
    /// This includes "import-only" filenames, otherwise the same as [`#find_file_use`].
    fn find_file_import(
        &self,
        url: &str,
    ) -> Result<Option<(Self, String, Self::File)>, Error> {
        do_find_file(
            self,
            url,
            &[
                // base will either be empty or end with a slash.
                &|base, name| format!("{}{}.scss", base, name),
                &|base, name| format!("{}_{}.scss", base, name),
                &|base, name| format!("{}{}.import.scss", base, name),
                &|base, name| format!("{}_{}.import.scss", base, name),
                &|base, name| format!("{}{}/index.scss", base, name),
                &|base, name| format!("{}{}/_index.scss", base, name),
            ],
        )
    }

    /// Find a file for `@use`
    fn find_file_use(
        &self,
        url: &str,
    ) -> Result<Option<(Self, String, Self::File)>, Error> {
        do_find_file(
            self,
            url,
            &[
                // base will either be empty or end with a slash.
                &|base, name| format!("{}{}.scss", base, name),
                &|base, name| format!("{}_{}.scss", base, name),
                &|base, name| format!("{}{}/index.scss", base, name),
                &|base, name| format!("{}{}/_index.scss", base, name),
            ],
        )
    }

    /// Find a file.
    ///
    /// If the file is imported from another file,
    /// the argument is the exact string specified in the import declaration.
    ///
    /// The official Sass spec prescribes that files are loaded by
    /// url instead of by path to ensure universal compatibility of style sheets.
    /// This effectively mandates the use of forward slashes on all platforms.
    fn find_file(
        &self,
        url: &str,
    ) -> Result<Option<(Self, String, Self::File)>, Error>;
}

/// Find a file for `@use`
fn do_find_file<FC: FileContext>(
    ctx: &FC,
    url: &str,
    names: &[&dyn Fn(&str, &str) -> String],
) -> Result<Option<(FC, String, FC::File)>, Error> {
    if let Some(result) = ctx.find_file(url)? {
        return Ok(Some(result));
    }

    let (base, name) = url
        .rfind('/')
        .map(|p| url.split_at(p + 1))
        .unwrap_or(("", url));

    for name in names.iter().map(|f| f(base, name)) {
        if let Some(result) = ctx.find_file(&name)? {
            return Ok(Some(result));
        }
    }
    Ok(None)
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
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            path: vec![PathBuf::new()],
        }
    }

    /// Add a path to search for files.
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
        &self,
        name: &str,
    ) -> Result<Option<(Self, String, Self::File)>, crate::Error> {
        // TODO Check docs what expansions should be tried!
        // Files with .sass extension needs another parser.
        let name = Path::new(name);
        let parent = name.parent();
        if let Some(name) = name.file_name().and_then(|n| n.to_str()) {
            for base in &self.path {
                let full = if let Some(parent) = parent {
                    base.join(parent).join(name)
                } else {
                    base.join(name)
                };
                if full.is_file() {
                    let c = if let Some(parent) = parent {
                        let mut path = vec![PathBuf::from(parent)];
                        path.extend_from_slice(&self.path);
                        Self { path }
                    } else {
                        self.clone()
                    };
                    let path = full.display().to_string();
                    return match Self::File::open(full) {
                        Ok(file) => Ok(Some((c, path, file))),
                        Err(e) => Err(Error::Input(path, e)),
                    };
                }
            }
        }
        Ok(None)
    }
}
