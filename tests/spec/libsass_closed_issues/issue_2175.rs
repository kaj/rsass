//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2175.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "a:nth-child() {\
             \n  color: yellowgreen;\
             \n}\n"
        ),
        "Error: Expected \"n\".\
         \n  ,\
         \n1 | a:nth-child(){\
         \n  |             ^\
         \n  \'\
         \n  input.scss 1:13  root stylesheet",
    );
}
