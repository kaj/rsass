//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1681/element.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("element")
}

#[test]
fn test() {
    assert_eq!(
        runner().err(
            "@function element() {\
             \n  @return null;\
             \n}\n"
        ),
        "Error: Invalid function name.\
         \n  ,\
         \n1 | @function element() {\
         \n  | ^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
    );
}
