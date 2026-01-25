//! Tests auto-converted from "sass-spec/spec/directives/import/configuration/import_twice.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("import_twice")
        .mock_file("no_change/_other.import.scss", "@forward \"other\";\n")
        .mock_file(
            "no_change/_other.scss",
            "$a: original !default;\nb {c: $a}\n",
        )
        .mock_file(
            "still_changes_in_same_file/_other.import.scss",
            "@forward \"other\";\n",
        )
        .mock_file(
            "still_changes_in_same_file/_other.scss",
            "$a: original !default;\nb {c: $a}\n",
        )
        .mock_file("with_change/_other.import.scss", "@forward \"other\";\n")
        .mock_file(
            "with_change/_other.scss",
            "$a: original !default;\nb {c: $a}\n",
        )
}

#[test]
#[ignore] // wrong result
fn no_change() {
    let runner = runner().with_cwd("no_change");
    assert_eq!(
        runner.ok("$a: configured;\
             \n@import \"other\";\
             \n@import \"other\";\n"),
        "b {\
         \n  c: configured;\
         \n}\
         \nb {\
         \n  c: configured;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn still_changes_in_same_file() {
    let runner = runner().with_cwd("still_changes_in_same_file");
    assert_eq!(
        runner.ok("@import \"other\";\
             \n$a: changed;\
             \n@import \"other\";\n\
             \nd {\
             \n  e: $a;\
             \n}\n"),
        "b {\
         \n  c: original;\
         \n}\
         \nb {\
         \n  c: original;\
         \n}\
         \nd {\
         \n  e: changed;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn with_change() {
    let runner = runner().with_cwd("with_change");
    assert_eq!(
        runner.ok("$a: configured;\
             \n@import \"other\";\
             \n$a: changed; // This should be ignored\
             \n@import \"other\";\n"),
        "b {\
         \n  c: configured;\
         \n}\
         \nb {\
         \n  c: configured;\
         \n}\n"
    );
}
