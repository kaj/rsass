//! Tests auto-converted from "sass-spec/spec/non_conformant/mixin/error/no_content.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "// A content block may not be passed to a mixin that doesn\'t include `@content`.\
             \n@mixin no-content {}\
             \n@include no-content {}\
             \n"
        ).unwrap_err(),
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
