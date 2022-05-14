//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2304.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("issue_2304")
        .mock_file("_module.scss", ".foo, & {\n  background: red;\n}\n\n.foo, &:before {\n  background: red;\n}")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "@import \"module\";"
        ),
        "Error: Top-level selectors may not contain the parent selector \"&\".\
         \n  ,\
         \n1 | .foo, & {\
         \n  | ^^^^^^^^\
         \n  \'\
         \n  _module.scss 1:1  @import\
         \n  input.scss 1:9    root stylesheet",
    );
}
