//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1793.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "@media (max-width: (2px*5in)) {\
             \n  foo { bar: baz; }\
             \n}\
             \n"
        )
        .unwrap_err(),
        "Error: 10px*in isn\'t a valid CSS value.\
         \n  ,\
         \n1 | @media (max-width: (2px*5in)) {\
         \n  |                    ^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:20  root stylesheet",
    );
}
