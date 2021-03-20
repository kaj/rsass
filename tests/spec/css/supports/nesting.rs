//! Tests auto-converted from "sass-spec/spec/css/supports/nesting.hrx"

#[test]
#[ignore] // wrong result
fn empty() {
    assert_eq!(
        crate::rsass(
            "@supports (a: b) {}\
            \n"
        )
        .unwrap(),
        ""
    );
}
#[test]
#[ignore] // wrong result
fn invisible() {
    assert_eq!(
        crate::rsass(
            "@supports (a: b) {\
            \n  %c {d: e}\
            \n}\
            \n"
        )
        .unwrap(),
        ""
    );
}
#[test]
fn loud_comment() {
    assert_eq!(
        crate::rsass(
            "// Regression test for sass/libsass#2158\
            \n\
            \n@supports (a: b) {\
            \n  /* c */\
            \n  d {e: f}\
            \n}\
            \n"
        )
        .unwrap(),
        "@supports (a: b) {\
        \n  /* c */\
        \n  d {\
        \n    e: f;\
        \n  }\
        \n}\
        \n"
    );
}
mod media {
    #[test]
    #[ignore] // wrong result
    fn in_style_rule() {
        assert_eq!(
            crate::rsass(
                "c {\
            \n  @media screen {\
            \n    @supports (a: b) {d: e}\
            \n  }\
            \n}\
            \n"
            )
            .unwrap(),
            "@media screen {\
        \n  @supports (a: b) {\
        \n    c {\
        \n      d: e;\
        \n    }\
        \n  }\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn top() {
        assert_eq!(
            crate::rsass(
                "@media screen {\
            \n  @supports (a: b) {\
            \n    c {d: e}\
            \n  }\
            \n}\
            \n"
            )
            .unwrap(),
            "@media screen {\
        \n  @supports (a: b) {\
        \n    c {\
        \n      d: e;\
        \n    }\
        \n  }\
        \n}\
        \n"
        );
    }
}
#[test]
fn style_rule() {
    assert_eq!(
        crate::rsass(
            "a {\
            \n  @supports (b: c) {d: e}\
            \n}\
            \n"
        )
        .unwrap(),
        "@supports (b: c) {\
        \n  a {\
        \n    d: e;\
        \n  }\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // wrong result
fn supports() {
    assert_eq!(
        crate::rsass(
            "@supports (a: b) {\
            \n  @supports (c: d) {\
            \n    e {f: g}\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "@supports (a: b) {\
        \n  @supports (c: d) {\
        \n    e {\
        \n      f: g;\
        \n    }\
        \n  }\
        \n}\
        \n"
    );
}
