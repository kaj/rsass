//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load_css/with.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .mock_file("core_module/indirect/_other.scss", "@use \"sass:color\";\n\n$c: d !default;\n\na {b: $c}\n")
        .mock_file("dash_insensitive/_other.scss", "$a-b: original !default;\nb {c: $a-b}\n")
        .mock_file("doesnt_run_default/_other.scss", "// This will throw an error if it\'s evaluated, but it shouldn\'t be because `$a`\n// already has a value.\n$a: 1px + 1em !default;\nb {c: $a}\n")
        .mock_file("empty/_other.scss", "a {b: c}\n")
        .mock_file("multi_load/empty/_midstream.scss", "@use \"upstream\";\nb {c: upstream.$a}\n")
        .mock_file("multi_load/empty/_upstream.scss", "$a: original !default;\n")
        .mock_file("multi_load/forward/_loads.scss", "@use \"sass:meta\";\n@include meta.load-css(\"upstream\", $with: (a: configured));\n")
        .mock_file("multi_load/forward/_midstream.scss", "@forward \"upstream\";\n")
        .mock_file("multi_load/forward/_upstream.scss", "$a: original !default;\n")
        .mock_file("multi_load/use/_midstream.scss", "@use \"upstream\";\nb {c: upstream.$a}\n")
        .mock_file("multi_load/use/_upstream.scss", "$a: original !default;\n")
        .mock_file("multiple/_other.scss", "$a: original a !default;\n$b: original b !default;\n$c: original c !default;\n\nd {\n  a: $a;\n  b: $b;\n  c: $c;\n}\n")
        .mock_file("null/_other.scss", "$a: original !default;\nb {c: $a}\n")
        .mock_file("single/_other.scss", "$a: original !default;\nb {c: $a}\n")
        .mock_file("some_unconfigured/_other.scss", "$a: original a !default;\n$b: original b !default;\n\nc {\n  a: $a;\n  b: $b;\n}\n")
        .mock_file("through_forward/as/_forwarded.scss", "$a: original !default;\nc {d: $a}\n")
        .mock_file("through_forward/as/_loaded.scss", "@forward \"forwarded\" as b-*;\n")
        .mock_file("through_forward/bare/_forwarded.scss", "$a: original !default;\nb {c: $a}\n")
        .mock_file("through_forward/bare/_loaded.scss", "@forward \"forwarded\";\n")
        .mock_file("through_forward/hide/_forwarded.scss", "$a: original !default;\nb {c: $a}\n")
        .mock_file("through_forward/hide/_loaded.scss", "@forward \"forwarded\" hide $b;\n")
        .mock_file("through_forward/show/_forwarded.scss", "$a: original !default;\nb {c: $a}\n")
        .mock_file("through_forward/show/_loaded.scss", "@forward \"forwarded\" show $a;\n")
        .mock_file("through_forward/transitive/_loaded.scss", "@forward \"midstream\";\n")
        .mock_file("through_forward/transitive/_midstream.scss", "@forward \"upstream\";\n")
        .mock_file("through_forward/transitive/_upstream.scss", "$a: original !default;\nb {c: $a}\n")
        .mock_file("through_forward/with/default/_forwarded.scss", "$a: from forwarded !default;\nb {c: $a}\n")
        .mock_file("through_forward/with/default/_loaded.scss", "@forward \"forwarded\" with ($a: from loaded !default);\n")
        .mock_file("through_forward/with/null/_forwarded.scss", "$a: from forwarded !default;\nb {c: $a}\n")
        .mock_file("through_forward/with/null/_loaded.scss", "@forward \"forwarded\" with ($a: from loaded !default);\n")
        .mock_file("through_forward/with/unconfigured/_forwarded.scss", "$a: from forwarded !default;\n$b: from forwarded !default;\nc {\n  a: $a;\n  b: $b;\n}\n")
        .mock_file("through_forward/with/unconfigured/_loaded.scss", "@forward \"forwarded\" with ($b: from loaded);\n")
        .mock_file("through_import/direct/_imported.scss", "$a: original !default;\nb {c: $a}\n")
        .mock_file("through_import/direct/_loaded.scss", "@import \"imported\";\n")
        .mock_file("through_import/transitive/_loaded.scss", "@import \"midstream\";\n")
        .mock_file("through_import/transitive/_midstream.scss", "@import \"upstream\";\n")
        .mock_file("through_import/transitive/_upstream.scss", "$a: original !default;\nb {c: $a}\n")
        .mock_file("variable_exists/_other.scss", "$before-declaration: variable-exists(a);\n$a: original !default;\nb {\n  before-declaration: $before-declaration;\n  after-declaration: variable-exists(a);\n}\n")
}

