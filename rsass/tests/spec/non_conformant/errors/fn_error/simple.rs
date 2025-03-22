//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/fn-error/simple.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("simple")
}

#[test]
fn test() {
    assert_eq!(
        runner().err("@error \"error\";"),
        "Error: \"error\"\
         \n  ,\
         \n1 | @error \"error\";\
         \n  | ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
    );
}
