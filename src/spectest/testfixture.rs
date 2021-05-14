use super::options::Options;
use super::writestr::WriteStr;
use super::{fn_name, ignore, Error, TestRunner};
use lazy_static::lazy_static;
use regex::Regex;
use std::io::Write;

pub struct TestFixture {
    name: Option<String>,
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
        name: Option<String>,
        input: String,
        expected_css: &str,
        options: Options,
    ) -> Self {
        TestFixture {
            name,
            input,
            expectation: ExpectedCSS(normalize_output_css(expected_css)),
            options,
        }
    }

    pub fn new_err(
        name: Option<String>,
        input: String,
        error: String,
        options: Options,
    ) -> Self {
        TestFixture {
            name,
            input,
            expectation: ExpectedError(error),
            options,
        }
    }

    pub fn write_test(
        &self,
        rs: &mut dyn Write,
        runner: TestRunner,
    ) -> Result<(), Error> {
        if let Some(ref reason) = self.options.should_skip {
            ignore(rs, &self.name, reason)?;
            return Ok(());
        }
        let has_files = runner.has_files();
        rs.write_all(b"#[test]\n")?;
        if let Some(reason) = self.is_failure(runner) {
            writeln!(rs, "#[ignore] // {}", reason)?;
        }
        let fname = self.name.as_deref().map(fn_name);
        writeln!(rs, "fn {}() {{", fname.as_deref().unwrap_or("test"))?;
        let runner = if self.options.precision.is_some()
            || (has_files && self.name.is_some())
        {
            rs.write_all(b"    let runner = runner()")?;
            if let Some(p) = self.options.precision {
                write!(rs, ".set_precision({});", p)?;
            }
            if has_files {
                if let Some(name) = &self.name {
                    write!(rs, ".with_cwd({:?})", name)?;
                }
            }
            rs.write_all(b";\n")?;
            "runner"
        } else {
            "runner()"
        };
        match self.expectation {
            ExpectedError(ref err) => {
                writeln!(
                    rs,
                    "    assert_eq!(\
                     \n        {}.err(\
                     \n            {}\
                     \n        ),\
                     \n        {},\
                     \n    );",
                    runner,
                    WriteStr(&self.input, 13),
                    WriteStr(err, 9),
                )?;
            }
            ExpectedCSS(ref expected) => {
                writeln!(
                    rs,
                    "    assert_eq!(\
                     \n        {}.ok(\
                     \n            {}\
                     \n        ),\
                     \n        {}\
                     \n    );",
                    runner,
                    WriteStr(&self.input, 13),
                    WriteStr(expected, 9),
                )?;
            }
        }
        rs.write_all(b"}\n")?;
        Ok(())
    }

    /// Execute the test here and now, return None for success or Some reason to fail.
    fn is_failure(&self, runner: TestRunner) -> Option<&'static str> {
        let runner = if let Some(precision) = self.options.precision {
            runner.set_precision(precision as usize)
        } else {
            runner
        };
        match (&self.expectation, runner.run(&self.input)) {
            (ExpectedError(_), Ok(_)) => Some("missing error"),
            (ExpectedError(ref expected), Err(ref actual)) => {
                // TODO: some flexibility in comparision?
                if expected == actual {
                    None
                } else {
                    Some("wrong error")
                }
            }
            (ExpectedCSS(ref expected), Ok(ref actual)) => {
                if expected == actual {
                    None
                } else {
                    Some("wrong result")
                }
            }
            (ExpectedCSS(_), Err(_)) => Some("unexepected error"),
        }
    }
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
