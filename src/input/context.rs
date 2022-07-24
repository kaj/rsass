use super::{FsLoader, Loader, SourceFile, SourceKind};
use crate::output::Format;
use crate::{Error, ScopeRef};
use std::{borrow::Cow, collections::BTreeMap, path::Path};
use tracing::instrument;

/// Utility keeping track of loading files.
#[derive(Debug)]
pub struct Context<FileC> {
    file_context: FileC,
    loading: BTreeMap<String, SourceKind>,
    // TODO: Maybe have a map to loaded SourceFiles as well?  Or even Parsed?
}

/// A file-system based [`Context`].
pub type FsContext = Context<FsLoader>;

impl Context<FsLoader> {
    /// Create a new `Context`, loading files based on the current
    /// working directory.
    pub fn for_cwd() -> Self {
        Context::for_loader(FsLoader::for_cwd())
    }

    /// Create a new `Context` and load a file.
    ///
    /// The directory part of `path` is used as a base directory for the loader.
    pub fn for_path(path: &Path) -> Result<(Self, SourceFile), Error> {
        let (file_context, file) = FsLoader::for_path(path)?;
        Ok((Context::for_loader(file_context), file))
    }

    /// Add a path to search for files.
    pub fn push_path(&mut self, path: &Path) {
        self.file_context.push_path(path);
    }
}

impl<AnyLoader: Loader> Context<AnyLoader> {
    /// Create a new `Context` for a given file [`Loader`].
    pub fn for_loader(fc: AnyLoader) -> Self {
        Context {
            file_context: fc,
            loading: Default::default(),
        }
    }

    /// Transform some input source to css.
    ///
    /// The css output is returned as a raw byte vector.
    ///
    /// # Examples
    ///
    /// Input is usually a scss file.
    ///
    /// ```
    /// # use rsass::input::{FsContext, SourceFile, SourceName};
    /// # use rsass::output::{Format, Style};
    /// # fn main() -> Result<(), rsass::Error> {
    /// # let context = FsContext::for_cwd();
    /// let compressed = Format { style: Style::Compressed, precision: 2 };
    /// let scss_input = SourceFile::scss_bytes(
    ///     "$gap: 4em / 3;
    ///     \np {\
    ///     \n    margin: $gap 0;
    ///     \n}\n",
    ///     SourceName::root("-")
    /// );
    /// assert_eq!(
    ///     context.transform(scss_input, compressed)?,
    ///     b"p{margin:1.33em 0}\n"
    /// );
    /// # Ok(()) }
    /// ```
    ///
    /// This method can also be used as a plain css compression.
    /// ```
    /// # use rsass::input::{FsContext, SourceFile, SourceName};
    /// # use rsass::output::{Format, Style};
    /// # fn main() -> Result<(), rsass::Error> {
    /// # let context = FsContext::for_cwd();
    /// # let compressed = Format { style: Style::Compressed, precision: 2 };
    /// let css_input = SourceFile::css_bytes(
    ///     "p {\
    ///     \n    margin: 1.333333333em 0;\
    ///     \n}\n",
    ///     SourceName::root("-")
    /// );
    /// assert_eq!(
    ///     context.transform(css_input, compressed)?,
    ///     b"p{margin:1.33em 0}\n"
    /// );
    /// # Ok(()) }
    /// ```
    pub fn transform(
        mut self,
        file: SourceFile,
        format: Format,
    ) -> Result<Vec<u8>, Error> {
        let scope = ScopeRef::new_global(format);
        self.lock_loading(&file, false)?;
        let source = file.parse()?;
        let result = format.write_root(source, scope, &mut self);
        self.unlock_loading(&file);
        result
    }

