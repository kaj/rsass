//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_309.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$zzz: zzz;\r\
            \na[data-foo=\"#{$zzz}\"] { a: b; }"
        )
        .unwrap(),
        "a[data-foo=zzz] {\
        \n  a: b;\
        \n}\
        \n"
    );
}
