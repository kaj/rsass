//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1804/inline.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("inline")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "foo {\
             \n  bar: #{(2px*5in)};\
             \n}\n"
        ),
        "Error: calc(10px * 1in) isn\'t a valid CSS value.\
         \n  ,\
         \n2 |   bar: #{(2px*5in)};\
         \n  |          ^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:10  root stylesheet",
    );
}
