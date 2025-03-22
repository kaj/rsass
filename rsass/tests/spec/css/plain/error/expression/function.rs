//! Tests auto-converted from "sass-spec/spec/css/plain/error/expression/function.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("function")
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
        runner.err("@use \'plain\'"),
        "Error: This function isn\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   x: index(1 2 3, 1);\
         \n  |      ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  plain.css 2:6   @use\
         \n  input.scss 1:1  root stylesheet",
    );
}
#[test]
fn keyword_arguments() {
    let runner = runner().with_cwd("keyword_arguments");
    assert_eq!(
        runner.err("@use \'plain\'"),
        "Error: Sass variables aren\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   x: hsl(0, 100%, $lightness: 50%);\
         \n  |                   ^^^^^^^^^^\
         \n  \'\
         \n  plain.css 2:19  @use\
         \n  input.scss 1:1  root stylesheet",
    );
}
#[test]
fn variable_arguments() {
    let runner = runner().with_cwd("variable_arguments");
    assert_eq!(
        runner.err("@use \'plain\'"),
        "Error: expected \")\".\
         \n  ,\
         \n2 |   x: hsl(0, 100%, 50%...);\
         \n  |                      ^\
         \n  \'\
         \n  plain.css 2:22  @use\
         \n  input.scss 1:1  root stylesheet",
    );
}
