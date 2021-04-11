//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1583.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$ls: ((foo,));\
            \n\
            \nfoo {\
            \n  baz: length($ls);\
            \n  baz: type-of($ls);\
            \n  baz: inspect($ls);\
            \n}\
            \n\
            \nbar {\
            \n  baz: length(&);\
            \n  baz: type-of(&);\
            \n  baz: inspect(&);\
            \n}\
            \n\
            \nfoo {\
            \n  string: inspect(&);\
            \n  str-length: str-length(inspect(&));\
            \n  list-length: length(&);\
            \n}\
            \n\
            \nfoo, bar {\
            \n  string: inspect(&);\
            \n  str-length: str-length(inspect(&));\
            \n  list-length: length(&);\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  baz: 1;\
        \n  baz: list;\
        \n  baz: (foo,);\
        \n}\
        \nbar {\
        \n  baz: 1;\
        \n  baz: list;\
        \n  baz: (bar,);\
        \n}\
        \nfoo {\
        \n  string: (foo,);\
        \n  str-length: 6;\
        \n  list-length: 1;\
        \n}\
        \nfoo, bar {\
        \n  string: foo, bar;\
        \n  str-length: 8;\
        \n  list-length: 2;\
        \n}\
        \n"
    );
}
