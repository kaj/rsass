//! Tests auto-converted from "sass-spec/spec/libsass-todo-issues/issue_2295/error/basic.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("basic")
        .mock_file("include.scss", "display: none;\r\n")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            ".my-scope {\r\
             \n  @import \'include.scss\';\r\
             \n}"
        ),
        "DEPRECATION WARNING [import]: Sass @import rules are deprecated and will be removed in Dart Sass 3.0.0.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n2 |   @import \'include.scss\';\
         \n  |           ^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 2:11  root stylesheet\n\
         \nError: expected \"{\".\
         \n  ,\
         \n1 | display: none;\
         \n  |              ^\
         \n  \'\
         \n  include.scss 1:14  @import\
         \n  input.scss 2:11    root stylesheet",
    );
}
