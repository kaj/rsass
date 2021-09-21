//! Tests auto-converted from "sass-spec/spec/css/plain/error/expression/operation.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .mock_file("addition/plain.css", "a {\n  x: y + z;\n}\n")
        .mock_file("equals/plain.css", "a {\n  x: y == z;\n}\n")
        .mock_file("greater_than/plain.css", "a {\n  x: y > z;\n}\n")
        .mock_file(
            "greater_than_or_equal/plain.css",
            "a {\n  x: y >= z;\n}\n",
        )
        .mock_file("less_than/plain.css", "a {\n  x: y < z;\n}\n")
        .mock_file("less_than_or_equal/plain.css", "a {\n  x: y <= z;\n}\n")
        .mock_file("modulo/plain.css", "a {\n  x: y % z;\n}\n")
        .mock_file("multiplication/plain.css", "a {\n  x: y * z;\n}\n")
        .mock_file("not_equals/plain.css", "a {\n  x: y != z;\n}\n")
        .mock_file("subtraction/plain.css", "a {\n  x: y - z;\n}\n")
}

#[test]
#[ignore] // missing error
fn addition() {
    let runner = runner().with_cwd("addition");
    assert_eq!(
        runner.err("@import \'plain\'"),
        "Error: Operators aren\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   x: y + z;\
         \n  |        ^\
         \n  \'\
         \n  plain.css 2:8   @import\
         \n  input.scss 1:9  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn equals() {
    let runner = runner().with_cwd("equals");
    assert_eq!(
        runner.err("@import \'plain\'"),
        "Error: Operators aren\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   x: y == z;\
         \n  |        ^^\
         \n  \'\
         \n  plain.css 2:8   @import\
         \n  input.scss 1:9  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn greater_than() {
    let runner = runner().with_cwd("greater_than");
    assert_eq!(
        runner.err("@import \'plain\'"),
        "Error: Operators aren\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   x: y > z;\
         \n  |        ^\
         \n  \'\
         \n  plain.css 2:8   @import\
         \n  input.scss 1:9  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn greater_than_or_equal() {
    let runner = runner().with_cwd("greater_than_or_equal");
    assert_eq!(
        runner.err("@import \'plain\'"),
        "Error: Operators aren\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   x: y >= z;\
         \n  |        ^^\
         \n  \'\
         \n  plain.css 2:8   @import\
         \n  input.scss 1:9  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn less_than() {
    let runner = runner().with_cwd("less_than");
    assert_eq!(
        runner.err("@import \'plain\'"),
        "Error: Operators aren\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   x: y < z;\
         \n  |        ^\
         \n  \'\
         \n  plain.css 2:8   @import\
         \n  input.scss 1:9  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn less_than_or_equal() {
    let runner = runner().with_cwd("less_than_or_equal");
    assert_eq!(
        runner.err("@import \'plain\'"),
        "Error: Operators aren\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   x: y <= z;\
         \n  |        ^^\
         \n  \'\
         \n  plain.css 2:8   @import\
         \n  input.scss 1:9  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn modulo() {
    let runner = runner().with_cwd("modulo");
    assert_eq!(
        runner.err("@import \'plain\'"),
        "Error: Operators aren\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   x: y % z;\
         \n  |        ^\
         \n  \'\
         \n  plain.css 2:8   @import\
         \n  input.scss 1:9  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn multiplication() {
    let runner = runner().with_cwd("multiplication");
    assert_eq!(
        runner.err("@import \'plain\'"),
        "Error: Operators aren\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   x: y * z;\
         \n  |        ^\
         \n  \'\
         \n  plain.css 2:8   @import\
         \n  input.scss 1:9  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn not_equals() {
    let runner = runner().with_cwd("not_equals");
    assert_eq!(
        runner.err("@import \'plain\'"),
        "Error: Operators aren\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   x: y != z;\
         \n  |        ^^\
         \n  \'\
         \n  plain.css 2:8   @import\
         \n  input.scss 1:9  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn subtraction() {
    let runner = runner().with_cwd("subtraction");
    assert_eq!(
        runner.err("@import \'plain\'"),
        "Error: Operators aren\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   x: y - z;\
         \n  |        ^\
         \n  \'\
         \n  plain.css 2:8   @import\
         \n  input.scss 1:9  root stylesheet",
    );
}
