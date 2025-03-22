//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/css_unary_ops.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("css_unary_ops")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  a: -0.5em;\
             \n  b: 0.5em;\
             \n  c: -foo(12px);\
             \n  d: +foo(12px); }\n"),
        "foo {\
         \n  a: -0.5em;\
         \n  b: 0.5em;\
         \n  c: -foo(12px);\
         \n  d: +foo(12px);\
         \n}\n"
    );
}
