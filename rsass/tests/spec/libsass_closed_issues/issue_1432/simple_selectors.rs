//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1432/simple-selectors.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("simple-selectors")
}

#[test]
fn test() {
    assert_eq!(
        runner().err(
            "@debug(simple-selectors(null));\n"
        ),
        "Error: $selector: null is not a valid selector: it must be a string,\
         \na list of strings, or a list of lists of strings.\
         \n  ,\
         \n1 | @debug(simple-selectors(null));\
         \n  |        ^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:8  root stylesheet",
    );
}
