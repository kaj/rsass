//! Tests auto-converted from "sass-spec/spec/libsass-todo-issues/issue_1801/simple-import-loop.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "@import \'susy\';\
             \n"
        )
        .unwrap_err(),
        "Error: This file is already being loaded.\
         \n  ,--> _susy.scss\
         \n1 | @import \'susy\';\
         \n  |         ^^^^^^ new load\
         \n  \'\
         \n  ,--> input.scss\
         \n1 | @import \'susy\';\
         \n  |         ====== original load\
         \n  \'\
         \n  _susy.scss 1:9  @import\
         \n  input.scss 1:9  root stylesheet",
    );
}
