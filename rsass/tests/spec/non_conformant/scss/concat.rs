//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/concat.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("concat")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("div {\
             \n  a: hello + \"goodbye\";\
             \n  b: \"hello\" + goodbye;\
             \n  c: 3 + \"hello\";\
             \n  d: \"hello\" + 3;\
             \n}"),
        "div {\
         \n  a: hellogoodbye;\
         \n  b: \"hellogoodbye\";\
         \n  c: \"3hello\";\
         \n  d: \"hello3\";\
         \n}\n"
    );
}
