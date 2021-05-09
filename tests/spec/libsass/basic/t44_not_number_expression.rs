//! Tests auto-converted from "sass-spec/spec/libsass/basic/44_not_number_expression.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@if 2 {\
             \n  div {\
             \n    background: green;\
             \n  }\
             \n}\
             \n@if not 2 {\
             \n  div {\
             \n    background: red;\
             \n  }\
             \n}\
             \n@if not not 2 {\
             \n  div {\
             \n    background: blue;\
             \n  }\
             \n}\n"),
        "div {\
         \n  background: green;\
         \n}\
         \ndiv {\
         \n  background: blue;\
         \n}\n"
    );
}
