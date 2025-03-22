//! Tests auto-converted from "sass-spec/spec/css/supports/nesting.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("nesting")
}

#[test]
#[ignore] // wrong result
fn empty() {
    assert_eq!(runner().ok("@supports (a: b) {}\n"), "");
}
#[test]
#[ignore] // wrong result
fn invisible() {
    assert_eq!(
        runner().ok("@supports (a: b) {\
             \n  %c {d: e}\
             \n}\n"),
        ""
    );
}
#[test]
fn loud_comment() {
    assert_eq!(
        runner().ok("// Regression test for sass/libsass#2158\n\
             \n@supports (a: b) {\
             \n  /* c */\
             \n  d {e: f}\
             \n}\n"),
        "@supports (a: b) {\
         \n  /* c */\
         \n  d {\
         \n    e: f;\
         \n  }\
         \n}\n"
    );
}
mod media {
    use super::runner;

    #[test]
    fn in_style_rule() {
        assert_eq!(
            runner().ok("c {\
             \n  @media screen {\
             \n    @supports (a: b) {d: e}\
             \n  }\
             \n}\n"),
            "@media screen {\
         \n  @supports (a: b) {\
         \n    c {\
         \n      d: e;\
         \n    }\
         \n  }\
         \n}\n"
        );
    }
    #[test]
    fn top() {
        assert_eq!(
            runner().ok("@media screen {\
             \n  @supports (a: b) {\
             \n    c {d: e}\
             \n  }\
             \n}\n"),
            "@media screen {\
         \n  @supports (a: b) {\
         \n    c {\
         \n      d: e;\
         \n    }\
         \n  }\
         \n}\n"
        );
    }
}
#[test]
fn style_rule() {
    assert_eq!(
        runner().ok("a {\
             \n  @supports (b: c) {d: e}\
             \n}\n"),
        "@supports (b: c) {\
         \n  a {\
         \n    d: e;\
         \n  }\
         \n}\n"
    );
}
#[test]
fn supports() {
    assert_eq!(
        runner().ok("@supports (a: b) {\
             \n  @supports (c: d) {\
             \n    e {f: g}\
             \n  }\
             \n}\n"),
        "@supports (a: b) {\
         \n  @supports (c: d) {\
         \n    e {\
         \n      f: g;\
         \n    }\
         \n  }\
         \n}\n"
    );
}
