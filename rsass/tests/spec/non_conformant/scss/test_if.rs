//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/if.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("if")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@if false {\
             \n  div {\
             \n    color: red;\
             \n  }\
             \n}\
             \n@else if true {\
             \n  span {\
             \n    color: blue;\
             \n  }\
             \n}\n\
             \ndiv {\
             \n  @if true {\
             \n    color: green;\
             \n  }\
             \n  @if false {\
             \n    height: 10px;\
             \n  }\
             \n  @else if false {\
             \n    height: 20px;\
             \n  }\
             \n  @else if false {\
             \n    height: 30px;\
             \n  }\
             \n  @else {\
             \n    height: 40px;\
             \n  }\
             \n}"),
        "span {\
         \n  color: blue;\
         \n}\
         \ndiv {\
         \n  color: green;\
         \n  height: 40px;\
         \n}\n"
    );
}
