//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1432/selector-replace.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("selector-replace")
}

#[test]
fn test() {
    assert_eq!(
        runner().err(
            "@use \"sass:selector\";\
             \n@debug(selector.replace(\"foo\", \"bar\", null));\n"
        ),
        "Error: $replacement: null is not a valid selector: it must be a string,\
         \na list of strings, or a list of lists of strings.\
         \n  ,\
         \n2 | @debug(selector.replace(\"foo\", \"bar\", null));\
         \n  |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:8  root stylesheet",
    );
}
