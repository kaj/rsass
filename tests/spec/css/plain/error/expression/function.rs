//! Tests auto-converted from "sass-spec/spec/css/plain/error/expression/function.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .mock_file("built_in/plain.css", "a {\n  x: index(1 2 3, 1);\n}\n")
        .mock_file(
            "keyword_arguments/plain.css",
            "a {\n  x: hsl(0, 100%, $lightness: 50%);\n}\n",
        )
        .mock_file(
            "variable_arguments/plain.css",
            "a {\n  x: hsl(0, 100%, 50%...);\n}\n",
        )
}

#[test]
#[ignore] // missing error
fn built_in() {
    let runner = runner().with_cwd("built_in");
    assert_eq!(
        runner.err("@import \'plain\'"),
        "Error: This function isn\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   x: index(1 2 3, 1);\
         \n  |      ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  plain.css 2:6   @import\
         \n  input.scss 1:9  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn keyword_arguments() {
    let runner = runner().with_cwd("keyword_arguments");
    assert_eq!(
        runner.err("@import \'plain\'"),
        "Error: Sass variables aren\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   x: hsl(0, 100%, $lightness: 50%);\
         \n  |                   ^^^^^^^^^^\
         \n  \'\
         \n  plain.css 2:19  @import\
         \n  input.scss 1:9  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn variable_arguments() {
    let runner = runner().with_cwd("variable_arguments");
    assert_eq!(
        runner.err("@import \'plain\'"),
        "Error: expected \")\".\
         \n  ,\
         \n2 |   x: hsl(0, 100%, 50%...);\
         \n  |                      ^\
         \n  \'\
         \n  plain.css 2:22  @import\
         \n  input.scss 1:9  root stylesheet",
    );
}
