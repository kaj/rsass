extern crate deunicode;
use deunicode::deunicode;
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
            "14_imports", // Need to handle separate files in tests
            "15_arithmetic_and_lists", // requirements changed
            "33_ambiguous_imports", // Need to handle separate files in tests
        ],
    )?;
    handle_suite(&base, "colors", &[])?;
    handle_suite(
        &base,
        "css",
        &[
            "bizarrely_formatted_comments", // Strange indent?
            "custom_properties",            // Most fails  :-(
            "moz_document",                 // Deprecated functionality
            "ms_long_filter_syntax",        // ?
            "multi_star_comments",          // Some problem with whitespace?
            "plain", // Need to access separate files in tests
            "selector/slotted", // Requires @extend
            "unknown_directive", // ?
        ],
    )?;
    handle_suite(
        &base,
        "libsass",
        &[
            "Sa\u{301}ss-UT\u{327}F8", // resolves to duplicate name
            "at-error/feature-test",
            "at-root/ampersand",
            "at-root/extend",
            "at-root/138_test_at_root_in_media",
            "at-root/139_test_at_root_in_bubbled_media",
            "at-root/140_test_at_root_in_unknown_directive",
            "at-root/with_without",
            "at-stuff",
            "base-level-parent/imported",
            "base-level-parent/nested/at-root-alone-itpl",
            "base-level-parent/nested/at-root-postfix-itpl",
            "base-level-parent/nested/at-root-prefix-itpl",
            "base-level-parent/root/at-root-postfix-itpl",
            "base-level-parent/root/at-root-prefix-itpl",
            "bool",
            "bourbon", // Need to handle separate files in tests
            "calc",
            "charset", // actually expects to-upper-case('ø...') to NOT work?
            "color-functions/opacity/fade-out",
            "color-functions/opacity/transparentize",
            "color-functions/other/change-color/a",
            "color-functions/rgb/rgba/a",
            "color-functions/saturate",
            "conversions",
            "css-import",
            "css_nth_selectors",
            "css_unicode",
            "debug-directive-nested/function",
            "debug-directive-nested/mixin",
            "delayed",
            "div",
            "env",
            "features/at-error",
            "features/extend-selector-pseudoclass",
            "http_import",
            "import",
            "inh", // Requires @extend
            "inheritance",
            "interpolated-function-call",
            "interpolated-urls",
            "list-evaluation",
            "lists",
            "media",
            "mixin", // requires arbitrary (unknown) units
            "mixins-and-media-queries",
            "multi-blocks",
            "placeholder-mediaquery",
            "placeholder-nested",
            "precision/default",
            "precision/lower",
            "properties-in-media",
            "propsets",
            "rel",
            "selector-functions/is_superselector",
            "selector-functions/selector-length",
            "selector-functions/simple-selector",
            "selectors/access",
            "selectors/interpolation",
            "selectors/mixin-argument",
            "selectors/simple",
            "selectors/variables/multiple/bare",
            "selectors/variables/multiple/interpolated",
            "selectors/variables/nested/bare",
            "selectors/variables/nested/interpolated",
            "test",
            "unary-ops",
            "unicode-bom/utf-16-big", // spectest only handles utf8
            "unicode-bom/utf-16-little", // spectest only handles utf8
            "unicode-bom/utf-8",
            "units/conversion/angle",
            "units/conversion/frequency",
            "units/conversion/resolution",
            "units/conversion/size",
            "units/conversion/time",
            "units/simple",
            "url",
            "variable-scoping/blead-global",
            "variable-scoping/defaults",
            "variable-scoping/lexical-scope",
            "variable-scoping/root-scope",
            "variables_in_media",
            "warn-directive-nested/function",
            "warn-directive-nested/mixin",
        ],
    )?;
    handle_suite(
        &base,
        "misc",
        &[
            "mixin_content", // ?
            "negative_numbers",
            "JMA-pseudo-test", // Requires @extend
            "trailing_comma_in_selector",
            "warn-directive",
        ],
    )?;
    // Note: The round test is broken; it requires 1.49999999999 to be rounded to 1.
    handle_suite(&base, "number-functions", &["round"])?;
    handle_suite(
        &base,
        "parser",
        &[
            "interpolate/11_escaped_literal",
            "interpolate/12_escaped_double_quoted/06_escape_interpolation",
            "interpolate/13_escaped_single_quoted/06_escape_interpolation",
            "interpolate/14_escapes_literal_numbers",
            "interpolate/15_escapes_double_quoted_numbers/06_escape_interpolation",
            "interpolate/16_escapes_single_quoted_numbers/06_escape_interpolation",
            "interpolate/17_escapes_literal_lowercase",
            "interpolate/18_escapes_double_quoted_lowercase/06_escape_interpolation",
            "interpolate/19_escapes_single_quoted_lowercase/06_escape_interpolation",
            "interpolate/20_escapes_literal_uppercase",
            "interpolate/21_escapes_double_quoted_uppercase/06_escape_interpolation",
            "interpolate/22_escapes_single_quoted_uppercase/06_escape_interpolation",
            "interpolate/23_escapes_literal_specials",
            "interpolate/24_escapes_double_quoted_specials/todo_05_variable_quoted_double-4.0",
            "interpolate/24_escapes_double_quoted_specials/06_escape_interpolation",
            "interpolate/25_escapes_single_quoted_specials/todo_05_variable_quoted_double-4.0",
            "interpolate/25_escapes_single_quoted_specials/06_escape_interpolation",
            "operations/binary-and-unary",
        ],
    )?;
    handle_suite(&base, "values", &["ids", "numbers/units/multiple"])?;
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
    writeln!(
        rs,
        "extern crate rsass;\
         \nuse rsass::{{compile_scss, OutputStyle}};",
    )?;

    handle_entries(&mut rs, &suitedir, &rssuitedir, None, ignored)?;

    writeln!(
        rs,
        "\nfn rsass(input: &str) -> Result<String, String> {{\
         \n    compile_scss(input.as_bytes(), OutputStyle::Expanded)\
         \n        .map_err(|e| format!(\"rsass failed: {{}}\", e))\
         \n        .and_then(|s| String::from_utf8(s).map_err(|e| format!(\"{{:?}}\", e)))\
         \n}}")?;
    Ok(())
}

