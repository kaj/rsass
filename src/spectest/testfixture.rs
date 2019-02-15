use super::fn_name_os;
use super::ignore;
use super::options::Options;
use super::Error;
use lazy_static::lazy_static;
use regex::Regex;
use std::io::Write;
use std::path::Path;

pub struct TestFixture {
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
        input: String,
        expected_css: String,
        options: Options,
    ) -> Self {
        TestFixture {
            input: input,
            options: options,
            expectation: ExpectedCSS(normalize_output_css(&expected_css)),
        }
    }

    pub fn new_err(input: String, error: String, options: Options) -> Self {
        TestFixture {
            input: input,
            expectation: ExpectedError(error),
            options: options,
        }
    }

    pub fn write_test(
        &self,
        rs: &mut Write,
        specdir: &Path,
        precision: Option<i64>,
    ) -> Result<(), Error> {
        let test = &specdir.file_name().unwrap_or_default();
        if let Some(ref reason) = self.options.should_skip {
            ignore(rs, &specdir.file_name().unwrap_or_default(), reason)?;
            return Ok(());
        }
        match self.expectation {
            ExpectedError(_) => {
                // TODO: Support error tests;
                ignore(
                    rs,
                    &specdir.file_name().unwrap_or_default(),
                    "error tests are not supported yet",
                )?;
            }
            ExpectedCSS(ref expected) => {
                writeln!(rs, "\n/// From {:?}", specdir)?;
                rs.write_all(b"#[test]\n")?;
                if !check_test(&self.input, expected) {
                    rs.write_all(b"#[ignore] // failing\n")?;
                }
                writeln!(rs, "fn {}() {{", fn_name_os(test))?;
                let precision = self.options.precision.or(precision);
                if let Some(precision) = precision {
                    writeln!(rs, "    set_precision({});", precision)?;
                }
                write_test_input_expected(rs, &self.input, expected)?;
                rs.write_all(b"}\n")?;
            }
        }
        Ok(())
    }
}

fn write_test_input_expected(
    rs: &mut Write,
    input: &str,
    expected: &str,
) -> Result<(), std::io::Error> {
    let input = format!("{:?}", input);
    let expected = format!("{:?}", expected);
    if input.len() + expected.len() < 45 {
        writeln!(
            rs,
            "    assert_eq!(rsass({}).unwrap(), {});",
            input, expected
        )
    } else if input.len() < 54 {
        writeln!(
            rs,
            "    assert_eq!(\
             \n        rsass({}).unwrap(),\
             \n        {}\
             \n    );",
            input, expected
        )
    } else if input.len() < 63 {
        writeln!(
            rs,
            "    assert_eq!(\
             \n        rsass({})\
             \n            .unwrap(),\
             \n        {}\
             \n    );",
            input, expected
        )
    } else {
        writeln!(
            rs,
            "    assert_eq!(\
             \n        rsass(\
             \n            {}\
             \n        )\
             \n        .unwrap(),\
             \n        {}\
             \n    );",
            input, expected
        )
    }
}

fn check_test(input: &str, expected_output: &str) -> bool {
    match rsass(input) {
        Ok(ref output) => output == expected_output,
        Err(_) => false,
    }
}

use rsass::{compile_scss, OutputStyle};

fn rsass(input: &str) -> Result<String, String> {
    compile_scss(input.as_bytes(), OutputStyle::Expanded)
        .map_err(|e| format!("rsass failed: {}", e))
        .and_then(|s| {
            String::from_utf8(s)
                .map(|s| normalize_output_css(s.as_str()))
                .map_err(|e| format!("{:?}", e))
        })
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
