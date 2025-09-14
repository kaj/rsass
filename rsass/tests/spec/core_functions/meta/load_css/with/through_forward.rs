//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load_css/with/through_forward.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("through_forward")
        .mock_file("as/_forwarded.scss", "$a: original !default;\nc {d: $a}\n")
        .mock_file("as/_loaded.scss", "@forward \"forwarded\" as b-*;\n")
        .mock_file("bare/_forwarded.scss", "$a: original !default;\nb {c: $a}\n")
        .mock_file("bare/_loaded.scss", "@forward \"forwarded\";\n")
        .mock_file("hide/_forwarded.scss", "$a: original !default;\nb {c: $a}\n")
        .mock_file("hide/_loaded.scss", "@forward \"forwarded\" hide $b;\n")
        .mock_file("show/_forwarded.scss", "$a: original !default;\nb {c: $a}\n")
        .mock_file("show/_loaded.scss", "@forward \"forwarded\" show $a;\n")
        .mock_file("transitive/_loaded.scss", "@forward \"midstream\";\n")
        .mock_file("transitive/_midstream.scss", "@forward \"upstream\";\n")
        .mock_file("transitive/_upstream.scss", "$a: original !default;\nb {c: $a}\n")
        .mock_file("with/default/_forwarded.scss", "$a: from forwarded !default;\nb {c: $a}\n")
        .mock_file("with/default/_loaded.scss", "@forward \"forwarded\" with ($a: from loaded !default);\n")
        .mock_file("with/null/_forwarded.scss", "$a: from forwarded !default;\nb {c: $a}\n")
        .mock_file("with/null/_loaded.scss", "@forward \"forwarded\" with ($a: from loaded !default);\n")
        .mock_file("with/unconfigured/_forwarded.scss", "$a: from forwarded !default;\n$b: from forwarded !default;\nc {\n  a: $a;\n  b: $b;\n}\n")
        .mock_file("with/unconfigured/_loaded.scss", "@forward \"forwarded\" with ($b: from loaded);\n")
}

#[test]
#[ignore] // wrong result
fn test_as() {
    let runner = runner().with_cwd("as");
    assert_eq!(
        runner.ok(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"loaded\", $with: (b-a: configured));\n"
        ),
        "c {\
         \n  d: configured;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn bare() {
    let runner = runner().with_cwd("bare");
    assert_eq!(
        runner.ok(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"loaded\", $with: (a: configured));\n"
        ),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn hide() {
    let runner = runner().with_cwd("hide");
    assert_eq!(
        runner.ok(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"loaded\", $with: (a: configured));\n"
        ),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn show() {
    let runner = runner().with_cwd("show");
    assert_eq!(
        runner.ok(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"loaded\", $with: (a: configured));\n"
        ),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn transitive() {
    let runner = runner().with_cwd("transitive");
    assert_eq!(
        runner.ok(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"loaded\", $with: (a: configured));\n"
        ),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
}
mod with {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("with")
    }

    #[test]
    fn default() {
        let runner = runner().with_cwd("default");
        assert_eq!(
            runner.ok(
                "@use \"sass:meta\";\
             \n@include meta.load-css(\"loaded\", $with: (a: from input));\n"
            ),
            "b {\
         \n  c: from input;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn null() {
        let runner = runner().with_cwd("null");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n@include meta.load-css(\"loaded\", $with: (a: null));\n"),
            "b {\
         \n  c: from loaded;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn unconfigured() {
        let runner = runner().with_cwd("unconfigured");
        assert_eq!(
            runner.ok(
                "@use \"sass:meta\";\
             \n@include meta.load-css(\"loaded\", $with: (a: from input));\n"
            ),
            "c {\
         \n  a: from input;\
         \n  b: from loaded;\
         \n}\n"
        );
    }
}
