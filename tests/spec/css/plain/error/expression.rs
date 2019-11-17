//! Tests auto-converted from "sass-spec/spec/css/plain/error/expression.hrx"

mod function {
    #[test]
    #[ignore] // missing error
    fn built_in() {
        assert_eq!(
            crate::rsass("@import \'plain\'").unwrap_err(),
            "Error: This function isn\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   x: index(1 2 3, 1);\
         \n  |      ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  plain.css 2:6   @import\
         \n  input.scss 1:9  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // missing error
    fn keyword_arguments() {
        assert_eq!(
            crate::rsass("@import \'plain\'").unwrap_err(),
            "Error: Sass variables aren\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   x: hsl(0, 100%, $lightness: 50%);\
         \n  |                   ^^^^^^^^^^\
         \n  \'\
         \n  plain.css 2:19  @import\
         \n  input.scss 1:9  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // missing error
    fn variable_arguments() {
        assert_eq!(
            crate::rsass("@import \'plain\'").unwrap_err(),
            "Error: expected \")\".\
         \n  ,\
         \n2 |   x: hsl(0, 100%, 50%...);\
         \n  |                      ^\
         \n  \'\
         \n  plain.css 2:22  @import\
         \n  input.scss 1:9  root stylesheet\
         \n",
        );
    }
}
mod interpolation {
    #[test]
    #[ignore] // missing error
    fn calc() {
        assert_eq!(
            crate::rsass("@import \'plain\'").unwrap_err(),
            "Error: Interpolation isn\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   w: calc(#{1px} + 10%);\
         \n  |           ^^^^^^\
         \n  \'\
         \n  plain.css 2:11  @import\
         \n  input.scss 1:9  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // missing error
    fn identifier() {
        assert_eq!(
            crate::rsass("@import \'plain\'").unwrap_err(),
            "Error: Interpolation isn\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   w: x#{y}z;\
         \n  |       ^^^^\
         \n  \'\
         \n  plain.css 2:7   @import\
         \n  input.scss 1:9  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // missing error
    fn quoted_string() {
        assert_eq!(
            crate::rsass("@import \'plain\'").unwrap_err(),
            "Error: Interpolation isn\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   w: \"x#{y}z\";\
         \n  |        ^^^^\
         \n  \'\
         \n  plain.css 2:8   @import\
         \n  input.scss 1:9  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // missing error
    fn standalone() {
        assert_eq!(
            crate::rsass("@import \'plain\'").unwrap_err(),
            "Error: Interpolation isn\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   w: #{x};\
         \n  |      ^^^^\
         \n  \'\
         \n  plain.css 2:6   @import\
         \n  input.scss 1:9  root stylesheet\
         \n",
        );
    }
}
mod list {
    #[test]
    #[ignore] // missing error
    fn empty() {
        assert_eq!(
            crate::rsass("@import \'plain\'").unwrap_err(),
            "Error: Parentheses aren\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   x: ();\
         \n  |      ^\
         \n  \'\
         \n  plain.css 2:6   @import\
         \n  input.scss 1:9  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // missing error
    fn empty_comma() {
        assert_eq!(
            crate::rsass("@import \'plain\'").unwrap_err(),
            "Error: Parentheses aren\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   x: (,);\
         \n  |      ^\
         \n  \'\
         \n  plain.css 2:6   @import\
         \n  input.scss 1:9  root stylesheet\
         \n",
        );
    }
}
#[test]
#[ignore] // missing error
fn map() {
    assert_eq!(
        crate::rsass("@import \'plain\'").unwrap_err(),
        "Error: Parentheses aren\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   x: (y: z);\
         \n  |      ^\
         \n  \'\
         \n  plain.css 2:6   @import\
         \n  input.scss 1:9  root stylesheet\
         \n",
    );
}
mod operation {
    #[test]
    #[ignore] // missing error
    fn addition() {
        assert_eq!(
            crate::rsass("@import \'plain\'").unwrap_err(),
            "Error: Operators aren\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   x: y + z;\
         \n  |        ^\
         \n  \'\
         \n  plain.css 2:8   @import\
         \n  input.scss 1:9  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // missing error
    fn equals() {
        assert_eq!(
            crate::rsass("@import \'plain\'").unwrap_err(),
            "Error: Operators aren\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   x: y == z;\
         \n  |        ^^\
         \n  \'\
         \n  plain.css 2:8   @import\
         \n  input.scss 1:9  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // missing error
    fn greater_than() {
        assert_eq!(
            crate::rsass("@import \'plain\'").unwrap_err(),
            "Error: Operators aren\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   x: y > z;\
         \n  |        ^\
         \n  \'\
         \n  plain.css 2:8   @import\
         \n  input.scss 1:9  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // missing error
    fn greater_than_or_equal() {
        assert_eq!(
            crate::rsass("@import \'plain\'").unwrap_err(),
            "Error: Operators aren\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   x: y >= z;\
         \n  |        ^^\
         \n  \'\
         \n  plain.css 2:8   @import\
         \n  input.scss 1:9  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // missing error
    fn less_than() {
        assert_eq!(
            crate::rsass("@import \'plain\'").unwrap_err(),
            "Error: Operators aren\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   x: y < z;\
         \n  |        ^\
         \n  \'\
         \n  plain.css 2:8   @import\
         \n  input.scss 1:9  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // missing error
    fn less_than_or_equal() {
        assert_eq!(
            crate::rsass("@import \'plain\'").unwrap_err(),
            "Error: Operators aren\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   x: y <= z;\
         \n  |        ^^\
         \n  \'\
         \n  plain.css 2:8   @import\
         \n  input.scss 1:9  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // missing error
    fn modulo() {
        assert_eq!(
            crate::rsass("@import \'plain\'").unwrap_err(),
            "Error: Operators aren\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   x: y % z;\
         \n  |        ^\
         \n  \'\
         \n  plain.css 2:8   @import\
         \n  input.scss 1:9  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // missing error
    fn multiplication() {
        assert_eq!(
            crate::rsass("@import \'plain\'").unwrap_err(),
            "Error: Operators aren\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   x: y * z;\
         \n  |        ^\
         \n  \'\
         \n  plain.css 2:8   @import\
         \n  input.scss 1:9  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // missing error
    fn not_equals() {
        assert_eq!(
            crate::rsass("@import \'plain\'").unwrap_err(),
            "Error: Operators aren\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   x: y != z;\
         \n  |        ^^\
         \n  \'\
         \n  plain.css 2:8   @import\
         \n  input.scss 1:9  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // missing error
    fn subtraction() {
        assert_eq!(
            crate::rsass("@import \'plain\'").unwrap_err(),
            "Error: Operators aren\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   x: y - z;\
         \n  |        ^\
         \n  \'\
         \n  plain.css 2:8   @import\
         \n  input.scss 1:9  root stylesheet\
         \n",
        );
    }
}
#[test]
#[ignore] // missing error
fn parent_selector() {
    assert_eq!(
        crate::rsass("@import \'plain\'").unwrap_err(),
        "Error: The parent selector isn\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   x: &;\
         \n  |      ^\
         \n  \'\
         \n  plain.css 2:6   @import\
         \n  input.scss 1:9  root stylesheet\
         \n",
    );
}
#[test]
#[ignore] // missing error
fn parentheses() {
    assert_eq!(
        crate::rsass("@import \'plain\'").unwrap_err(),
        "Error: Parentheses aren\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   x: (y);\
         \n  |      ^\
         \n  \'\
         \n  plain.css 2:6   @import\
         \n  input.scss 1:9  root stylesheet\
         \n",
    );
}
#[test]
#[ignore] // missing error
fn silent_comment() {
    assert_eq!(
        crate::rsass(
            "@import \'plain\'\
             \n"
        )
        .unwrap_err(),
        "Error: Silent comments aren\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   b: c // d\
         \n  |        ^^^^\
         \n  \'\
         \n  plain.css 2:8   @import\
         \n  input.scss 1:9  root stylesheet\
         \n",
    );
}
mod variable {
    #[test]
    #[ignore] // missing error
    fn declaration() {
        assert_eq!(
            crate::rsass("@import \'plain\'").unwrap_err(),
            "Error: Sass variables aren\'t allowed in plain CSS.\
         \n  ,\
         \n1 | $var: value;\
         \n  | ^^^^\
         \n  \'\
         \n  plain.css 1:1   @import\
         \n  input.scss 1:9  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // missing error
    fn test_use() {
        assert_eq!(
            crate::rsass("@import \'plain\'").unwrap_err(),
            "Error: Sass variables aren\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   x: $var;\
         \n  |      ^^^^\
         \n  \'\
         \n  plain.css 2:6   @import\
         \n  input.scss 1:9  root stylesheet\
         \n",
        );
    }
}
