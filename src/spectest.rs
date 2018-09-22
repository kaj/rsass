use std::ffi::OsStr;
use std::fs::{create_dir, DirEntry, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

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
        "misc",
        &[
            "mixin_content",
            "negative_numbers",
            "unicode_variables",
            "JMA-pseudo-test",
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
    );
    if !ignored.is_empty() {
        writeln!(
            rs,
            "//! The following tests are excluded from conversion:\
             \n//! {:?}",
            ignored,
        );
    }
    writeln!(
        rs,
        "extern crate rsass;\
         \nuse rsass::{{compile_scss, OutputStyle}};\n",
    );

    let mut entries: Vec<DirEntry> =
        suitedir.read_dir()?.collect::<Result<_, _>>()?;
    entries.sort_by_key(|e| e.file_name());
    for entry in entries {
        if entry.file_type()?.is_dir() {
            if entry.path().join("error").exists() {
                eprintln!(
                    "Ignoring {:?}, \
                     tests with expected error not implemented yet.",
                    entry.file_name()
                );
                writeln!(
                    rs,
                    "// Ignoring {:?}, \
                     tests with expected error not implemented yet.\n",
                    entry.file_name()
                );
            } else if ignored.iter().any(|&i| entry.file_name() == i) {
                eprintln!(
                    "Ignoring {:?}, not expected to work yet",
                    entry.file_name()
                );
                writeln!(
                    rs,
                    "// Ignoring {:?}, not expected to work yet\n",
                    entry.file_name()
                );
            } else {
                eprintln!("Should handle {:?}", entry.file_name());
                spec_to_test(&mut rs, &suitedir, &entry.file_name())?;
            }
        }
    }

    writeln!(
        rs,
        "fn rsass(input: &str) -> Result<String, String> {{\
         \n    compile_scss(input.as_bytes(), OutputStyle::Expanded)\
         \n        .map_err(|e| format!(\"rsass failed: {{}}\", e))\
         \n        .and_then(|s| String::from_utf8(s).map_err(|e| format!(\"{{:?}}\", e)))\
         \n}}");
    Ok(())
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
        "/// From {:?}\
         \n#[test]\
         \nfn {}() -> Result<(), String> {{",
        specdir,
        fn_name_os(test),
    );
    let input = format!("{:?}", content(&input)?);
    let expected = format!("{:?}", content(&expected)?);
    if input.len() + expected.len() < 52 {
        writeln!(rs, "    assert_eq!(rsass({})?, {});", input, expected);
    } else if input.len() < 60 {
        writeln!(
            rs,
            "    assert_eq!(\
             \n        rsass({})?,\
             \n        {}\
             \n    );",
            input, expected
        );
    } else {
        writeln!(
            rs,
            "    assert_eq!(\
             \n        rsass(\
             \n            {}\
             \n        )?,\
             \n        {}\
             \n    );",
            input, expected
        );
    }
    writeln!(rs, "    Ok(())\n}}\n");
    Ok(())
}

fn fn_name_os(name: &OsStr) -> String {
    fn_name(&name.to_string_lossy())
}
fn fn_name(name: &str) -> String {
    let t = name.to_lowercase().replace('-', "_").replace('.', "_");
    if t.chars().next().unwrap_or('0').is_numeric() {
        format!("t{}", t)
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
