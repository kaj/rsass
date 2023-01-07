//! Tests auto-converted from "sass-spec/spec/libsass/debug-directive-nested/mixin.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("mixin")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@mixin c() {\
             \n  @warn test;\
             \n  c: d;\
             \n}\n\
             \na {\
             \n  b: {\
             \n    @include c();\
             \n  }\
             \n}\n"),
        "a {\
         \n  b-c: d;\
         \n}\n"
    );
}
