//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1153.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "/* precision: 0 */\
            \n$foo: 123px;\
            \nfoo {\
            \n  bar: $foo;\
            \n}"
        )
        .unwrap(),
        "/* precision: 0 */\
        \nfoo {\
        \n  bar: 123px;\
        \n}\
        \n"
    );
}
