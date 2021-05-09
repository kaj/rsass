//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_659/static.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("\
             \n%bam { bam: null; }\n\
             \n@mixin bar() {\
             \n   bar: null;\
             \n}\n\
             \n@mixin baz() {\
             \n   baz: null !important;\
             \n}\n\
             \nfoo {\
             \n  foo: null;\
             \n}\n\
             \nbar {\
             \n  @include bar;\
             \n}\n\
             \nbaz {\
             \n  @include baz;\
             \n}\n\
             \nbam {\
             \n  @extend %bam;\
             \n}\n"),
        "baz {\
         \n  baz: !important;\
         \n}\n"
    );
}
