//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1396.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  foo: foo\"bar\"#{baz};\
            \n  foo: foo\"bar\"baz;\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  foo: foo \"bar\" baz;\
        \n  foo: foo \"bar\" baz;\
        \n}\
        \n"
    );
}
