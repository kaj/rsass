//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1801/import-cycle.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "@import \'alpha\';\
             \n"
        )
        .unwrap_err(),
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
         \n  input.scss 1:9   root stylesheet\
         \n",
    );
}
