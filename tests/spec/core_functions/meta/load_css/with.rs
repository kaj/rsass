//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load_css/with.hrx"

mod core_module {
    #[test]
    #[ignore] // unexepected error
    fn indirect() {
        assert_eq!(
            crate::rsass(
                "// Regression test for sass/dart-sass#838.\
            \n@use \"sass:meta\";\
            \n@include meta.load-css(\"other\", $with: (c: e));\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: e;\
        \n}\
        \n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn dash_insensitive() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
            \n@include meta.load-css(\"other\", $with: (a_b: configured));\
            \n"
        )
        .unwrap(),
        "b {\
        \n  c: configured;\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // unexepected error
fn doesnt_run_default() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
            \n@include meta.load-css(\"other\", $with: (a: configured));\
            \n"
        )
        .unwrap(),
        "b {\
        \n  c: configured;\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // unexepected error
fn empty() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
            \n@include meta.load-css(\"other\", $with: ());\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: c;\
        \n}\
        \n"
    );
}
mod multi_load {
    #[test]
    #[ignore] // unexepected error
    fn empty() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:meta\";\
            \n@include meta.load-css(\"upstream\", $with: (a: configured));\
            \n\
            \n// An empty configuration map counts as no configuration.\
            \n@include meta.load-css(\"midstream\", $with: ());\
            \n"
            )
            .unwrap(),
            "b {\
        \n  c: configured;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn forward() {
        assert_eq!(
        crate::rsass(
            "// This indirection is necessary so that we can execute `meta.load-css()` before\
            \n// we begin loading `used`.\
            \n@use \"loads\";\
            \n@use \"midstream\";\
            \n\
            \nb {c: midstream.$a}\
            \n"
        )
        .unwrap(),
        "b {\
        \n  c: configured;\
        \n}\
        \n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn test_use() {
        assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
            \n@include meta.load-css(\"upstream\", $with: (a: configured));\
            \n\
            \n// We have to load this dynamically, because we can\'t have a `@use` after an\
            \n// `@include`.\
            \n@include meta.load-css(\"midstream\");\
            \n"
        )
        .unwrap(),
        "b {\
        \n  c: configured;\
        \n}\
        \n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn multiple() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
            \n@include meta.load-css(\"other\", $with: (\
            \n  a: configured a,\
            \n  b: configured b,\
            \n  c: configured c\
            \n));\
            \n"
        )
        .unwrap(),
        "d {\
        \n  a: configured a;\
        \n  b: configured b;\
        \n  c: configured c;\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // unexepected error
fn single() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
            \n@include meta.load-css(\"other\", $with: (a: configured));\
            \n"
        )
        .unwrap(),
        "b {\
        \n  c: configured;\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // unexepected error
fn some_unconfigured() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
            \n@include meta.load-css(\"other\", $with: (a: configured a));\
            \n"
        )
        .unwrap(),
        "c {\
        \n  a: configured a;\
        \n  b: original b;\
        \n}\
        \n"
    );
}
mod through_forward {
    #[test]
    #[ignore] // unexepected error
    fn test_as() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:meta\";\
            \n@include meta.load-css(\"loaded\", $with: (b-a: configured));\
            \n"
            )
            .unwrap(),
            "c {\
        \n  d: configured;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn bare() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:meta\";\
            \n@include meta.load-css(\"loaded\", $with: (a: configured));\
            \n"
            )
            .unwrap(),
            "b {\
        \n  c: configured;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn hide() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:meta\";\
            \n@include meta.load-css(\"loaded\", $with: (a: configured));\
            \n"
            )
            .unwrap(),
            "b {\
        \n  c: configured;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn show() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:meta\";\
            \n@include meta.load-css(\"loaded\", $with: (a: configured));\
            \n"
            )
            .unwrap(),
            "b {\
        \n  c: configured;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn transitive() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:meta\";\
            \n@include meta.load-css(\"loaded\", $with: (a: configured));\
            \n"
            )
            .unwrap(),
            "b {\
        \n  c: configured;\
        \n}\
        \n"
        );
    }
    mod with {
        #[test]
        #[ignore] // unexepected error
        fn default() {
            assert_eq!(
                crate::rsass(
                    "@use \"sass:meta\";\
            \n@include meta.load-css(\"loaded\", $with: (a: from input));\
            \n"
                )
                .unwrap(),
                "b {\
        \n  c: from input;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn null() {
            assert_eq!(
                crate::rsass(
                    "@use \"sass:meta\";\
            \n@include meta.load-css(\"loaded\", $with: (a: null));\
            \n"
                )
                .unwrap(),
                "b {\
        \n  c: from loaded;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn unconfigured() {
            assert_eq!(
                crate::rsass(
                    "@use \"sass:meta\";\
            \n@include meta.load-css(\"loaded\", $with: (a: from input));\
            \n"
                )
                .unwrap(),
                "c {\
        \n  a: from input;\
        \n  b: from loaded;\
        \n}\
        \n"
            );
        }
    }
}
mod through_import {
    #[test]
    #[ignore] // unexepected error
    fn direct() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:meta\";\
            \n@include meta.load-css(\"loaded\", $with: (a: configured));\
            \n"
            )
            .unwrap(),
            "b {\
        \n  c: configured;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn transitive() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:meta\";\
            \n@include meta.load-css(\"loaded\", $with: (a: configured));\
            \n"
            )
            .unwrap(),
            "b {\
        \n  c: configured;\
        \n}\
        \n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn variable_exists() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
            \n@include meta.load-css(\"other\", $with: (a: configured));\
            \n"
        )
        .unwrap(),
        "b {\
        \n  before-declaration: false;\
        \n  after-declaration: true;\
        \n}\
        \n"
    );
}
