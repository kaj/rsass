//! Tests auto-converted from "sass-spec/spec/directives/import/configuration.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("configuration")
        .mock_file(
            "import_twice/no_change/_other.import.scss",
            "@forward \"other\";\n",
        )
        .mock_file(
            "import_twice/no_change/_other.scss",
            "$a: original !default;\nb {c: $a}\n",
        )
        .mock_file(
            "import_twice/still_changes_in_same_file/_other.import.scss",
            "@forward \"other\";\n",
        )
        .mock_file(
            "import_twice/still_changes_in_same_file/_other.scss",
            "$a: original !default;\nb {c: $a}\n",
        )
        .mock_file(
            "import_twice/with_change/_other.import.scss",
            "@forward \"other\";\n",
        )
        .mock_file(
            "import_twice/with_change/_other.scss",
            "$a: original !default;\nb {c: $a}\n",
        )
        .mock_file(
            "indirect/through_forward/_midstream.import.scss",
            "@forward \"midstream\";\n",
        )
        .mock_file(
            "indirect/through_forward/_midstream.scss",
            "@forward \"upstream\";\n",
        )
        .mock_file(
            "indirect/through_forward/_upstream.import.scss",
            "@forward \"upstream\";\n",
        )
        .mock_file(
            "indirect/through_forward/_upstream.scss",
            "$a: original !default;\nb {c: $a}\n",
        )
        .mock_file(
            "indirect/through_import/_midstream.scss",
            "@import \"upstream\";\n",
        )
        .mock_file(
            "indirect/through_import/_upstream.import.scss",
            "@forward \"upstream\";\n",
        )
        .mock_file(
            "indirect/through_import/_upstream.scss",
            "$a: original !default;\nb {c: $a}\n",
        )
        .mock_file(
            "midstream_definition/no_config/_midstream.scss",
            "$a: midstream;\n@forward \"upstream\";\n",
        )
        .mock_file(
            "midstream_definition/no_config/_upstream.scss",
            "$a: original !default;\nb {c: $a}\n",
        )
        .mock_file(
            "midstream_definition/with_config/_midstream.scss",
            "$a: midstream;\n@forward \"upstream\";\n",
        )
        .mock_file(
            "midstream_definition/with_config/_upstream.scss",
            "$a: original !default;\nb {c: $a}\n",
        )
        .mock_file("nested/_midstream.scss", "@forward \"upstream\";\n")
        .mock_file(
            "nested/_upstream.scss",
            "$a: original !default;\nb {c: $a}\n",
        )
        .mock_file(
            "prefixed_as/_midstream.scss",
            "@forward \"upstream\" as d-*;\n",
        )
        .mock_file(
            "prefixed_as/_upstream.scss",
            "$a: original !default;\nb {c: $a}\n",
        )
        .mock_file("same_file/_midstream.scss", "@forward \"upstream\";\n")
        .mock_file(
            "same_file/_upstream.scss",
            "$a: original !default;\nb {c: $a}\n",
        )
        .mock_file("separate_file/_config.scss", "$a: configured;\n")
        .mock_file(
            "separate_file/_midstream.scss",
            "@forward \"upstream\";\n",
        )
        .mock_file(
            "separate_file/_upstream.scss",
            "$a: original !default;\nb {c: $a}\n",
        )
        .mock_file(
            "unrelated_variable/_midstream.scss",
            "@forward \"upstream\";\n",
        )
        .mock_file(
            "unrelated_variable/_upstream.scss",
            "$a: original !default;\nb {c: $a}\n",
        )
}

mod import_twice {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("import_twice")
    }

    #[test]
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
}
mod indirect {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("indirect")
    }

    #[test]
    #[ignore] // wrong result
    fn through_forward() {
        let runner = runner().with_cwd("through_forward");
        assert_eq!(
            runner.ok("$a: configured;\
             \n@import \"midstream\";\n"),
            "b {\
         \n  c: configured;\
         \n}\n"
        );
    }
    #[test]
    fn through_import() {
        let runner = runner().with_cwd("through_import");
        assert_eq!(
            runner.ok("$a: configured;\
             \n@import \"midstream\";\n"),
            "b {\
         \n  c: configured;\
         \n}\n"
        );
    }
}
mod midstream_definition {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("midstream_definition")
    }

    #[test]
    fn no_config() {
        let runner = runner().with_cwd("no_config");
        assert_eq!(
            runner.ok("@import \"midstream\";\n"),
            "b {\
         \n  c: original;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn with_config() {
        let runner = runner().with_cwd("with_config");
        assert_eq!(
            runner.ok("$a: configured;\
             \n@import \"midstream\";\n"),
            "b {\
         \n  c: configured;\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // wrong result
fn nested() {
    let runner = runner().with_cwd("nested");
    assert_eq!(
        runner.ok("a {\
             \n  $a: configured;\
             \n  @import \"midstream\";\
             \n}\n"),
        "a b {\
         \n  c: configured;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn prefixed_as() {
    let runner = runner().with_cwd("prefixed_as");
    assert_eq!(
        runner.ok("$d-a: configured;\
             \n@import \"midstream\";\n"),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn same_file() {
    let runner = runner().with_cwd("same_file");
    assert_eq!(
        runner.ok("$a: configured;\
             \n@import \"midstream\";\n"),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn separate_file() {
    let runner = runner().with_cwd("separate_file");
    assert_eq!(
        runner.ok("@import \"config\";\
             \n@import \"midstream\";\n"),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn unrelated_variable() {
    let runner = runner().with_cwd("unrelated_variable");
    assert_eq!(
        runner.ok("$a: configured;\
             \n$d: other;\
             \n@import \"midstream\";\n"),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
}
