//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_510.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$before: map-remove((foo: 1, bar: 2, baz: 3, burp: 4), bar, baz);\
            \n$after: (foo: 1, burp: 4);\
            \n\
            \ndiv {\
            \n  foo: $before == $after;\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  foo: true;\
        \n}\
        \n"
    );
}
