//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1991.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$tests: (\
            \n  0: \'foo\',\
            \n  false: \'bar\',\
            \n);"
        )
        .unwrap(),
        ""
    );
}
