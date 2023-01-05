//! Tests auto-converted from "sass-spec/spec/libsass-todo-issues/issue_1801/simple-import-loop.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("simple-import-loop")
        .mock_file("_susy.scss", "@import 'susy';\n")
}

#[test]
fn test() {
    assert_eq!(
        runner().err("@import \'susy\';\n"),
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