    /// Find a file.
    ///
    /// This method handles sass file name resolution, but delegates
    /// the actual checking for existing files to the [`Loader`].
    ///
    /// Given a url like `my/util`, this method will check for
    /// `my/util`, `my/util.scss`, `my/_util.scss`,
    /// `my/util/index.scss`, and `my/util/_index.scss`.
    /// The variants that are not a directory index will also be
    /// checked for `.css` files (and in the future it may also check
    /// for `.sass` files if rsass suports that format).
    ///
    /// If `from` indicates that the loading is for an `@import` rule,
    /// some [extra file names][import-only] are checked.
    ///
    /// The `Context` keeps track of "locked" files (files currently beeing
    /// parsed or transformed into css).
    /// The source file returned from this function is locked, so the
    /// caller of this method need to call [`Self::unlock_loading`] after
    /// handling it.
    ///
    /// [import-only]: https://sass-lang.com/documentation/at-rules/import#import-only-files
    #[instrument]
    pub fn find_file(
        &mut self,
        url: &str,
        from: SourceKind,
    ) -> Result<Option<SourceFile>, Error> {
        let names: &[&dyn Fn(&str, &str) -> String] = if from.is_import() {
            &[
                // base will either be empty or end with a slash.
                &|base, name| format!("{}{}.import.scss", base, name),
                &|base, name| format!("{}_{}.import.scss", base, name),
                &|base, name| format!("{}{}.scss", base, name),
                &|base, name| format!("{}_{}.scss", base, name),
                &|base, name| format!("{}{}/index.import.scss", base, name),
                &|base, name| format!("{}{}/_index.import.scss", base, name),
                &|base, name| format!("{}{}/index.scss", base, name),
                &|base, name| format!("{}{}/_index.scss", base, name),
                &|base, name| format!("{}{}.css", base, name),
                &|base, name| format!("{}_{}.css", base, name),
            ]
        } else {
            &[
                // base will either be empty or end with a slash.
                &|base, name| format!("{}{}.scss", base, name),
                &|base, name| format!("{}_{}.scss", base, name),
                &|base, name| format!("{}{}/index.scss", base, name),
                &|base, name| format!("{}{}/_index.scss", base, name),
                &|base, name| format!("{}{}.css", base, name),
                &|base, name| format!("{}_{}.css", base, name),
            ]
        };
        // Note: Should a "full stack" of bases be used here?
        // Or is this fine?
        let url = relative(&from, url);
        if let Some((path, mut file)) = self.do_find_file(&url, names)? {
            let is_module = !from.is_import();
            let source = from.url(&path);
            let file = SourceFile::read(&mut file, source)?;
            self.lock_loading(&file, is_module)?;
            Ok(Some(file))
        } else {
            Ok(None)
        }
    }

    /// Find a file in a given filecontext matching a url over a set of
    /// name rules.
    fn do_find_file(
        &self,
        url: &str,
        names: &[&dyn Fn(&str, &str) -> String],
    ) -> Result<Option<(String, AnyLoader::File)>, Error> {
        if url.ends_with(".css")
            || url.ends_with(".sass")
            || url.ends_with(".scss")
        {
            self.file_context
                .find_file(url)
                .map(|file| file.map(|file| (url.into(), file)))
        } else {
            let (base, name) = url
                .rfind('/')
                .map(|p| url.split_at(p + 1))
                .unwrap_or(("", url));

            for name in names.iter().map(|f| f(base, name)) {
                if let Some(result) = self.file_context.find_file(&name)? {
                    return Ok(Some((name, result)));
                }
            }
            Ok(None)
        }
    }

    pub(crate) fn lock_loading(
        &mut self,
        file: &SourceFile,
        as_module: bool,
    ) -> Result<(), Error> {
        let name = file.source().name();
        let pos = &file.source().imported;
        if let Some(old) = self.loading.insert(name.into(), pos.clone()) {
            Err(Error::ImportLoop(
                as_module,
                pos.next().unwrap().clone(),
                old.next().cloned(),
            ))
        } else {
            Ok(())
        }
    }

    /// Unlock a file that is locked for input processing.
    ///
    /// The lock exists to break circular dependency chains.
    /// Each file that is locked (by [`Self::find_file`]) needs to be unlocked
    /// when processing of it is done.
    pub fn unlock_loading(&mut self, file: &SourceFile) {
        self.loading.remove(file.path());
    }
}

/// Make a url relative to a given base.
fn relative<'a>(base: &SourceKind, url: &'a str) -> Cow<'a, str> {
    base.next()
        .map(|pos| pos.file_url())
        .and_then(|base| {
            base.rfind('/')
                .map(|p| base.split_at(p + 1).0)
                .map(|base| format!("{}{}", base, url).into())
        })
        .unwrap_or_else(|| url.into())
}
