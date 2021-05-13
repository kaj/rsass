//! Tests auto-converted from "sass-spec/spec/non_conformant/mixin/error/no_content.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "// A content block may not be passed to a mixin that doesn\'t include `@content`.\
             \n@mixin no-content {}\
             \n@include no-content {}\n"
        ),
        "Error: Mixin doesn\'t accept a content block.\
         \n  ,\
         \n2 | @mixin no-content {}\
         \n  |        ========== declaration\
         \n3 | @include no-content {}\
         \n  | ^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  input.scss 3:1  root stylesheet",
    );
}
