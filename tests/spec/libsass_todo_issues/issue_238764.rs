//! Tests auto-converted from "sass-spec/spec/libsass-todo-issues/issue_238764.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin bar {\r\
             \n  @at-root @bar {a: b}\r\
             \n}\r\
             \n\r\
             \n.foo {\r\
             \n  @include bar;\r\
             \n}"
        )
        .unwrap_err(),
        "Error: expected selector.\
         \n  ,\
         \n2 |   @at-root @bar{a: b}\
         \n  |            ^\
         \n  \'\
         \n  input.scss 2:12  bar()\
         \n  input.scss 6:3   root stylesheet\
         \n",
    );
}
