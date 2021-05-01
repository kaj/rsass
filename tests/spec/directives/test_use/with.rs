//! Tests auto-converted from "sass-spec/spec/directives/use/with.hrx"

mod core_module {
    mod indirect {
        #[test]
        #[ignore] // unexepected error
        fn forward() {
            assert_eq!(
                crate::rsass(
                    "// Regression test for sass/dart-sass#838.\
            \n@use \"other\" with ($c: e);\
            \n\
            \na {b: other.$c}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: e;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn test_use() {
            assert_eq!(
                crate::rsass(
                    "// Regression test for sass/dart-sass#838.\
            \n@use \"other\" with ($c: e);\
            \n\
            \na {b: other.$c}\
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
}
#[test]
#[ignore] // unexepected error
fn dash_insensitive() {
    assert_eq!(
        crate::rsass(
            "@use \"other\" with ($a_b: configured);\
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
            "@use \"other\" with ($a: configured);\
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
fn from_variable() {
    assert_eq!(
        crate::rsass(
            "$a: configured;\
            \n@use \"other\" with ($a: $a);\
            \n"
        )
        .unwrap(),
        "b {\
        \n  c: configured;\
        \n}\
        \n"
    );
}
mod multi_load {
    #[test]
    #[ignore] // unexepected error
    fn forward() {
        assert_eq!(
            crate::rsass(
                "@use \"upstream\" with ($a: configured);\
            \n@use \"midstream\";\
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
    fn transitive() {
        assert_eq!(
            crate::rsass(
                "// Regression test for sass/dart-sass#854.\
            \n@use \"midstream1\" with ($a: overridden 1);\
            \n@use \"midstream2\" with ($a: overridden 2);\
            \n\
            \nb {\
            \n  midstream1: midstream1.$a;\
            \n  midstream2: midstream2.$a;\
            \n}\
            \n"
            )
            .unwrap(),
            "c {\
        \n  d: e;\
        \n}\
        \nb {\
        \n  midstream1: overridden 1;\
        \n  midstream2: overridden 2;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn test_use() {
        assert_eq!(
            crate::rsass(
                "@use \"upstream\" with ($a: configured);\
            \n@use \"midstream\";\
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
            "@use \"other\" with (\
            \n  $a: configured a,\
            \n  $b: configured b,\
            \n  $c: configured c\
            \n);\
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
            "@use \"other\" with ($a: configured);\
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
            "@use \"other\" with ($a: configured a);\
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
    fn and_use() {
        assert_eq!(
            crate::rsass(
                "// Regression test for sass/sass#2744.\
            \n@use \"forwarder\" with ($c: e);\
            \n\
            \na {b: forwarder.$c}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: e;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn test_as() {
        assert_eq!(
            crate::rsass(
                "@use \"used\" with ($b-a: configured);\
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
                "@use \"used\" with ($a: configured);\
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
                "@use \"used\" with ($a: configured);\
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
                "@use \"used\" with ($a: configured);\
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
                "@use \"used\" with ($a: configured);\
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
                    "@use \"used\" with ($a: from input);\
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
                    "@use \"used\" with ($a: null);\
            \n"
                )
                .unwrap(),
                "b {\
        \n  c: from used;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn unconfigured() {
            assert_eq!(
                crate::rsass(
                    "@use \"used\" with ($a: from input);\
            \n"
                )
                .unwrap(),
                "c {\
        \n  a: from input;\
        \n  b: from used;\
        \n}\
        \n"
            );
        }
    }
    #[test]
    #[ignore] // unexepected error
    fn with_unrelated_config() {
        assert_eq!(
            crate::rsass(
                "@use \"used\" with ($from-used: configured);\
            \n"
            )
            .unwrap(),
            "b {\
        \n  from-forwarded: original;\
        \n}\
        \na {\
        \n  from-used: configured;\
        \n}\
        \n"
        );
    }
}
mod through_import {
    #[test]
    #[ignore] // unexepected error
    fn direct() {
        assert_eq!(
            crate::rsass(
                "@use \"used\" with ($a: configured);\
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
                "@use \"used\" with ($a: configured);\
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
fn trailing_comma() {
    assert_eq!(
        crate::rsass(
            "@use \"other\" with ($a: configured,);\
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
fn used_in_input() {
    assert_eq!(
        crate::rsass(
            "@use \"other\" with ($a: configured);\
            \nb {c: other.$a}\
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
fn variable_exists() {
    assert_eq!(
        crate::rsass(
            "@use \"other\" with ($a: configured);\
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
