//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1527/selector/last.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "foo & {\
             \n  bar: baz;\
             \n}\n"
        ),
        "Error: Top-level selectors may not contain the parent selector \"&\".\
         \n  ,\
         \n1 | foo & {\
         \n  | ^^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
    );
}
