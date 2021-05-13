//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2081.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err("$foo: #{bar();};\n"),
        "Error: expected \"}\".\
         \n  ,\
         \n1 | $foo: #{bar();};\
         \n  |              ^\
         \n  \'\
         \n  input.scss 1:14  root stylesheet",
    );
}
