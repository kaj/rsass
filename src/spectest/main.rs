use deunicode::deunicode;
use hrx_get::Archive;
use lazy_regex::regex_is_match;
use std::ffi::OsStr;
use std::fs::{create_dir, DirEntry, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

mod options;
use options::Options;
mod testfixture;
use testfixture::TestFixture;
mod testrunner;
use testrunner::{runner, TestRunner};
mod writestr;

fn main() -> Result<(), Error> {
    let base = PathBuf::from("sass-spec");
    handle_suite(
        &base,
        "spec",
        &[
            "core_functions/selector/extend", // not supported
            "core_functions/selector/is_superselector", // not supported
            "core_functions/selector/unify",  // not supported
            "directives/extend", // `@extend` is not supported at all
            "libsass-todo-issues/issue_221262.hrx", // stack overflow
            "libsass-todo-issues/issue_221292.hrx", // stack overflow
            "libsass/unicode-bom/utf-16-big", // rsass only handles utf8
            "libsass/unicode-bom/utf-16-little", // rsass only handles utf8
            "non_conformant/scss/huge.hrx", // stack overflow in debug mode
            "non_conformant/scss/multiline-var.hrx", // duplicate rust name
        ],
    )?;
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
    rs.write_all(
        b"mod testrunner;\nuse testrunner::{runner, TestRunner};\n\n",
    )?;
    {
        let mut tr = File::create(rssuitedir.join("testrunner.rs"))?;
        tr.write_all(include_bytes!("testrunner.rs"))?;
    }
    handle_entries(&mut rs, base, &suitedir, &rssuitedir, None, ignored)
        .map_err(|e| {
            Error(format!("Failed to handle suite {:?}: {}", suite, e))
        })
}

fn handle_entries(
    rs: &mut dyn Write,
    root: &Path,
    suitedir: &Path,
    rssuitedir: &Path,
    precision: Option<usize>,
    ignored: &[&str],
) -> Result<(), Error> {
    let mut entries: Vec<DirEntry> =
        suitedir.read_dir()?.collect::<Result<_, _>>()?;
    entries.sort_by_key(|e| e.file_name());
    for entry in entries {
        if ignored.iter().any(|&i| entry.file_name() == i) {
            ignore(rs, &entry.file_name(), "not expected to work yet")?;
        } else if entry.file_type()?.is_file()
            && entry.file_name().to_string_lossy().ends_with(".hrx")
        {
            let name = fn_name_os(&entry.file_name());
            writeln!(rs, "\nmod {};", name)?;
            let mut rs =
                File::create(rssuitedir.join(format!("{}.rs", name)))?;
            writeln!(
                rs,
                "//! Tests auto-converted from {:?}\n",
                suitedir.join(entry.file_name()),
            )?;
            spec_hrx_to_test(&mut rs, &entry.path(), precision).map_err(
                |e| {
                    Error(format!(
                        "Failed to handle {:?}: {}",
                        entry.path(),
                        e
                    ))
                },
            )?;
        } else if entry.file_type()?.is_dir() {
            if entry.path().join("error").is_file() {
                ignore(
                    rs,
                    &entry.file_name(),
                    "tests with expected error not implemented yet",
                )?;
            } else {
                eprintln!(
                    "Should handle {}",
                    entry.path().strip_prefix(root)?.display()
                );
                let input = entry.path().join("input.scss");
                if input.exists() {
                    spec_dir_to_test(
                        rs,
                        suitedir,
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
                        let name = fn_name_os(&entry.file_name());
                        writeln!(rs, "\nmod {};", name)?;
                        let rssuitedir = rssuitedir.join(name);
                        let _may_exist = create_dir(&rssuitedir);
                        let mut rs = File::create(rssuitedir.join("mod.rs"))?;
                        writeln!(
                            rs,
                            "//! Tests auto-converted from {:?}\n",
                            suitedir.join(entry.file_name()),
                        )?;
                        writeln!(
                            rs,
                            "#[allow(unused)]\
                             \nfn runner() -> crate::TestRunner {{\
                             \n    super::runner()",
                        )?;
                        writeln!(
                            rs,
                            "        .with_cwd({:?})",
                            entry.file_name()
                        )?;
                        if let Some(p) = options.precision {
                            writeln!(rs, "    .set_precision({})\n", p)?;
                        }
                        writeln!(rs, "\n}}\n")?;
                        let precision = options.precision.or(precision);
                        let tt = format!(
                            "{}/",
                            entry.file_name().to_string_lossy(),
                        );
                        let ignored: Vec<&str> = ignored
                            .iter()
                            .filter_map(|p| p.strip_prefix(&tt))
                            .collect();
                        handle_entries(
                            &mut rs,
                            root,
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

fn ignore<T: std::fmt::Debug>(
    rs: &mut dyn Write,
    name: &T,
    reason: &str,
) -> Result<(), io::Error> {
    eprintln!("Ignoring {:?}, {}.", name, reason);
    writeln!(rs, "\n// Ignoring {:?}, {}.", name, reason)
}

fn spec_dir_to_test(
    rs: &mut dyn Write,
    suite: &Path,
    test: &OsStr,
    precision: Option<usize>,
) -> Result<(), Error> {
    let specdir = suite.join(test);
    let fixture = load_test_fixture_dir(&specdir, precision)?;
    writeln!(rs, "\n// From {:?}", specdir)?;
    let runner = if let Some(stem) = suite.file_stem() {
        runner().with_cwd(stem.to_str().unwrap())
    } else {
        runner()
    };
    fixture.write_test(rs, runner)
}

fn spec_hrx_to_test(
    rs: &mut dyn Write,
    suite: &Path,
    precision: Option<usize>,
) -> Result<(), Error> {
    let archive = Archive::load(suite)
        .map_err(|e| Error(format!("Failed to load hrx: {}", e)))?;

    eprintln!("Handle {}", suite.display());
    rs.write_all(
        b"#[allow(unused)]\
          \nfn runner() -> crate::TestRunner {\
          \n    super::runner()\n",
    )?;
    let mut runner = if let Some(stem) = suite.file_stem() {
        writeln!(rs, "        .with_cwd({:?})", stem)?;
        runner().with_cwd(stem.to_str().unwrap())
    } else {
        runner()
    };
    for (name, content) in archive.entries() {
        if ![
            "README.md",
            "input.scss",
            "input.sass",
            "output.css",
            "output-libsass.css",
            "output-dart-sass.css",
            "error",
            "error-libsass",
            "error-dart-sass",
            "warning",
            "warning-libsass",
            "warning-dart-sass",
            "options.yml",
        ]
        .iter()
        .any(|n| name.ends_with(n))
        {
            writeln!(rs, "        .mock_file({:?}, {:#?})", name, content)?;
            runner = runner.mock_file(name, content);
        }
    }
    for (name, content) in archive.entries() {
        if let Some(base) = name.strip_suffix("input.scss") {
            if references_input(content)
                || archive.entries().any(|(path, content)| {
                    path.starts_with(base) && references_input(content)
                })
            {
                writeln!(
                    rs,
                    "        .mock_file({:?}, {:#?})",
                    name, content
                )?;
                runner = runner.mock_file(name, content);
            }
        }
    }
    if let Some(p) = precision {
        writeln!(rs, "        .set_precision({})", p)?;
    }
    rs.write_all(b"}\n\n")?;
    handle_hrx_part(rs, suite, &archive, "", precision, runner)
}

fn references_input(content: &str) -> bool {
    regex_is_match!("['\"]input['\"]", content)
}

fn handle_hrx_part(
    rs: &mut dyn Write,
    suite: &Path,
    archive: &Archive,
    prefix: &str,
    precision: Option<usize>,
    runner: TestRunner,
) -> Result<(), Error> {
    use std::collections::BTreeSet;
    let tests = archive
        .names()
        .iter()
        .flat_map(|s| {
            s.strip_prefix(prefix)
                .and_then(|s| s.find('/').map(|p| &s[..p + 1]))
        })
        .collect::<BTreeSet<_>>();

    let name = if prefix.is_empty() {
        None
    } else {
        let t = prefix.trim_end_matches('/');
        let t = if let Some(p) = t.rfind('/') {
            &t[p + 1..]
        } else {
            t
        };
        Some(t)
    };

    let mut options = archive
        .get(&format!("{}options.yml", prefix))
        .map(Options::parse)
        .transpose()?
        .unwrap_or_default();
    options.precision = options.precision.or(precision);

    if archive.get(&format!("{}input.scss", prefix)).is_some() {
        let fixture = load_test_fixture_hrx(
            name.map(str::to_owned),
            archive,
            prefix,
            options,
        )?;
        fixture.write_test(rs, runner)
    } else if let Some(ref reason) = options.should_skip {
        ignore(rs, &suite.file_name(), reason).map_err(|e| e.into())
    } else {
        if !tests.is_empty() {
            if let Some(ref name) = name {
                writeln!(rs, "mod {} {{", fn_name(name))?;
                if options.precision.is_some() || runner.has_files() {
                    write!(
                        rs,
                        "    #[allow(unused)]\
                         \n    fn runner() -> crate::TestRunner {{\
                         \n        super::runner().with_cwd({:?})",
                        name
                    )?;
                    if let Some(p) = options.precision {
                        write!(rs, ".set_precision({})", p)?;
                    }
                    writeln!(rs, "\n}}\n")?;
                } else {
                    rs.write_all(
                        b"#[allow(unused)]\nuse super::runner;\n\n",
                    )?;
                }
            }
            for name in tests {
                handle_hrx_part(
                    rs,
                    suite,
                    archive,
                    &format!("{}{}", prefix, name),
                    options.precision,
                    runner.clone().with_cwd(name),
                )?;
            }
            if name.is_some() {
                writeln!(rs, "}}")?;
            }
        }
        Ok(())
    }
}

fn fn_name_os(name: &OsStr) -> String {
    fn_name(&name.to_string_lossy())
}
fn fn_name(name: &str) -> String {
    use lazy_static::lazy_static;
    lazy_static! {
        static ref RUST_WORDS: WordSet<20> = WordSet::new([
            "as", "else", "false", "final", "for", "if", "in", "loop",
            "macro", "match", "mod", "override", "return", "static", "super",
            "true", "type", "use", "where", "while",
        ]);
    };
    let t = deunicode(name)
        .to_lowercase()
        .replace(".hrx", "")
        .replace('/', "_")
        .replace('-', "_")
        .replace('.', "_");
    if t.chars().next().unwrap_or('0').is_numeric() {
        format!("t{}", t)
    } else if RUST_WORDS.contains(&t) {
        format!("test_{}", t)
    } else {
        t
    }
}

struct WordSet<const N: usize> {
    words: [&'static str; N],
}
impl<const N: usize> WordSet<N> {
    fn new(words: [&'static str; N]) -> Self {
        assert!(is_sorted(&words));
        Self { words }
    }
    fn contains(&self, word: &str) -> bool {
        self.words.binary_search(&word).is_ok()
    }
}
fn is_sorted(data: &[&str]) -> bool {
    data.windows(2).all(|w| w[0] < w[1])
}

fn load_test_fixture_dir(
    specdir: &Path,
    precision: Option<usize>,
) -> Result<TestFixture, Error> {
    static INPUT_FILENAME: &str = "input.scss";
    static EXPECTED_OUTPUT_FILENAMES: &[&str] =
        &["output-dart-sass.css", "output.css"];
    static EXPECTED_ERROR_FILENAMES: &[&str] = &["error-dart-sass", "error"];

    let name = specdir.file_name().map(|s| s.to_string_lossy().to_string());
    let mut options = load_options(specdir)?;
    options.precision = options.precision.or(precision);
    let input = content(&specdir.join(INPUT_FILENAME))?;

    for filename in EXPECTED_ERROR_FILENAMES {
        let path = specdir.join(filename);
        if path.exists() {
            return Ok(TestFixture::new_err(
                name,
                input,
                content(&path)?.trim().to_string(),
                options,
            ));
        }
    }
    for filename in EXPECTED_OUTPUT_FILENAMES {
        let path = specdir.join(filename);
        if path.exists() {
            return Ok(TestFixture::new_ok(
                name,
                input,
                &content(&path)?,
                options,
            ));
        }
    }
    Err(Error(format!(
        "No expected CSS / error found for {}",
        specdir.file_name().unwrap_or_default().to_str().unwrap()
    )))
}

fn load_test_fixture_hrx(
    name: Option<String>,
    archive: &Archive,
    prefix: &str,
    options: Options,
) -> Result<TestFixture, Error> {
    static INPUT_FILENAME: &str = "input.scss";
    static EXPECTED_OUTPUT_FILENAMES: &[&str] =
        &["output-dart-sass.css", "output.css"];

    if let Some(input) = archive.get(&format!("{}{}", prefix, INPUT_FILENAME))
    {
        if let Some(error) =
            archive.get(&format!("{}{}", prefix, "error-dart-sass"))
        {
            return Ok(TestFixture::new_err(
                name,
                input.to_string(),
                error.trim().to_string(),
                options,
            ));
        }
        for filename in EXPECTED_OUTPUT_FILENAMES {
            if let Some(output) =
                archive.get(&format!("{}{}", prefix, filename))
            {
                return Ok(TestFixture::new_ok(
                    name,
                    input.to_string(),
                    output,
                    options,
                ));
            }
        }
        if let Some(error) = archive.get(&format!("{}{}", prefix, "error")) {
            return Ok(TestFixture::new_err(
                name,
                input.to_string(),
                error.trim().to_string(),
                options,
            ));
        }
        return Err(Error(format!(
            "No expected CSS / error found for {:?}",
            prefix,
        )));
    }
    Err(Error(format!("No input found for {:?}", prefix)))
}

/// Load options from options.yml.
fn load_options(path: &Path) -> Result<Options, Error> {
    let yml = path.join("options.yml");
    if yml.exists() {
        Options::parse(&content(&yml)?)
    } else {
        Ok(Options {
            precision: None,
            should_skip: None,
        })
    }
}

fn content(path: &Path) -> Result<String, io::Error> {
    let mut buf = String::new();
    File::open(path)?.read_to_string(&mut buf)?;
    Ok(buf)
}

#[derive(Debug)]
pub struct Error(String);
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
impl From<std::num::TryFromIntError> for Error {
    fn from(e: std::num::TryFromIntError) -> Self {
        Error(e.to_string())
    }
}

use std::fmt;
impl fmt::Display for Error {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(out)
    }
}
