//! Tests auto-converted from "sass-spec/spec/libsass/debug-directive-nested/function.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("function")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@function c() {\
             \n  @warn test;\
             \n  @return d;\
             \n}\n\
             \na {\
             \n  b: {\
             \n    c: c();\
             \n  }\
             \n}\n"),
        "a {\
         \n  b-c: d;\
         \n}\n"
    );
}
