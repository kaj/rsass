//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_238760.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_238760")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err("$id: inspect((a#b:c)...)"),
        "Error: Variable keyword argument map must have string keys.\
         \na #b is not a string in (a #b: c).\
         \n  ,\
         \n1 | $id: inspect((a#b:c)...)\
         \n  |              ^^^^^^^\
         \n  \'\
         \n  input.scss 1:14  root stylesheet",
    );
}
