//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_495.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "/* Testing to make sure that a trailing comma doesn\'t break the tests */\
            \n$map: (\
            \n  hello: world,\
            \n);\
            \n"
        )
        .unwrap(),
        "/* Testing to make sure that a trailing comma doesn\'t break the tests */\
        \n"
    );
}
