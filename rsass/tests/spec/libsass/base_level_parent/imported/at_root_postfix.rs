//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/imported/at-root-postfix.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("at-root-postfix").mock_file(
        "include.scss",
        "@at-root {\n  &post {\n    foo {\n      bar: baz;\n    }\n  }\n}",
    )
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
         \n2 |   &post {\
         \n  |   ^^^^^\
         \n  \'\
         \n  include.scss 2:3  @import\
         \n  input.scss 1:9    root stylesheet",
    );
}
