//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_659/sass-script.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$foo: null;\n\
             \n@mixin bar() {\
             \n   bar: $foo;\
             \n}\n\
             \n@mixin baz() {\
             \n   baz: $foo !important;\
             \n}\n\
             \nfoo {\
             \n  baz: $foo;\
             \n}\n\
             \nbar {\
             \n  @include bar;\
             \n}\n\
             \nbaz {\
             \n  @include baz;\
             \n}\n"),
        "baz {\
         \n  baz: !important;\
         \n}\n"
    );
}
