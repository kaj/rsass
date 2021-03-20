//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1333.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@function baz() {\
            \n    @return \'baz\';\
            \n}\
            \n\
            \nfoo {\
            \n    bar: baz()#{\' !important\'};\
            \n    bar: baz() #{\' !important\'};\
            \n}\
            \n\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  bar: \"baz\"  !important;\
        \n  bar: \"baz\"  !important;\
        \n}\
        \n"
    );
}
