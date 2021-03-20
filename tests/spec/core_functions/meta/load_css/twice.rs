//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load_css/twice.hrx"

mod load_css {
    #[test]
    #[ignore] // unexepected error
    fn different_extend() {
        assert_eq!(
            crate::rsass(
                "@use \"left\";\
            \n@use \"right\";\
            \n"
            )
            .unwrap(),
            "a, left {\
        \n  b: c;\
        \n}\
        \na, right {\
        \n  b: c;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn different_nesting() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:meta\";\
            \na {@include meta.load-css(\"other\")}\
            \nb {@include meta.load-css(\"other\")}\
            \n"
            )
            .unwrap(),
            "a c {\
        \n  d: e;\
        \n}\
        \nb c {\
        \n  d: e;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn runs_once() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:meta\";\
            \n@include meta.load-css(\"other\");\
            \n@include meta.load-css(\"other\");\
            \n\
            \n/* No output other than this */\
            \n"
            )
            .unwrap(),
            "/* No output other than this */\
        \n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn shares_state() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
            \n@use \"shared\";\
            \n@include meta.load-css(\"other\");\
            \n\
            \na {shared-b: shared.$b}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  shared-b: value set by other;\
        \n}\
        \n"
    );
}
mod test_use {
    #[test]
    #[ignore] // unexepected error
    fn different_extend() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:meta\";\
            \n@use \"midstream\";\
            \n@include meta.load-css(\"other\")\
            \n"
            )
            .unwrap(),
            "b, a {\
        \n  c: d;\
        \n}\
        \nb {\
        \n  c: d;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn different_nesting() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:meta\";\
            \n@use \"other\";\
            \na {@include meta.load-css(\"other\")}\
            \n"
            )
            .unwrap(),
            "b {\
        \n  c: d;\
        \n}\
        \na b {\
        \n  c: d;\
        \n}\
        \n"
        );
    }
    mod runs_once {
        #[test]
        #[ignore] // unexepected error
        fn different_text() {
            assert_eq!(
                crate::rsass(
                    "@use \"sass:meta\";\
            \n@use \"other\";\
            \n@include meta.load-css(\"_other\");\
            \n\
            \n/* No output other than this */\
            \n"
                )
                .unwrap(),
                "/* No output other than this */\
        \n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn same_text() {
            assert_eq!(
                crate::rsass(
                    "@use \"sass:meta\";\
            \n@use \"other\";\
            \n@include meta.load-css(\"other\");\
            \n\
            \n/* No output other than this */\
            \n"
                )
                .unwrap(),
                "/* No output other than this */\
        \n"
            );
        }
    }
}
