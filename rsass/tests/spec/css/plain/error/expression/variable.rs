//! Tests auto-converted from "sass-spec/spec/css/plain/error/expression/variable.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("variable")
        .mock_file("declaration/plain.css", "$var: value;\n")
        .mock_file("use/plain.css", "a {\n  x: $var;\n}\n")
}

#[test]
#[ignore] // wrong error
fn declaration() {
    let runner = runner().with_cwd("declaration");
    assert_eq!(
        runner.err("@import \'plain\'"),
        "Error: Sass variables aren\'t allowed in plain CSS.\
         \n  ,\
         \n1 | $var: value;\
         \n  | ^^^^\
         \n  \'\
         \n  plain.css 1:1   @import\
         \n  input.scss 1:9  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn test_use() {
    let runner = runner().with_cwd("use");
    assert_eq!(
        runner.err("@import \'plain\'"),
        "Error: Sass variables aren\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   x: $var;\
         \n  |      ^^^^\
         \n  \'\
         \n  plain.css 2:6   @import\
         \n  input.scss 1:9  root stylesheet",
    );
}
