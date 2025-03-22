//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2304.hrx"

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
        "DEPRECATION WARNING [import]: Sass @import rules are deprecated and will be removed in Dart Sass 3.0.0.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | @import \"module\";\
         \n  |         ^^^^^^^^\
         \n  \'\
         \n    input.scss 1:9  root stylesheet\n\
         \nError: Top-level selectors may not contain the parent selector \"&\".\
         \n  ,\
         \n1 | .foo, & {\
         \n  |       ^\
         \n  \'\
         \n  _module.scss 1:7  @import\
         \n  input.scss 1:9    root stylesheet",
    );
}
