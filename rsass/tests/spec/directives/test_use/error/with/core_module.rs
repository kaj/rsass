//! Tests auto-converted from "sass-spec/spec/directives/use/error/with/core_module.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("core_module")
}

#[test]
fn test() {
    assert_eq!(
        runner().err("@use \"sass:color\" with ($a: b);\n"),
        "Error: Built-in modules can\'t be configured.\
         \n  ,\
         \n1 | @use \"sass:color\" with ($a: b);\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
    );
}
