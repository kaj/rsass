//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_309.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$zzz: zzz;\r\
             \na[data-foo=\"#{$zzz}\"] { a: b; }"),
        "a[data-foo=zzz] {\
         \n  a: b;\
         \n}\n"
    );
}
