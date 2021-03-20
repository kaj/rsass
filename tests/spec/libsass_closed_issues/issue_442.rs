//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_442.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$lhs: (100/10)#{rem};\
            \n$rhs: 10rem;\
            \n\
            \nfoo {\
            \n  a: $lhs;\
            \n  a: $rhs;\
            \n  a: $lhs == $rhs;\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: 10 rem;\
        \n  a: 10rem;\
        \n  a: false;\
        \n}\
        \n"
    );
}
