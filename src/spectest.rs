use std::ffi::OsStr;
use std::fs::{create_dir, DirEntry, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    handle_suites().expect("handle suites");
}

fn handle_suites() -> Result<(), Error> {
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
            "plain",                        // Not working yet
            "media",             // only subdir range not working yet
            "min_max", // Expected handling of raw css functions changed.
            "selector", // Only test requres @extend
            "unknown_directive", // Some problem with whitespace?
        ],
    )?;
    handle_suite(
        &base,
        "misc",
        &[
            "mixin_content", // ?
            "negative_numbers",
            "unicode_variables",
            "JMA-pseudo-test", // Requires @extend
            "trailing_comma_in_selector",
        ],
    )?;
    // Note: The round test is broken; it requires 1.49999999999 to be rounded to 1.
    handle_suite(&base, "number-functions", &["round"])?;
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

    handle_entries(&mut rs, &suitedir, &rssuitedir, ignored)?;

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
                let input = entry.path().join("input.scss");
                if input.exists() {
                    eprintln!("Should handle {:?}", entry.file_name());
                    spec_to_test(rs, &suitedir, &entry.file_name())?;
                } else {
                    let name = fn_name_os(&entry.file_name());
                    writeln!(rs, "\nmod {};", name);
                    let rssuitedir = rssuitedir.join(name);
                    let _may_exist = create_dir(&rssuitedir);
                    let mut rs = File::create(rssuitedir.join("mod.rs"))?;
                    writeln!(
                        rs,
                        "//! Tests auto-converted from {:?}\
                         \n#[allow(unused)]\
                         \nuse super::rsass;",
                        suitedir.join(entry.file_name()),
                    )?;
                    handle_entries(
                        &mut rs,
                        &entry.path(),
                        &rssuitedir,
                        ignored,
                    )?;
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
) -> Result<(), Error> {
    let specdir = suite.join(test);
    let input = specdir.join("input.scss");
    let expected = specdir.join("expected_output.css");
    writeln!(
        rs,
        "\n/// From {:?}\
         \n#[test]\
         \nfn {}() {{",
        specdir,
        fn_name_os(test),
    )?;
    let input = format!("{:?}", content(&input)?);
    let expected = format!("{:?}", content(&expected)?);
    if input.len() + expected.len() < 48 {
        writeln!(
            rs,
            "    assert_eq!(rsass({}).unwrap(), {});",
            input, expected
        )?;
    } else if input.len() < 56 {
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
             \n        ).unwrap(),\
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
    let t = name.to_lowercase().replace('-', "_").replace('.', "_");
    if t.chars().next().unwrap_or('0').is_numeric() {
        format!("t{}", t)
    } else if t == "static" {
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