fn handle_entries(
    rs: &mut Write,
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
                eprintln!("Should handle {:?}", entry.file_name());
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
    let options = load_options(&specdir)?;
    if let Some(ref reason) = options.should_skip {
        ignore(rs, &specdir.file_name().unwrap_or_default(), reason)?;
        return Ok(());
    }
    let precision = options.precision.or(precision);
    let input = specdir.join("input.scss");
    let expected = specdir.join("output.css");
    writeln!(
        rs,
        "\n/// From {:?}\
         \n#[test]\
         \nfn {}() {{",
        specdir,
        fn_name_os(test),
    )?;
    if let Some(precision) = precision {
        writeln!(rs, "    set_precision({});", precision)?;
    }
    let input = format!("{:?}", content(&input)?);
    let expected = format!("{:?}", content(&expected)?);
    if input.len() + expected.len() < 48 {
        writeln!(
            rs,
            "    assert_eq!(rsass({}).unwrap(), {});",
            input, expected
        )?;
    } else if input.len() < 55 {
        writeln!(
            rs,
            "    assert_eq!(\
             \n        rsass({}).unwrap(),\
             \n        {}\
             \n    );",
            input, expected
        )?;
    } else if input.len() < 62 {
        writeln!(
            rs,
            "    assert_eq!(\
             \n        rsass({})\
             \n            .unwrap(),\
             \n        {}\
             \n    );",
            input, expected
        )?;
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
        )?;
    }
    writeln!(rs, "}}")?;
    Ok(())
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

use std::fmt;
impl fmt::Display for Error {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(out)
    }
}

extern crate yaml_rust;
use yaml_rust::{Yaml, YamlLoader};

struct Options {
    pub precision: Option<i64>,
    /// None for tests that should work, or Some(reason to skip).
    pub should_skip: Option<String>,
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
