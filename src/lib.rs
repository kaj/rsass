//! Sass reimplemented in rust with nom.
//!
//! The "r" in the name might stand for the Rust programming language,
//! or for my name Rasmus.
//!
//! # Example
//!
//! ```
//! use rsass::{OutputStyle, compile_scss_file};
//!
//! let file = "tests/basic/14_imports/a.scss".as_ref();
//! let css = compile_scss_file(file, OutputStyle::Compressed).unwrap();
//!
//! assert_eq!(css, b"div span{moo:goo}\n")
//! ```
//!
//! # Sass language and implemetation status
//!
//! The sass language [is defined in its reference
//! doc](http://sass-lang.com/documentation/file.SASS_REFERENCE.html).
//! This implementation is incomplete but getting there, if slowly.
//!
//! Progress: ![1146](http://progressed.io/bar/114?scale=331&suffix=6)
//! of 3310 tests passed
//! (or 1215 of 6065 when claiming to be libsass).
//!
//! If you want a working rust library for sass right now, you will
//! probably be better of with [sass-rs](https://crates.io/crates/sass-rs)
//! which is a rust wrapper around libsass.
//! Another alternative is [sassers](https://crates.io/crates/sassers)
//! which is another early stage pure rust implementation.
//! That said, this implementation has reached a version where I find it
//! usable for my personal projects, and the number of working tests are
//! improving.
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate nom;
extern crate num_rational;
extern crate num_traits;
extern crate rand;

use std::path::{Path, PathBuf};

mod colors;
mod error;
mod formalargs;
mod functions;
mod selectors;
mod value;
mod variablescope;
mod output_style;
mod unit;
mod parser;

pub use error::Error;
use formalargs::{CallArgs, FormalArgs};
pub use functions::SassFunction;
pub use num_rational::Rational;
pub use output_style::OutputStyle;

pub use parser::{parse_scss_data, parse_scss_file};

use selectors::Selectors;
pub use unit::Unit;
pub use value::{ListSeparator, Quotes, Value, function_call, interpolation,
                quoted_string, singlequoted_string};
pub use variablescope::{GlobalScope, Scope};

/// Parse scss data from a buffer and write css in the given style.
///
/// # Example
///
/// ```
/// use rsass::{OutputStyle, compile_scss};
///
/// assert_eq!(compile_scss(b"foo {\n\
///                             bar {\n\
///                               baz:value;\n\
///                             }\n\
///                           }", OutputStyle::Compressed).unwrap(),
///            b"foo bar{baz:value}\n")
/// ```
pub fn compile_scss(input: &[u8],
                    style: OutputStyle)
                    -> Result<Vec<u8>, Error> {
    let file_context = FileContext::new();
    let items = parse_scss_data(input)?;
    style.write_root(&items, &mut GlobalScope::new(), file_context)
}

