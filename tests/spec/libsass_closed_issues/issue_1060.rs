//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1060.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  @if true {\
            \n    foo: true;\
            \n  } @elseif true {\
            \n    foo: false;\
            \n  } @else {\
            \n    foo: false;\
            \n  }\
            \n\
            \n  @if true {\
            \n    bar: true;\
            \n  } @else if true {\
            \n    bar: false;\
            \n  } @else {\
            \n    bar: false;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  foo: true;\
        \n  bar: true;\
        \n}\
        \n"
    );
}
