//! Tests auto-converted from "sass-spec/spec/libsass-todo-issues/issue_1801/simple-import-loop.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("simple-import-loop")
        .mock_file("_susy.scss", "@import 'susy';\n")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "@import \'susy\';\n"
        ),
        "DEPRECATION WARNING: Sass @import rules are deprecated and will be removed in Dart Sass 3.0.0.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | @import \'susy\';\
         \n  |         ^^^^^^\
         \n  \'\
         \n    input.scss 1:9  root stylesheet\n\
         \nDEPRECATION WARNING: Sass @import rules are deprecated and will be removed in Dart Sass 3.0.0.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | @import \'susy\';\
         \n  |         ^^^^^^\
         \n  \'\
         \n    _susy.scss 1:9  @import\
         \n    input.scss 1:9  root stylesheet\n\
         \nError: This file is already being loaded.\
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