/// Parse a file of scss data and write css in the given style.
///
/// Any `@import` directives will be handled relative to the directory
/// part of `file`.
///
/// # Example
///
/// ```
/// use rsass::{OutputStyle, compile_scss_file};
///
/// assert_eq!(compile_scss_file("tests/basic/14_imports/a.scss".as_ref(),
///                              OutputStyle::Compressed).unwrap(),
///            b"div span{moo:goo}\n")
/// ```
pub fn compile_scss_file(file: &Path,
                         style: OutputStyle)
                         -> Result<Vec<u8>, Error> {
    let file_context = FileContext::new();
    let (sub_context, file) = file_context.file(file);
    let items = parse_scss_file(&file)?;
    style.write_root(&items, &mut GlobalScope::new(), sub_context)
}

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
        FileContext { path: PathBuf::new() }
    }
    /// Get a file from this context.
    ///
    /// Get a path and a FileContext from this FileContext and a path.
    pub fn file(&self, file: &Path) -> (Self, PathBuf) {
        let t = self.path.join(file);
        if let Some(dir) = t.parent() {
            (FileContext { path: PathBuf::from(dir) }, t.clone())
        } else {
            (FileContext::new(), t.clone())
        }
    }
    fn find_file(&self, name: &Path) -> Option<(Self, PathBuf)> {
        // TODO Check docs what expansions should be tried!
        let parent = name.parent();
        if let Some(name) = name.file_name().and_then(|n| n.to_str()) {
            for name in &[name,
                          &format!("{}.scss", name),
                          &format!("_{}.scss", name)] {
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

/// Every sass file is a sequence of sass items.
/// Scoping items contains further sequences of items.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SassItem {
    Import(Value),
    VariableDeclaration {
        name: String,
        val: Value,
        default: bool,
        global: bool,
    },
    AtRule {
        name: String,
        args: Value,
        body: Option<Vec<SassItem>>,
    },

    MixinDeclaration {
        name: String,
        args: FormalArgs,
        body: Vec<SassItem>,
    },
    MixinCall { name: String, args: CallArgs, body: Vec<SassItem> },
    Content,

    FunctionDeclaration { name: String, func: SassFunction },
    Return(Value),

    IfStatement(Value, Vec<SassItem>, Vec<SassItem>),
    /// The value may be or evaluate to a list.
    Each(String, Value, Vec<SassItem>),
    For {
        name: String,
        from: Box<Value>,
        to: Box<Value>,
        inclusive: bool,
        body: Vec<SassItem>,
    },
    While(Value, Vec<SassItem>),

    Rule(Selectors, Vec<SassItem>),
    NamespaceRule(String, Value, Vec<SassItem>),
    Property(String, Value, bool),
    Comment(String),
    None,
}

/// Tests from `sass_spec/spec/css/unknown_directive`
#[cfg(test)]
mod css_unknown_directive {
    use super::check;
    // Unknown directives should support almost any sequence of valid tokens,
    // including interpolation.

    #[test]
    fn t01_characters_are_passed_through_unaltered() {
        check("@asdf .~@#$%^&*()_-+=[]|:<>,.?/;\n",
              "@asdf .~@#$%^&*()_-+=[]|:<>,.?/;\n")
    }
    #[test]
    fn t02_strings_are_tokenized_as_strings() {
        check("@asdf \"f'o\" 'b\"r' url(baz) url(\"qux\");\n",
              "@asdf \"f'o\" 'b\"r' url(baz) url(\"qux\");\n")
    }
    #[test]
    fn t03_comments_are_preserved() {
        check("@asdf foo //\n      bar;\n", "@asdf foo //\n      bar;\n")
    }
    #[test]
    fn t04_comments_are_preserved() {
        check("@asdf foo /* bar */ baz;", "@asdf foo /* bar */ baz;\n")
    }
    #[test]
    fn t05_interpolation_plain() {
        check("@asdf #{1 + 2};\n", "@asdf 3;\n")
    }
    #[test]
    fn t06_interpolation_in_string() {
        check("@asdf \"foo #{\"bar\"} baz\";\n", "@asdf \"foo bar baz\";\n")
    }
    #[test]
    #[ignore] // TODO The single quotes should not be converted to double.
    fn t07_interpolation_in_string() {
        check("@asdf 'foo #{'bar'} baz';\n", "@asdf 'foo bar baz';\n")
    }
    #[test]
    fn t08_interpolation_in_url() {
        check("@asdf url(http://#{\")\"}.com/);\n",
              "@asdf url(http://).com/);\n")
    }
    #[test]
    fn t09_interpolation_in_url() {
        check("@asdf url(\"http://#{\")\"}.com/\");\n",
              "@asdf url(\"http://).com/\");\n")
    }
}

#[cfg(test)]
fn check(input: &str, expected: &str) {
    assert_eq!(compile_scss(input.as_bytes(), OutputStyle::Normal)
                   .and_then(|s| Ok(String::from_utf8(s)?))
                   .map_err(|e| format!("{:?}", e)),
               Ok(expected.to_string()));
}
