//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/if-in-propset.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("div {\
             \n  prop: {\
             \n    a: \"hello\";\
             \n    b: \"goodbye\";\
             \n    @if true {\
             \n      c: \"badbye\";\
             \n    }\
             \n  }\
             \n}"),
        "div {\
         \n  prop-a: \"hello\";\
         \n  prop-b: \"goodbye\";\
         \n  prop-c: \"badbye\";\
         \n}\n"
    );
}
