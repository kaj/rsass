//! Tests auto-converted from "sass-spec/spec/css/plain/error/expression.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .mock_file(
            "function/built_in/plain.css",
            "a {\n  x: index(1 2 3, 1);\n}\n",
        )
        .mock_file(
            "function/keyword_arguments/plain.css",
            "a {\n  x: hsl(0, 100%, $lightness: 50%);\n}\n",
        )
        .mock_file(
            "function/variable_arguments/plain.css",
            "a {\n  x: hsl(0, 100%, 50%...);\n}\n",
        )
        .mock_file(
            "interpolation/calc/plain.css",
            "a {\n  w: calc(#{1px} + 10%);\n}\n",
        )
        .mock_file(
            "interpolation/identifier/plain.css",
            "a {\n  w: x#{y}z;\n}\n",
        )
        .mock_file(
            "interpolation/quoted_string/plain.css",
            "a {\n  w: \"x#{y}z\";\n}\n",
        )
        .mock_file(
            "interpolation/standalone/plain.css",
            "a {\n  w: #{x};\n}\n",
        )
        .mock_file("list/empty/plain.css", "a {\n  x: ();\n}\n")
        .mock_file("list/empty_comma/plain.css", "a {\n  x: (,);\n}\n")
        .mock_file("map/plain.css", "a {\n  x: (y: z);\n}\n")
        .mock_file("operation/addition/plain.css", "a {\n  x: y + z;\n}\n")
        .mock_file("operation/equals/plain.css", "a {\n  x: y == z;\n}\n")
        .mock_file(
            "operation/greater_than/plain.css",
            "a {\n  x: y > z;\n}\n",
        )
        .mock_file(
            "operation/greater_than_or_equal/plain.css",
            "a {\n  x: y >= z;\n}\n",
        )
        .mock_file("operation/less_than/plain.css", "a {\n  x: y < z;\n}\n")
        .mock_file(
            "operation/less_than_or_equal/plain.css",
            "a {\n  x: y <= z;\n}\n",
        )
        .mock_file("operation/modulo/plain.css", "a {\n  x: y % z;\n}\n")
        .mock_file(
            "operation/multiplication/plain.css",
            "a {\n  x: y * z;\n}\n",
        )
        .mock_file("operation/not_equals/plain.css", "a {\n  x: y != z;\n}\n")
        .mock_file("operation/subtraction/plain.css", "a {\n  x: y - z;\n}\n")
        .mock_file("parent_selector/plain.css", "a {\n  x: &;\n}\n")
        .mock_file("parentheses/plain.css", "a {\n  x: (y);\n}\n")
        .mock_file(
            "silent_comment/plain.css",
            "a {\n  b: c // d\n     e;\n}\n",
        )
        .mock_file("variable/declaration/plain.css", "$var: value;\n")
        .mock_file("variable/use/plain.css", "a {\n  x: $var;\n}\n")
}

mod function {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("function")
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
}
mod interpolation {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("interpolation")
    }

    #[test]
    #[ignore] // missing error
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
    #[ignore] // missing error
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
    #[ignore] // missing error
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
}
mod list {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("list")
    }

    #[test]
    #[ignore] // missing error
    fn empty() {
        let runner = runner().with_cwd("empty");
        assert_eq!(
            runner.err("@import \'plain\'"),
            "Error: Parentheses aren\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   x: ();\
         \n  |      ^\
         \n  \'\
         \n  plain.css 2:6   @import\
         \n  input.scss 1:9  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn empty_comma() {
        let runner = runner().with_cwd("empty_comma");
        assert_eq!(
            runner.err("@import \'plain\'"),
            "Error: Parentheses aren\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   x: (,);\
         \n  |      ^\
         \n  \'\
         \n  plain.css 2:6   @import\
         \n  input.scss 1:9  root stylesheet",
        );
    }
}
#[test]
#[ignore] // missing error
fn map() {
    let runner = runner().with_cwd("map");
    assert_eq!(
        runner.err("@import \'plain\'"),
        "Error: Parentheses aren\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   x: (y: z);\
         \n  |      ^\
         \n  \'\
         \n  plain.css 2:6   @import\
         \n  input.scss 1:9  root stylesheet",
    );
}
mod operation {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("operation")
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
}
#[test]
#[ignore] // missing error
fn parent_selector() {
    let runner = runner().with_cwd("parent_selector");
    assert_eq!(
        runner.err("@import \'plain\'"),
        "Error: The parent selector isn\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   x: &;\
         \n  |      ^\
         \n  \'\
         \n  plain.css 2:6   @import\
         \n  input.scss 1:9  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn parentheses() {
    let runner = runner().with_cwd("parentheses");
    assert_eq!(
        runner.err("@import \'plain\'"),
        "Error: Parentheses aren\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   x: (y);\
         \n  |      ^\
         \n  \'\
         \n  plain.css 2:6   @import\
         \n  input.scss 1:9  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn silent_comment() {
    let runner = runner().with_cwd("silent_comment");
    assert_eq!(
        runner.err("@import \'plain\'\n"),
        "Error: Silent comments aren\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   b: c // d\
         \n  |        ^^^^\
         \n  \'\
         \n  plain.css 2:8   @import\
         \n  input.scss 1:9  root stylesheet",
    );
}
mod variable {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("variable")
    }

    #[test]
    #[ignore] // missing error
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
    #[ignore] // missing error
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
}
