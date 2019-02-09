use deunicode::deunicode;
use lazy_static::lazy_static;
use regex::Regex;
use std::ffi::OsStr;
use std::fs::{create_dir, DirEntry, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

/// Sass spec version targeted.
const VERSION: f32 = 3.6;

fn main() -> Result<(), Error> {
    let base = PathBuf::from("sass-spec/spec");
    handle_suite(
        &base,
        "basic",
        &[
            "14_imports",           // Need to handle separate files in tests
            "33_ambiguous_imports", // Need to handle separate files in tests
        ],
    )?;
    handle_suite(&base, "colors", &[])?;
    handle_suite(
        &base,
        "css",
        &[
            "plain", // Need to access separate files in tests
        ],
    )?;
    handle_suite(
        &base,
        "libsass",
        &[
            "Sa\u{301}ss-UT\u{327}F8", // resolves to duplicate name
            "bourbon", // Need to handle separate files in tests
            "base-level-parent/imported", // multiple input files
            "selector-functions/is_superselector", // multiple input files
            "unicode-bom/utf-16-big", // rsass only handles utf8
            "unicode-bom/utf-16-little", // rsass only handles utf8
            "debug-directive-nested/function", // panic
            "warn-directive-nested/function", // panic
        ],
    )?;
    handle_suite(&base, "misc", &[])?;
    handle_suite(&base, "number-functions", &[])?;
    handle_suite(&base, "parser", &[])?;
    handle_suite(
        &base,
        "selector-functions",
        &[
            "is_superselector", // multiple input files
        ],
    )?;
    handle_suite(
        &base,
        "scss",
        &[
            "multiline-var", // name conflict with other test.
            "mixin-content", // stack overflow?!?
            "huge",          // stack overflow
        ],
    )?;
    handle_suite(&base, "values", &[])?;
    Ok(())
}

fn handle_suite(
    base: &Path,
    suite: &str,
    ignored: &[&str],
) -> Result<(), Error> {
    eprintln!("Handle suite {:?}", suite);
    let suitedir = base.join(suite);
    let rssuitedir = PathBuf::from("tests").join(fn_name(suite));
    let _may_exist = create_dir(&rssuitedir);
    let mut rs = File::create(rssuitedir.join("main.rs"))?;
    writeln!(
        rs,
        "//! Tests auto-converted from {:?}\
         \n//! version {}\
         \n//! See <https://github.com/sass/sass-spec> for source material.\\n",
        suitedir,
        String::from_utf8(
            Command::new("git")
                .args(&["log", "-1", "--format=%h, %ai."])
                .current_dir(base)
                .output()?
                .stdout
        )?.trim(),
    )?;
    if !ignored.is_empty() {
        writeln!(
            rs,
            "//! The following tests are excluded from conversion:\
             \n//! {:?}",
            ignored,
        )?;
    }
    writeln!(rs, "use rsass::{{compile_scss, OutputStyle}};",)?;

    handle_entries(&mut rs, &base, &suitedir, &rssuitedir, None, ignored)?;

    writeln!(
        rs,
        "\nfn rsass(input: &str) -> Result<String, String> {{\
         \n    compile_scss(input.as_bytes(), OutputStyle::Expanded)\
         \n        .map_err(|e| format!(\"rsass failed: {{}}\", e))\
         \n        .and_then(|s| {{\
         \n            String::from_utf8(s)\
         \n                .map(|s| s.replace(\"\\n\\n\", \"\\n\"))\
         \n                .map_err(|e| format!(\"{{:?}}\", e))\
         \n        }})\
         \n}}"
    )?;
    Ok(())
}

fn handle_entries(
    rs: &mut Write,
    root: &Path,
    suitedir: &Path,
    rssuitedir: &Path,
    precision: Option<i64>,
    ignored: &[&str],
) -> Result<(), Error> {
    let mut entries: Vec<DirEntry> =
        suitedir.read_dir()?.collect::<Result<_, _>>()?;
    entries.sort_by_key(|e| e.file_name());
    for entry in entries {
        if entry.file_type()?.is_dir() {
            if entry.path().join("error").is_file() {
                ignore(
                    rs,
                    &entry.file_name(),
                    "tests with expected error not implemented yet",
                )?;
            } else if ignored.iter().any(|&i| &entry.file_name() == i) {
                ignore(rs, &entry.file_name(), "not expected to work yet")?;
            } else {
                eprintln!(
                    "Should handle {}",
                    entry.path().strip_prefix(root)?.display()
                );
                let input = entry.path().join("input.scss");
                if input.exists() {
                    spec_to_test(
                        rs,
                        &suitedir,
                        &entry.file_name(),
                        precision,
                    )
                    .map_err(|e| {
                        Error(format!(
                            "Failed to handle {:?}: {}",
                            entry.file_name(),
                            e,
                        ))
                    })?;
                } else {
                    let options = load_options(&entry.path())?;
                    if let Some(ref reason) = options.should_skip {
                        ignore(rs, &entry.file_name(), reason)?;
                    } else {
                        let precision = options.precision.or(precision);
                        let name = fn_name_os(&entry.file_name());
                        writeln!(rs, "\nmod {};", name)?;
                        let rssuitedir = rssuitedir.join(name);
                        let _may_exist = create_dir(&rssuitedir);
                        let mut rs = File::create(rssuitedir.join("mod.rs"))?;
                        writeln!(
                            rs,
                            "//! Tests auto-converted from {:?}\
                             \n#[allow(unused)]\
                             \nuse super::rsass;\
                             \n#[allow(unused)]\
                             \nuse rsass::set_precision;",
                            suitedir.join(entry.file_name()),
                        )?;
                        let tt = format!(
                            "{}/",
                            entry.file_name().to_string_lossy(),
                        );
                        let ignored: Vec<&str> = ignored
                            .iter()
                            .filter_map(|p| {
                                if p.starts_with(&tt) {
                                    Some(p.split_at(tt.len()).1)
                                } else {
                                    None
                                }
                            })
                            .collect();
                        handle_entries(
                            &mut rs,
                            &root,
                            &entry.path(),
                            &rssuitedir,
                            precision,
                            &ignored,
                        )?;
                    }
                }
            }
        }
    }
    Ok(())
}

fn ignore(
    rs: &mut Write,
    name: &OsStr,
    reason: &str,
) -> Result<(), io::Error> {
    eprintln!("Ignoring {:?}, {}.", name, reason);
    writeln!(rs, "\n// Ignoring {:?}, {}.", name, reason)
}

fn spec_to_test(
    rs: &mut Write,
    suite: &Path,
    test: &OsStr,
    precision: Option<i64>,
) -> Result<(), Error> {
    let specdir = suite.join(test);
    let fixture = load_test_fixture(&specdir)?;

    if let Some(ref reason) = fixture.options.should_skip {
        ignore(rs, &specdir.file_name().unwrap_or_default(), reason)?;
        return Ok(());
    }
    match fixture.expectation {
        ExpectedError(_) => {
            // TODO: Support error tests;
            ignore(
                rs,
                &specdir.file_name().unwrap_or_default(),
                "error tests are not supported yet",
            )?;
            return Ok(());
        }
        ExpectedCSS(ref expected) => {
            writeln!(rs, "\n/// From {:?}", specdir)?;
            rs.write_all(b"#[test]\n")?;
            if !check_test(&fixture.input, expected) {
                rs.write_all(b"#[ignore] // failing\n")?;
            }
            writeln!(rs, "fn {}() {{", fn_name_os(test))?;
            let precision = fixture.options.precision.or(precision);
            if let Some(precision) = precision {
                writeln!(rs, "    set_precision({});", precision)?;
            }
            write_test_input_expected(rs, &fixture.input, expected)?;
        }
    }
    rs.write_all(b"}\n")?;
    Ok(())
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

fn fn_name_os(name: &OsStr) -> String {
    fn_name(&name.to_string_lossy())
}
fn fn_name(name: &str) -> String {
    let t = deunicode(name)
        .to_lowercase()
        .replace('-', "_")
        .replace('.', "_");
    if t.chars().next().unwrap_or('0').is_numeric() {
        format!("t{}", t)
    } else if t == "else"
        || t == "for"
        || t == "if"
        || t == "static"
        || t == "while"
    {
        format!("test_{}", t)
    } else {
        t
    }
}

struct TestFixture {
    pub input: String,
    pub expectation: TestExpectation,
    pub options: Options,
}

enum TestExpectation {
    ExpectedCSS(String),
    ExpectedError(String),
}

use TestExpectation::{ExpectedCSS, ExpectedError};

use yaml_rust::{Yaml, YamlLoader};

struct Options {
    pub precision: Option<i64>,
    /// None for tests that should work, or Some(reason to skip).
    pub should_skip: Option<String>,
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

fn load_test_fixture(specdir: &Path) -> Result<TestFixture, Error> {
    static INPUT_FILENAME: &str = "input.scss";
    static EXPECTED_OUTPUT_FILENAME: &str = "output.css";
    static EXPECTED_ERROR_FILENAMES: &[&str] = &["error-dart-sass", "error"];

    // TODO: hrx support.
    let options = load_options(&specdir)?;
    let input = content(&specdir.join(INPUT_FILENAME))?;

    {
        let path = specdir.join(EXPECTED_OUTPUT_FILENAME);
        if path.exists() {
            return Ok(TestFixture {
                input: input,
                options: options,
                expectation: ExpectedCSS(normalize_output_css(&content(
                    &path,
                )?)),
            });
        }
    }
    for filename in EXPECTED_ERROR_FILENAMES {
        let path = specdir.join(filename);
        if path.exists() {
            return Ok(TestFixture {
                input: input,
                expectation: ExpectedError(content(&path)?),
                options: options,
            });
        }
    }
    Err(Error(format!(
        "No expected CSS / error found for {}",
        specdir.file_name().unwrap_or_default().to_str().unwrap()
    )))
}

/// Load options from options.yml.
fn load_options(path: &Path) -> Result<Options, Error> {
    let yml = path.join("options.yml");
    if yml.exists() {
        let options = content(&yml)?;
        let options = YamlLoader::load_from_str(&options)?;
        if options.len() > 1 {
            Err(Error(format!("Found multiple-doc options {:?}", options)))?;
        }
        if options.len() == 0 {
            Err(Error(format!("Found zero-doc options {:?}", options)))?;
        }
        let options = &options[0];
        eprintln!("Found options: {:?}", options);
        Ok(Options {
            precision: options[":precision"].as_i64(),
            should_skip: {
                if let Some(skip) = skip_ended(options)? {
                    Some(skip)
                } else {
                    skip_unstarted(options)?
                }
            },
        })
    } else {
        Ok(Options {
            precision: None,
            should_skip: None,
        })
    }
}

fn skip_ended(options: &Yaml) -> Result<Option<String>, Error> {
    if let Some(end) = options[":end_version"].as_str() {
        if end.parse::<f32>()? <= VERSION {
            Ok(Some(format!("end_version is {}", end)))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

fn skip_unstarted(options: &Yaml) -> Result<Option<String>, Error> {
    if let Some(start) = options[":start_version"].as_str() {
        if start.parse::<f32>()? <= VERSION {
            Ok(None)
        } else {
            Ok(Some(format!("start_version is {}", start)))
        }
    } else {
        Ok(None)
    }
}

fn content(path: &Path) -> Result<String, io::Error> {
    let mut buf = String::new();
    File::open(path)?.read_to_string(&mut buf)?;
    Ok(buf)
}

#[derive(Debug)]
struct Error(String);
use std::convert::From;

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error(format!("io error: {}", e))
    }
}
impl From<std::string::FromUtf8Error> for Error {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Error(format!("utf8 error: {}", e))
    }
}
impl From<yaml_rust::ScanError> for Error {
    fn from(e: yaml_rust::ScanError) -> Self {
        Error(format!("utf8 error: {}", e))
    }
}
impl From<std::num::ParseFloatError> for Error {
    fn from(e: std::num::ParseFloatError) -> Self {
        Error(format!("utf8 error: {}", e))
    }
}
impl From<std::path::StripPrefixError> for Error {
    fn from(e: std::path::StripPrefixError) -> Self {
        Error(format!("{}", e))
    }
}

use std::fmt;
impl fmt::Display for Error {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(out)
    }
}
