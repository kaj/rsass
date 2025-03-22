//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1487.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1487")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "@mixin foo() {\
             \n    foo: &;\
             \n}\n\
             \nfoo {\
             \n  @include foo { bar: baz }\
             \n}\n"
        ),
        "Error: Mixin doesn\'t accept a content block.\
         \n    ,\
         \n1   | @mixin foo() {\
         \n    |        ===== declaration\
         \n... |\
         \n6   |   @include foo { bar: baz }\
         \n    |   ^^^^^^^^^^^^ invocation\
         \n    \'\
         \n  input.scss 6:3  root stylesheet",
    );
}
