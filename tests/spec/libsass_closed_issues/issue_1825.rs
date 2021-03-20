//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1825.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  &-- {\
            \n    &baz {\
            \n      color: red;\
            \n    } \
            \n  } \
            \n} "
        )
        .unwrap(),
        "foo--baz {\
        \n  color: red;\
        \n}\
        \n"
    );
}
