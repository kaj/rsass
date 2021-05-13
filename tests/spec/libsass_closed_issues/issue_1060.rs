//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1060.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  @if true {\
             \n    foo: true;\
             \n  } @elseif true {\
             \n    foo: false;\
             \n  } @else {\
             \n    foo: false;\
             \n  }\n\
             \n  @if true {\
             \n    bar: true;\
             \n  } @else if true {\
             \n    bar: false;\
             \n  } @else {\
             \n    bar: false;\
             \n  }\
             \n}\n"),
        "foo {\
         \n  foo: true;\
         \n  bar: true;\
         \n}\n"
    );
}
