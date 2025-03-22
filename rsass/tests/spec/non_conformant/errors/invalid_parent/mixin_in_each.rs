//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/invalid-parent/mixin-in-each.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("mixin-in-each")
}

#[test]
fn test() {
    assert_eq!(
        runner().err(
            "@each $item in (a, b) {\r\
             \n  @mixin foo() {}\r\
             \n}"
        ),
        "Error: Mixins may not be declared in control directives.\
         \n  ,\
         \n2 |   @mixin foo() {}\
         \n  |   ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:3  root stylesheet",
    );
}
