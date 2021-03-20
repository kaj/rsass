use super::ignore;
use super::options::Options;
use super::Error;
use lazy_static::lazy_static;
use regex::Regex;
use rsass::output::Format;
use rsass::{parse_scss_data, FsFileContext, ScopeRef};
use std::io::Write;

pub struct TestFixture {
    fn_name: String,
    input: String,
    expectation: TestExpectation,
    options: Options,
}

enum TestExpectation {
    ExpectedCSS(String),
    ExpectedError(String),
}

use TestExpectation::{ExpectedCSS, ExpectedError};

impl TestFixture {
    pub fn new_ok(
        fn_name: String,
        input: String,
        expected_css: &str,
        options: Options,
    ) -> Self {
        TestFixture {
            fn_name,
            input: input,
            options: options,
            expectation: ExpectedCSS(normalize_output_css(expected_css)),
        }
    }

    pub fn new_err(
        fn_name: String,
        input: String,
        error: String,
        options: Options,
    ) -> Self {
        TestFixture {
            fn_name,
            input: input,
            expectation: ExpectedError(error),
            options: options,
        }
    }

    pub fn write_test(&self, rs: &mut dyn Write) -> Result<(), Error> {
        if let Some(ref reason) = self.options.should_skip {
            ignore(rs, &self.fn_name, reason)?;
            return Ok(());
        }
        match self.expectation {
            ExpectedError(_) => {
                // TODO: Support error tests;
                ignore(
                    rs,
                    &self.fn_name,
                    "error tests are not supported yet",
                )?;
            }
            ExpectedCSS(ref expected) => {
                rs.write_all(b"#[test]\n")?;
                if let Some(reason) = self.is_failure() {
                    writeln!(rs, "#[ignore] // {}", reason)?;
                }
                writeln!(rs, "fn {}() {{", self.fn_name)?;
                let precision = self.options.precision;
                if let Some(precision) = precision {
                    writeln!(
                        rs,
                        "    let format = rsass::output::Format {{ \
                         style: rsass::output::Style::Expanded, \
                         precision: {} \
                         }};",
                        precision,
                    )?;
                }
                let input = format!("{:?}", self.input)
                    .replace(escaped_newline(), "\n            \\n");
                let expected = format!("{:?}", expected)
                    .replace(escaped_newline(), "\n        \\n");
                writeln!(
                    rs,
                    "    assert_eq!(\
                     \n        {}\
                     \n            {}\
                     \n        )\
                     \n        .unwrap(),\
                     \n        {}\
                     \n    );",
                    if precision.is_none() {
                        "crate::rsass("
                    } else {
                        "crate::rsass_fmt(format,"
                    },
                    input,
                    expected,
                )?;
                rs.write_all(b"}\n")?;
            }
        }
        Ok(())
    }

    /// Execute the test here and now, return None for success or Some
    /// reason to fail.
    fn is_failure(&self) -> Option<&'static str> {
        match &self.expectation {
            ExpectedError(_) => Some("Error tests not supported yet"),
            ExpectedCSS(ref expected) => {
                let mut format = Format::default();
                if let Some(precision) = self.options.precision {
                    format.precision = precision as usize;
                }
                match rsass(&self.input, format) {
                    Ok(ref actual) => {
                        if expected == actual {
                            None // Yes!
                        } else {
                            Some("wrong result")
                        }
                    }
                    Err(_) => Some("unexepected error"),
                }
            }
        }
    }
}

/// Return a pattern function matching the 'n' in \n, unless the
/// backslash is also escaped.
fn escaped_newline() -> impl FnMut(char) -> bool {
    let mut n = 0;
    move |c: char| {
        let next_n = if c == '\\' { n + 1 } else { 0 };
        let result = n % 2 == 1 && c == 'n';
        n = next_n;
        result
    }
}

fn rsass(input: &str, format: Format) -> Result<String, String> {
    compile_scss(input.as_bytes(), format)
        .map_err(|e| format!("rsass failed: {}", e))
        .and_then(|s| {
            String::from_utf8(s)
                .map(|s| normalize_output_css(s.as_str()))
                .map_err(|e| format!("{:?}", e))
        })
}

pub fn compile_scss(
    input: &[u8],
    format: Format,
) -> Result<Vec<u8>, rsass::Error> {
    let mut file_context = FsFileContext::new();
    file_context.push_path("tests/spec".as_ref());
    let items = parse_scss_data(input)?;
    format.write_root(&items, ScopeRef::new_global(format), &file_context)
}

fn normalize_output_css(css: &str) -> String {
    // Normalizes the whitespace in the given CSS to make it easier to compare. Based on:
    // https://github.com/sass/sass-spec/blob/0f59164aabb3273645fda068d0fb1b879aa3f1dc/lib/sass_spec/util.rb#L5-L7
    // NOTE: This is done on input and expected output.
    // The actual result is normalized in a simler way in the rsass function in generated tests.
    lazy_static! {
        static ref RE: Regex = Regex::new("(?:\r?\n)+").unwrap();
    }
    RE.replace_all(&css, "\n").to_string()
}