mod core_module {
    #[allow(unused)]
    use super::runner;
    #[test]
    #[ignore] // unexepected error
    fn indirect() {
        assert_eq!(
            runner().ok("// Regression test for sass/dart-sass#838.\
             \n@use \"sass:meta\";\
             \n@include meta.load-css(\"other\", $with: (c: e));\n"),
            "a {\
         \n  b: e;\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn dash_insensitive() {
    assert_eq!(
        runner().ok(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"other\", $with: (a_b: configured));\n"
        ),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn doesnt_run_default() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \n@include meta.load-css(\"other\", $with: (a: configured));\n"),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn empty() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \n@include meta.load-css(\"other\", $with: ());\n"),
        "a {\
         \n  b: c;\
         \n}\n"
    );
}
mod multi_load {
    #[allow(unused)]
    use super::runner;
    #[test]
    #[ignore] // unexepected error
    fn empty() {
        assert_eq!(
        runner().ok(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"upstream\", $with: (a: configured));\n\
             \n// An empty configuration map counts as no configuration.\
             \n@include meta.load-css(\"midstream\", $with: ());\n"
        ),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn forward() {
        assert_eq!(
        runner().ok(
            "// This indirection is necessary so that we can execute `meta.load-css()` before\
             \n// we begin loading `used`.\
             \n@use \"loads\";\
             \n@use \"midstream\";\n\
             \nb {c: midstream.$a}\n"
        ),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn test_use() {
        assert_eq!(
        runner().ok(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"upstream\", $with: (a: configured));\n\
             \n// We have to load this dynamically, because we can\'t have a `@use` after an\
             \n// `@include`.\
             \n@include meta.load-css(\"midstream\");\n"
        ),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn multiple() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \n@include meta.load-css(\"other\", $with: (\
             \n  a: configured a,\
             \n  b: configured b,\
             \n  c: configured c\
             \n));\n"),
        "d {\
         \n  a: configured a;\
         \n  b: configured b;\
         \n  c: configured c;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn null() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \n@include meta.load-css(\"other\", $with: (a: null));\n"),
        "b {\
         \n  c: original;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn single() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \n@include meta.load-css(\"other\", $with: (a: configured));\n"),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn some_unconfigured() {
    assert_eq!(
        runner().ok(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"other\", $with: (a: configured a));\n"
        ),
        "c {\
         \n  a: configured a;\
         \n  b: original b;\
         \n}\n"
    );
}
mod through_forward {
    #[allow(unused)]
    use super::runner;
    #[test]
    #[ignore] // unexepected error
    fn test_as() {
        assert_eq!(
        runner().ok(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"loaded\", $with: (b-a: configured));\n"
        ),
        "c {\
         \n  d: configured;\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn bare() {
        assert_eq!(
            runner().ok(
                "@use \"sass:meta\";\
             \n@include meta.load-css(\"loaded\", $with: (a: configured));\n"
            ),
            "b {\
         \n  c: configured;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn hide() {
        assert_eq!(
            runner().ok(
                "@use \"sass:meta\";\
             \n@include meta.load-css(\"loaded\", $with: (a: configured));\n"
            ),
            "b {\
         \n  c: configured;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn show() {
        assert_eq!(
            runner().ok(
                "@use \"sass:meta\";\
             \n@include meta.load-css(\"loaded\", $with: (a: configured));\n"
            ),
            "b {\
         \n  c: configured;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn transitive() {
        assert_eq!(
            runner().ok(
                "@use \"sass:meta\";\
             \n@include meta.load-css(\"loaded\", $with: (a: configured));\n"
            ),
            "b {\
         \n  c: configured;\
         \n}\n"
        );
    }
    mod with {
        #[allow(unused)]
        use super::runner;
        #[test]
        #[ignore] // unexepected error
        fn default() {
            assert_eq!(
                runner().ok(
                    "@use \"sass:meta\";\
             \n@include meta.load-css(\"loaded\", $with: (a: from input));\n"
                ),
                "b {\
         \n  c: from input;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn null() {
            assert_eq!(
                runner().ok("@use \"sass:meta\";\
             \n@include meta.load-css(\"loaded\", $with: (a: null));\n"),
                "b {\
         \n  c: from loaded;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn unconfigured() {
            assert_eq!(
                runner().ok(
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
}
mod through_import {
    #[allow(unused)]
    use super::runner;
    #[test]
    #[ignore] // unexepected error
    fn direct() {
        assert_eq!(
            runner().ok(
                "@use \"sass:meta\";\
             \n@include meta.load-css(\"loaded\", $with: (a: configured));\n"
            ),
            "b {\
         \n  c: configured;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn transitive() {
        assert_eq!(
            runner().ok(
                "@use \"sass:meta\";\
             \n@include meta.load-css(\"loaded\", $with: (a: configured));\n"
            ),
            "b {\
         \n  c: configured;\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn variable_exists() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \n@include meta.load-css(\"other\", $with: (a: configured));\n"),
        "b {\
         \n  before-declaration: false;\
         \n  after-declaration: true;\
         \n}\n"
    );
}
