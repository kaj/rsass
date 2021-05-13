//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1915.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("@mixin wrapper() {\
             \n  .wrapped {\
             \n    @content;\
             \n  }\
             \n}\n\
             \n%ext {\
             \n  background: #000;\
             \n}\n\
             \n@include wrapper() {\
             \n  @extend %ext;\
             \n}"),
        ".wrapped {\
         \n  background: #000;\
         \n}\n"
    );
}
