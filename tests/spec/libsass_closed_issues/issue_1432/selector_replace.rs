//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1432/selector-replace.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "@debug(selector-replace(\"foo\", \"bar\", null));\n"
        ),
        "Error: $replacement: null is not a valid selector: it must be a string,\
         \na list of strings, or a list of lists of strings.\
         \n  ,\
         \n1 | @debug(selector-replace(\"foo\", \"bar\", null));\
         \n  |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:8  root stylesheet",
    );
}
