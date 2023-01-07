//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1801/import-cycle.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("import-cycle")
        .mock_file("_alpha.scss", "@import 'beta';\n")
        .mock_file("_beta.scss", "@import 'alpha';\n")
}

#[test]
fn test() {
    assert_eq!(
        runner().err("@import \'alpha\';\n"),
        "Error: This file is already being loaded.\
         \n  ,--> _beta.scss\
         \n1 | @import \'alpha\';\
         \n  |         ^^^^^^^ new load\
         \n  \'\
         \n  ,--> input.scss\
         \n1 | @import \'alpha\';\
         \n  |         ======= original load\
         \n  \'\
         \n  _beta.scss 1:9   @import\
         \n  _alpha.scss 1:9  @import\
         \n  input.scss 1:9   root stylesheet",
    );
}
