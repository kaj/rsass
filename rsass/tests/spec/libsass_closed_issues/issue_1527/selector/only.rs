//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1527/selector/only.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("only")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "& {}\n"
        ),
        "Error: Top-level selectors may not contain the parent selector \"&\".\
         \n  ,\
         \n1 | & {}\
         \n  | ^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
    );
}
