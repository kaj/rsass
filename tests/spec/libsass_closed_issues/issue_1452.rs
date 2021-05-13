//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1452.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "foo {\
             \n  foo: foo(());\
             \n}\n"
        ),
        "Error: () isn\'t a valid CSS value.\
         \n  ,\
         \n2 |   foo: foo(());\
         \n  |            ^^\
         \n  \'\
         \n  input.scss 2:12  root stylesheet",
    );
}
