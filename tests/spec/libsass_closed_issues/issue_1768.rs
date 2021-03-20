//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1768.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@debug(());\
            \n@debug(foo, (), bar);\
            \n@debug(foo () bar);\
            \n@debug((foo: (), bar: baz));"
        )
        .unwrap(),
        ""
    );
}
