//! Tests auto-converted from "sass-spec/spec/non_conformant/misc/media_interpolation.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$baz: 12;\
             \n@media bar#{$baz} {a {b: c}}\n"),
        "@media bar12 {\
         \n  a {\
         \n    b: c;\
         \n  }\
         \n}\n"
    );
}
