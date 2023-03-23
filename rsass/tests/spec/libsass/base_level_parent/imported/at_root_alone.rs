//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/imported/at-root-alone.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("at-root-alone").mock_file(
        "include.scss",
        "@at-root {\n  & {\n    foo {\n      bar: baz;\n    }\n  }\n}",
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
         \n2 |   & {\
         \n  |   ^\
         \n  \'\
         \n  include.scss 2:3  @import\
         \n  input.scss 1:9    root stylesheet",
    );
}
