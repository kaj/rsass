//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/imported/basic-alone.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .mock_file("include.scss", "& {\n  foo {\n    bar: baz;\n  }\n}\n")
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
         \n1 | & {\
         \n  | ^^\
         \n  \'\
         \n  include.scss 1:1  @import\
         \n  input.scss 1:9    root stylesheet",
    );
}
