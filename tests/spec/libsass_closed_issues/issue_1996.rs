//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1996.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("$foo: \"bar\";\n\
             \n%class //#{$foo}\
             \n{\
             \n  &_baz {\
             \n    color: red;\
             \n  }\
             \n}"),
        ""
    );
}
