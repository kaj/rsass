//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1577.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1577")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "$foo: 10%; // line 1\
             \n// line 2\
             \n$error: $foo + 20px; // line 3\n"
        ),
        "Error: 10% and 20px have incompatible units.\
         \n  ,\
         \n3 | $error: $foo + 20px; // line 3\
         \n  |         ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:9  root stylesheet",
    );
}
