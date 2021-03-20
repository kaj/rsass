//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1331.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$m: (foo: 1px, null: 2px, false: 3px, true: 4px);\
            \n\
            \n@debug $m;\
            \n@debug map-get($m, foo);\
            \n@debug map-get($m, null);\
            \n@debug map-get($m, false);\
            \n@debug map-get($m, true);\
            \n"
        )
        .unwrap(),
        ""
    );
}
