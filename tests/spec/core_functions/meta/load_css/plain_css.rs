//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load_css/plain_css.hrx"

#[test]
#[ignore] // unexepected error
fn at_rule() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
            \n@include meta.load-css(\"other\");\
            \n"
        )
        .unwrap(),
        "@media screen {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n"
    );
}
mod empty {
    #[test]
    #[ignore] // unexepected error
    fn built_in() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:meta\";\
            \n@include meta.load-css(\"sass:color\");\
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
    fn user_defined() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:meta\";\
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
fn named() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
            \n@include meta.load-css($url: \"other\");\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: c;\
        \n}\
        \n"
    );
}
mod nested {
    #[test]
    #[ignore] // unexepected error
    fn media_query() {
        assert_eq!(
            crate::rsass(
                "// Regression test for dart-sass#843\
            \n@use \"sass:meta\";\
            \n@include meta.load-css(\"midstream\")\
            \n"
            )
            .unwrap(),
            "/**/\
        \n@media b {\
        \n  a {\
        \n    c: d;\
        \n  }\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn parent_selector() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:meta\";\
            \na {@include meta.load-css(\"other\")}\
            \n"
            )
            .unwrap(),
            "a c b {\
        \n  x: y;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn plain_plain_css() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:meta\";\
            \na {@include meta.load-css(\"other\")}\
            \n"
            )
            .unwrap(),
            "a b {\
        \n  c: d;\
        \n}\
        \n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn plain_css_import() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
            \n\
            \na {b: c}\
            \n\
            \n@include meta.load-css(\"other\");\
            \n"
        )
        .unwrap(),
        "@import \"style.css\";\
        \na {\
        \n  b: c;\
        \n}\
        \nd {\
        \n  e: f;\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // unexepected error
fn style_rule() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
            \n@include meta.load-css(\"other\");\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: c;\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // unexepected error
fn through_other_mixin() {
    assert_eq!(
        crate::rsass(
            "@use \"subdir/midstream\";\
            \n@include midstream.load-css(\"upstream\");\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: in subdir;\
        \n}\
        \n"
    );
}
