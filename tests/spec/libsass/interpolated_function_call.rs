//! Tests auto-converted from "sass-spec/spec/libsass/interpolated-function-call.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("interpolated-function-call")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$f: foo;\n\
             \ndiv {\
             \n  color: #{$f}(a, 1+2, c);\
             \n}"),
        "div {\
         \n  color: foo(a, 3, c);\
         \n}\n"
    );
}
