//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2625.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "something\\:{ padding: 2px; }\
            \n"
        )
        .unwrap(),
        "something\\: {\
        \n  padding: 2px;\
        \n}\
        \n"
    );
}
