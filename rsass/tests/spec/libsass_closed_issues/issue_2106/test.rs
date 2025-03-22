//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2106/test.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("test")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "@import \"../does-not-exist\";\n"
        ),
        "DEPRECATION WARNING [import]: Sass @import rules are deprecated and will be removed in Dart Sass 3.0.0.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | @import \"../does-not-exist\";\
         \n  |         ^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:9  root stylesheet\n\
         \nError: Can\'t find stylesheet to import.\
         \n  ,\
         \n1 | @import \"../does-not-exist\";\
         \n  |         ^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:9  root stylesheet",
    );
}
