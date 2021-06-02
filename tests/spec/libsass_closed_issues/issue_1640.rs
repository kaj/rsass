//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1640.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@mixin foo() {\
             \n    @if false {\
             \n      a { b: c }\
             \n    } @else {\
             \n      @content;\
             \n    }\
             \n}\n\
             \n@include foo() {\
             \n  .foo {\
             \n    bar: baz;\
             \n  }\
             \n}\n"),
        ".foo {\
         \n  bar: baz;\
         \n}\n"
    );
}
