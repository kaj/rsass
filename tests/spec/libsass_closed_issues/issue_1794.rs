//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1794.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("@media (max-width /*comment*/ : 500px) {\
             \n  foo { bar: baz; }\
             \n}"),
        "@media (max-width: 500px) {\
         \n  foo {\
         \n    bar: baz;\
         \n  }\
         \n}\n"
    );
}
