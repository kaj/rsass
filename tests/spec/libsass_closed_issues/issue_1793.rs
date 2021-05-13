//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1793.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "@media (max-width: (2px*5in)) {\
             \n  foo { bar: baz; }\
             \n}\n"
        ),
        "Error: 10px*in isn\'t a valid CSS value.\
         \n  ,\
         \n1 | @media (max-width: (2px*5in)) {\
         \n  |                    ^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:20  root stylesheet",
    );
}
