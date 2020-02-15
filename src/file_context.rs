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
/// use std::path::PathBuf;
///
/// let base = FileContext::new();
/// let (base, file1) =
///     base.file(&PathBuf::from("some").join("dir").join("file.scss"));
/// // base is now a relative to file1, usefull to open files
/// // by paths mentioned in file1.
/// let (base, file2) = base.file("some/other.scss".as_ref());
/// assert_eq!(file1, PathBuf::from("some").join("dir").join("file.scss"));
/// assert_eq!(file2, PathBuf::from("some").join("dir").join("some/other.scss"));
/// ```
#[derive(Clone, Debug)]
pub struct FileContext {
    path: Vec<PathBuf>,
}

impl FileContext {
    /// Create a new FileContext.
    ///
    /// Files will be resolved from the current working directory.
    pub fn new() -> Self {
        FileContext {
            path: vec![PathBuf::new()],
        }
    }

    pub fn push_path(&mut self, path: &Path) {
        self.path.push(path.into());
    }

    /// Get a file from this context.
    ///
    /// Get a path and a FileContext from this FileContext and a path.
    pub fn file(&self, file: &Path) -> (Self, PathBuf) {
        let t = self.path[0].join(file);
        let mut path = vec![];
        if let Some(dir) = t.parent() {
            path.push(PathBuf::from(dir));
        }
        path.extend_from_slice(&self.path);
        (FileContext { path }, t)
    }

    pub fn find_file(&self, name: &Path) -> Option<(Self, PathBuf)> {
        // TODO Check docs what expansions should be tried!
        // Files with .sass extension needs another parser.
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
                            FileContext { path }
                        } else {
                            self.clone()
                        };
                        return Some((c, full));
                    }
                }
            }
        }
        None
    }
}
