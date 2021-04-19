//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1577.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "$foo: 10%; // line 1\
             \n// line 2\
             \n$error: $foo + 20px; // line 3\
             \n"
        )
        .unwrap_err(),
        "Error: 10% and 20px have incompatible units.\
         \n  ,\
         \n3 | $error: $foo + 20px; // line 3\
         \n  |         ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:9  root stylesheet\
         \n",
    );
}
