//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_472.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("div {\
             \n  display: block;\
             \n  @keyframes {\
             \n    from {\
             \n      foo: bar;\
             \n    }\
             \n  }\
             \n}\n"),
        "div {\
         \n  display: block;\
         \n}\
         \n@keyframes {\
         \n  from {\
         \n    foo: bar;\
         \n  }\
         \n}\n"
    );
}
