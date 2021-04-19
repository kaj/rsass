//! Tests auto-converted from "sass-spec/spec/css/plain/error/statement.hrx"

mod at_rule {
    #[test]
    #[ignore] // missing error
    fn at_root() {
        assert_eq!(
            crate::rsass("@import \'plain\'").unwrap_err(),
            "Error: This at-rule isn\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   @at-root b {\
         \n  |   ^^^^^^^^^^^\
         \n  \'\
         \n  plain.css 2:3   @import\
         \n  input.scss 1:9  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn content() {
        assert_eq!(
            crate::rsass("@import \'plain\'").unwrap_err(),
            "Error: This at-rule isn\'t allowed in plain CSS.\
         \n  ,\
         \n1 | @content;\
         \n  | ^^^^^^^^\
         \n  \'\
         \n  plain.css 1:1   @import\
         \n  input.scss 1:9  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn debug() {
        assert_eq!(
            crate::rsass("@import \'plain\'").unwrap_err(),
            "Error: This at-rule isn\'t allowed in plain CSS.\
         \n  ,\
         \n1 | @debug foo;\
         \n  | ^^^^^^^^^^\
         \n  \'\
         \n  plain.css 1:1   @import\
         \n  input.scss 1:9  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn each() {
        assert_eq!(
            crate::rsass("@import \'plain\'").unwrap_err(),
            "Error: This at-rule isn\'t allowed in plain CSS.\
         \n  ,\
         \n1 | @each $i in 1 2 3 {\
         \n  | ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  plain.css 1:1   @import\
         \n  input.scss 1:9  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn error() {
        assert_eq!(
            crate::rsass("@import \'plain\'").unwrap_err(),
            "Error: This at-rule isn\'t allowed in plain CSS.\
         \n  ,\
         \n1 | @error foo;\
         \n  | ^^^^^^^^^^\
         \n  \'\
         \n  plain.css 1:1   @import\
         \n  input.scss 1:9  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn extend() {
        assert_eq!(
            crate::rsass("@import \'plain\'").unwrap_err(),
            "Error: This at-rule isn\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   @extend b;\
         \n  |   ^^^^^^^^^\
         \n  \'\
         \n  plain.css 2:3   @import\
         \n  input.scss 1:9  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn test_for() {
        assert_eq!(
            crate::rsass("@import \'plain\'").unwrap_err(),
            "Error: This at-rule isn\'t allowed in plain CSS.\
         \n  ,\
         \n1 | @for $i from 1 to 5 {\
         \n  | ^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  plain.css 1:1   @import\
         \n  input.scss 1:9  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn function() {
        assert_eq!(
            crate::rsass("@import \'plain\'").unwrap_err(),
            "Error: This at-rule isn\'t allowed in plain CSS.\
         \n  ,\
         \n1 | @function foo() {\
         \n  | ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  plain.css 1:1   @import\
         \n  input.scss 1:9  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn test_if() {
        assert_eq!(
            crate::rsass("@import \'plain\'").unwrap_err(),
            "Error: This at-rule isn\'t allowed in plain CSS.\
         \n  ,\
         \n1 | @if true {\
         \n  | ^^^^^^^^^\
         \n  \'\
         \n  plain.css 1:1   @import\
         \n  input.scss 1:9  root stylesheet",
        );
    }
    mod import {
        #[test]
        #[ignore] // missing error
        fn interpolated() {
            assert_eq!(
                crate::rsass("@import \'plain\'").unwrap_err(),
                "Error: Interpolation isn\'t allowed in plain CSS.\
         \n  ,\
         \n1 | @import url(\"foo#{bar}baz\");\
         \n  |                 ^^^^^^\
         \n  \'\
         \n  plain.css 1:17  @import\
         \n  input.scss 1:9  root stylesheet",
            );
        }
        #[test]
        #[ignore] // missing error
        fn multi() {
            assert_eq!(
                crate::rsass("@import \'plain\'").unwrap_err(),
                "Error: expected \";\".\
         \n  ,\
         \n1 | @import \"foo\", \"bar\";\
         \n  |              ^\
         \n  \'\
         \n  plain.css 1:14  @import\
         \n  input.scss 1:9  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong result
        fn nested() {
            assert_eq!(
                crate::rsass("@import \'plain\'").unwrap(),
                "a {\
        \n  @import \"foo\";\
        \n}\
        \n"
            );
        }
    }
    #[test]
    #[ignore] // missing error
    fn include() {
        assert_eq!(
            crate::rsass("@import \'plain\'").unwrap_err(),
            "Error: This at-rule isn\'t allowed in plain CSS.\
         \n  ,\
         \n1 | @include foo;\
         \n  | ^^^^^^^^^^^^\
         \n  \'\
         \n  plain.css 1:1   @import\
         \n  input.scss 1:9  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn interpolation() {
        assert_eq!(
            crate::rsass("@import \'plain\'").unwrap_err(),
            "Error: Interpolation isn\'t allowed in plain CSS.\
         \n  ,\
         \n1 | @foo a#{b}c;\
         \n  |       ^^^^\
         \n  \'\
         \n  plain.css 1:7   @import\
         \n  input.scss 1:9  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn mixin() {
        assert_eq!(
            crate::rsass("@import \'plain\'").unwrap_err(),
            "Error: This at-rule isn\'t allowed in plain CSS.\
         \n  ,\
         \n1 | @mixin foo {\
         \n  | ^^^^^^^^^^^\
         \n  \'\
         \n  plain.css 1:1   @import\
         \n  input.scss 1:9  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn test_return() {
        assert_eq!(
            crate::rsass("@import \'plain\'").unwrap_err(),
            "Error: This at-rule isn\'t allowed in plain CSS.\
         \n  ,\
         \n1 | @return foo;\
         \n  | ^^^^^^^^^^^\
         \n  \'\
         \n  plain.css 1:1   @import\
         \n  input.scss 1:9  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn warn() {
        assert_eq!(
            crate::rsass("@import \'plain\'").unwrap_err(),
            "Error: This at-rule isn\'t allowed in plain CSS.\
         \n  ,\
         \n1 | @warn foo;\
         \n  | ^^^^^^^^^\
         \n  \'\
         \n  plain.css 1:1   @import\
         \n  input.scss 1:9  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn test_while() {
        assert_eq!(
            crate::rsass("@import \'plain\'").unwrap_err(),
            "Error: This at-rule isn\'t allowed in plain CSS.\
         \n  ,\
         \n1 | @while false {\
         \n  | ^^^^^^^^^^^^^\
         \n  \'\
         \n  plain.css 1:1   @import\
         \n  input.scss 1:9  root stylesheet",
        );
    }
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
         \n1 | // silent\
         \n  | ^^^^^^^^^\
         \n  \'\
         \n  plain.css 1:1   @import\
         \n  input.scss 1:9  root stylesheet",
    );
}
mod style_rule {
    mod interpolation {
        #[test]
        #[ignore] // missing error
        fn custom_property() {
            assert_eq!(
                crate::rsass("@import \'plain\'").unwrap_err(),
                "Error: Interpolation isn\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   --b: #{c};\
         \n  |        ^^^^\
         \n  \'\
         \n  plain.css 2:8   @import\
         \n  input.scss 1:9  root stylesheet",
            );
        }
        #[test]
        #[ignore] // missing error
        fn declaration() {
            assert_eq!(
                crate::rsass("@import \'plain\'").unwrap_err(),
                "Error: Interpolation isn\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   w#{x}y: z;\
         \n  |    ^^^^\
         \n  \'\
         \n  plain.css 2:4   @import\
         \n  input.scss 1:9  root stylesheet",
            );
        }
        #[test]
        #[ignore] // missing error
        fn selector() {
            assert_eq!(
                crate::rsass("@import \'plain\'").unwrap_err(),
                "Error: Interpolation isn\'t allowed in plain CSS.\
         \n  ,\
         \n1 | a#{b}c {\
         \n  |  ^^^^\
         \n  \'\
         \n  plain.css 1:2   @import\
         \n  input.scss 1:9  root stylesheet",
            );
        }
    }
    #[test]
    #[ignore] // missing error
    fn nested() {
        assert_eq!(
            crate::rsass("@import \'plain\'").unwrap_err(),
            "Error: expected \":\".\
         \n  ,\
         \n2 |   b {\
         \n  |     ^\
         \n  \'\
         \n  plain.css 2:5   @import\
         \n  input.scss 1:9  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn nested_property() {
        assert_eq!(
            crate::rsass("@import \'plain\'").unwrap_err(),
            "Error: Nested declarations aren\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   x: {\
         \n  |      ^\
         \n  \'\
         \n  plain.css 2:6   @import\
         \n  input.scss 1:9  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn parent_selector() {
        assert_eq!(
            crate::rsass("@import \'plain\'").unwrap_err(),
            "Error: Parent selectors aren\'t allowed here.\
         \n  ,\
         \n1 | &.foo{\
         \n  | ^\
         \n  \'\
         \n  plain.css 1:1   @import\
         \n  input.scss 1:9  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn placeholder_selector() {
        assert_eq!(
            crate::rsass("@import \'plain\'").unwrap_err(),
            "Error: Placeholder selectors aren\'t allowed here.\
         \n  ,\
         \n1 | %foo{\
         \n  | ^^^^\
         \n  \'\
         \n  plain.css 1:1   @import\
         \n  input.scss 1:9  root stylesheet",
        );
    }
}
