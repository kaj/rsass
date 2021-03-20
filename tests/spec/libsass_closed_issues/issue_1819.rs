//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1819.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  bar: type-of(selector-unify(\'p\', \'a\'));\
            \n}"
        )
        .unwrap(),
        "foo {\
        \n  bar: null;\
        \n}\
        \n"
    );
}
