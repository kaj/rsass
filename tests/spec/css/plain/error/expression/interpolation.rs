//! Tests auto-converted from "sass-spec/spec/css/plain/error/expression/interpolation.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("interpolation")
        .mock_file("calc/plain.css", "a {\n  w: calc(#{1px} + 10%);\n}\n")
        .mock_file("identifier/plain.css", "a {\n  w: x#{y}z;\n}\n")
        .mock_file("quoted_string/plain.css", "a {\n  w: \"x#{y}z\";\n}\n")
        .mock_file("standalone/plain.css", "a {\n  w: #{x};\n}\n")
}

#[test]
#[ignore] // wrong error
fn calc() {
    let runner = runner().with_cwd("calc");
    assert_eq!(
        runner.err("@import \'plain\'"),
        "Error: Interpolation isn\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   w: calc(#{1px} + 10%);\
         \n  |           ^^^^^^\
         \n  \'\
         \n  plain.css 2:11  @import\
         \n  input.scss 1:9  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn identifier() {
    let runner = runner().with_cwd("identifier");
    assert_eq!(
        runner.err("@import \'plain\'"),
        "Error: Interpolation isn\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   w: x#{y}z;\
         \n  |       ^^^^\
         \n  \'\
         \n  plain.css 2:7   @import\
         \n  input.scss 1:9  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn quoted_string() {
    let runner = runner().with_cwd("quoted_string");
    assert_eq!(
        runner.err("@import \'plain\'"),
        "Error: Interpolation isn\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   w: \"x#{y}z\";\
         \n  |        ^^^^\
         \n  \'\
         \n  plain.css 2:8   @import\
         \n  input.scss 1:9  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn standalone() {
    let runner = runner().with_cwd("standalone");
    assert_eq!(
        runner.err("@import \'plain\'"),
        "Error: Interpolation isn\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   w: #{x};\
         \n  |      ^^^^\
         \n  \'\
         \n  plain.css 2:6   @import\
         \n  input.scss 1:9  root stylesheet",
    );
}
