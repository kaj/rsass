use std::path::{Path, PathBuf};

/// A file context specifies where to find files to load.
///
/// When opening an included file, an extended file context is
/// created, to find further included files relative to the file they
/// are inlcuded from.
///
/// # Example
/// ```
/// use rsass::FileContext;
///
/// let base = FileContext::new();
/// let (base, file1) = base.file("some/dir/file.scss".as_ref());
/// // base is now a relative to file1, usefull to open files
/// // by paths mentioned in file1.
/// let (base, file2) = base.file("some/other.scss".as_ref());
/// assert_eq!(file1.to_string_lossy(), "some/dir/file.scss");
/// assert_eq!(file2.to_string_lossy(), "some/dir/some/other.scss");
/// ```
#[derive(Clone, Debug)]
pub struct FileContext {
    path: PathBuf,
}

impl FileContext {
    /// Create a new FileContext.
    ///
    /// Files will be resolved from the current working directory.
    pub fn new() -> Self {
        FileContext {
            path: PathBuf::new(),
        }
    }
    /// Get a file from this context.
    ///
    /// Get a path and a FileContext from this FileContext and a path.
    pub fn file(&self, file: &Path) -> (Self, PathBuf) {
        let t = self.path.join(file);
        if let Some(dir) = t.parent() {
            (
                FileContext {
                    path: PathBuf::from(dir),
                },
                t.clone(),
            )
        } else {
            (FileContext::new(), t.clone())
        }
    }

    pub fn find_file(&self, name: &Path) -> Option<(Self, PathBuf)> {
        // TODO Check docs what expansions should be tried!
        let parent = name.parent();
        if let Some(name) = name.file_name().and_then(|n| n.to_str()) {
            for name in
                &[name, &format!("{}.scss", name), &format!("_{}.scss", name)]
            {
                let full =
                    parent.map(|p| p.join(name)).unwrap_or_else(|| name.into());
                let (c, p) = self.file(&full);
                if p.is_file() {
                    return Some((c, p));
                }
            }
        }
        None
    }
}
