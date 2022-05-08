use crate::{Error, SourceFile, SourceName, SourcePos};
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
///     ) -> Result<Option<(String, Self::File)>, Error> {
///         if let Some(file) = self.files.get(name).map(|data| *data) {
///             Ok(Some((name.to_string(), file)))
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
    #[tracing::instrument]
    fn find_file_import(
        &self,
        url: &str,
        from: SourcePos,
    ) -> Result<Option<SourceFile>, Error> {
        let names: &[&dyn Fn(&str, &str) -> String] = &[
            // base will either be empty or end with a slash.
            &|base, name| format!("{}{}.scss", base, name),
            &|base, name| format!("{}_{}.scss", base, name),
            &|base, name| format!("{}{}.import.scss", base, name),
            &|base, name| format!("{}_{}.import.scss", base, name),
            &|base, name| format!("{}{}/index.scss", base, name),
            &|base, name| format!("{}{}/_index.scss", base, name),
            &|base, name| format!("{}{}.css", base, name),
        ];
        // Note: Should a "full stack" of bases be used here?
        // Or is this fine?
        let base = from.file_url();
        if let Some((path, mut file)) = base
            .rfind('/')
            .map(|p| base.split_at(p + 1).0)
            .map(|base| {
                do_find_file(self, &format!("{}{}", base, url), names)
            })
            .unwrap_or_else(|| do_find_file(self, url, names))?
        {
            let source = SourceName::imported(path, from);
            Ok(Some(SourceFile::read(&mut file, source)?))
        } else {
            Ok(None)
        }
    }

    /// Find a file for `@use`
    #[tracing::instrument]
    fn find_file_use(
        &self,
        url: &str,
        from: SourcePos,
    ) -> Result<Option<SourceFile>, Error> {
        let names: &[&dyn Fn(&str, &str) -> String] = &[
            // base will either be empty or end with a slash.
            &|base, name| format!("{}{}.scss", base, name),
            &|base, name| format!("{}_{}.scss", base, name),
            &|base, name| format!("{}{}/index.scss", base, name),
            &|base, name| format!("{}{}/_index.scss", base, name),
            &|base, name| format!("{}{}.css", base, name),
        ];
        // Note: Should a "full stack" of bases be used here?
        // Or is this fine?
        let base = from.file_url();
        if let Some((path, mut file)) = base
            .rfind('/')
            .map(|p| base.split_at(p + 1).0)
            .map(|base| {
                do_find_file(self, &format!("{}{}", base, url), names)
            })
            .unwrap_or_else(|| do_find_file(self, url, names))?
        {
            let source = SourceName::used(path, from);
            Ok(Some(SourceFile::read(&mut file, source)?))
        } else {
            Ok(None)
        }
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
    ) -> Result<Option<(String, Self::File)>, Error>;
}

/// Find a file for `@use`
fn do_find_file<FC: FileContext>(
    ctx: &FC,
    url: &str,
    names: &[&dyn Fn(&str, &str) -> String],
) -> Result<Option<(String, FC::File)>, Error> {
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

    /// Create a FsFilecontext and a SourceFile from a given Path.
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

impl FileContext for FsFileContext {
    type File = std::fs::File;

    fn find_file(
        &self,
        name: &str,
    ) -> Result<Option<(String, Self::File)>, Error> {
        // TODO: Use rsplit_once when MSRV is 1.52 or above.
        let (parent, name) = if let Some(pos) = name.find('/') {
            (Some(&name[..pos + 1]), &name[pos + 1..])
        } else {
            (None, name)
        };
        if !name.is_empty() {
            for base in &self.path {
                use std::fmt::Write;
                let mut full = String::new();
                if !base.as_os_str().is_empty() {
                    write!(&mut full, "{}/", base.display()).unwrap();
                }
                if let Some(parent) = parent {
                    full.push_str(parent);
                }
                full.push_str(name);
                if Path::new(&full).is_file() {
                    tracing::debug!(?full, "opening file");
                    return match Self::File::open(&full) {
                        Ok(file) => Ok(Some((full, file))),
                        Err(e) => Err(Error::Input(full, e)),
                    };
                }
                tracing::trace!(?full, "Not found");
            }
        }
        Ok(None)
    }
}
