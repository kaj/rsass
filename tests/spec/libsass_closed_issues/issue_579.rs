//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_579.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$map: (\
            \n  foo: fump,\
            \n  bar: bump,\
            \n);\
            \n\
            \n@mixin vararg-test($foo, $bar) {\
            \n  foo: $foo;\
            \n  bar: $bar;\
            \n}\
            \n\
            \n.test {\
            \n  @include vararg-test($map...);\
            \n}\
            \n"
        )
        .unwrap(),
        ".test {\
        \n  foo: fump;\
        \n  bar: bump;\
        \n}\
        \n"
    );
}
