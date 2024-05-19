use super::{LoadError, SourceName};
use crate::parser::{css, sassfile, Span};
use crate::{Error, ParseError};
use std::io::Read;
use std::sync::Arc;

/// The full data of a source file.
///
/// This type contains both the contents (internally as a `Vec<u8>`)
/// of a file and information on from where (and why) it was loaded.
/// You can create a `SourceFile` with the [`SourceFile::read`]
/// constructor, but normally you will get one from an
/// [`input::Context`][crate::input::Context].
///
/// A `SourceFile` knows what format it is in, so it can apply the
/// correct parser in the [`parse`][Self::parse] method.
/// Currently, the `scss` and `css` formats are supported.
/// If rsass adds support for the `sass` (indented) format, it will
/// also be supported by this type.
///
/// This type is internally reference counted.
#[derive(Clone)]
pub struct SourceFile {
    data: Arc<Impl>,
}

struct Impl {
    data: Vec<u8>,
    source: SourceName,
    format: SourceFormat,
}

impl SourceFile {
    /// Create a `SourceFile` from something readable and a name.
    ///
    /// The format will be determined from the suffix of the `source`
    /// file name.
    ///
    /// This will return an error if reading the `file` fails, or if a
    /// supported format cannot be determined from the `source` name.
    pub fn read<T: Read>(
        file: &mut T,
        source: SourceName,
    ) -> Result<Self, LoadError> {
        let format = SourceFormat::try_from(source.name())?;
        let mut data = vec![];
        file.read_to_end(&mut data)
            .map_err(|e| LoadError::Input(source.name().to_string(), e))?;
        Ok(Self::new(data, source, format))
    }

    /// Handle some raw byte data as an input file with a given source
    /// name.
    ///
    /// The `data` is expected to be in the `scss` format, the `source`
    /// does not need a suffix (e.g. it can be `SourceName::root("-")`
    /// as per convention for standard input).
    pub fn scss_bytes(data: impl Into<Vec<u8>>, source: SourceName) -> Self {
        Self::new(data.into(), source, SourceFormat::Scss)
    }

    /// Handle some raw byte data as an input file with a given source
    /// name.
    ///
    /// The `data` is expected to be in the `css` format, the `source`
    /// does not need a suffix (e.g. it can be `SourceName::root("-")`
    /// as per convention for standard input).
    pub fn css_bytes(data: impl Into<Vec<u8>>, source: SourceName) -> Self {
        Self::new(data.into(), source, SourceFormat::Css)
    }

    fn new(data: Vec<u8>, source: SourceName, format: SourceFormat) -> Self {
        Self {
            data: Arc::new(Impl {
                data,
                source,
                format,
            }),
        }
    }

    /// Parse this source.
    ///
    /// The correct parser will be applied based on the (known) format
    /// of this `SourceFile`.
    pub fn parse(&self) -> Result<Parsed, Error> {
        let data = Span::new(self);
        match self.data.format {
            SourceFormat::Scss => {
                Ok(Parsed::Scss(ParseError::check(sassfile(data))?))
            }
            SourceFormat::Css => {
                Ok(Parsed::Css(ParseError::check(css::file(data))?))
            }
        }
    }

    pub(crate) fn data(&self) -> &[u8] {
        &self.data.data
    }
    pub(crate) fn source(&self) -> &SourceName {
        &self.data.source
    }
    pub(crate) fn path(&self) -> &str {
        self.data.source.name()
    }
}

impl Ord for SourceFile {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.data.source.cmp(&other.data.source)
    }
}
impl PartialOrd for SourceFile {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for SourceFile {
    fn eq(&self, other: &Self) -> bool {
        self.data.source == other.data.source
    }
}
impl Eq for SourceFile {}

/// A supported input format.
///
/// Rsass handles the scss format and raw css.
/// TODO: In the future, the sass format may also be supported.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SourceFormat {
    /// The scss format is the main input format.
    Scss,
    /// The css format
    Css,
}

impl TryFrom<&str> for SourceFormat {
    type Error = LoadError;
    fn try_from(name: &str) -> Result<Self, LoadError> {
        if name.ends_with(".scss") {
            Ok(Self::Scss)
        } else if name.ends_with(".css") {
            Ok(Self::Css)
        } else {
            Err(LoadError::UnknownFormat(name.into()))
        }
    }
}

/// Parsed source that is either css or sass data.
#[derive(Clone, Debug)]
pub enum Parsed {
    /// Raw css data.
    Css(Vec<crate::css::Item>),
    /// Sass (scss) data.
    Scss(Vec<crate::sass::Item>),
}
