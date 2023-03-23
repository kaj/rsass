//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/imported/basic-postfix.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("basic-postfix")
        .mock_file("include.scss", "&post {\n  foo {\n    bar: baz;\n  }\n}")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "@import \"include.scss\";"
        ),
        "Error: Top-level selectors may not contain the parent selector \"&\".\
         \n  ,\
         \n1 | &post {\
         \n  | ^^^^^\
         \n  \'\
         \n  include.scss 1:1  @import\
         \n  input.scss 1:9    root stylesheet",
    );
}
