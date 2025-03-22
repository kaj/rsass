//! Tests auto-converted from "sass-spec/spec/directives/use/error/syntax/within.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("within")
}

#[test]
fn function() {
    assert_eq!(
        runner().err(
            "@function foo {\
             \n  @use \"other\";\
             \n}\n"
        ),
        "Error: expected \"(\".\
         \n  ,\
         \n1 | @function foo {\
         \n  |               ^\
         \n  \'\
         \n  input.scss 1:15  root stylesheet",
    );
}
#[test]
fn mixin() {
    assert_eq!(
        runner().err(
            "@mixin foo {\
             \n  @use \"other\";\
             \n}\n"
        ),
        "Error: This at-rule is not allowed here.\
         \n  ,\
         \n2 |   @use \"other\";\
         \n  |   ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:3  root stylesheet",
    );
}
#[test]
fn style_rule() {
    assert_eq!(
        runner().err(
            "a {\
             \n  @use \"other\";\
             \n}\n"
        ),
        "Error: This at-rule is not allowed here.\
         \n  ,\
         \n2 |   @use \"other\";\
         \n  |   ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:3  root stylesheet",
    );
}
