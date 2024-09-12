//! Tests auto-converted from "sass-spec/spec/core_functions/color/hwb/error/zero_args.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("zero_args")
}

#[test]
fn test() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.hwb()}\n"
        ),
        "Error: Missing argument $channels.\
         \n  ,--> input.scss\
         \n2 | a {b: color.hwb()}\
         \n  |       ^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,\
         \n1 | @function hwb($channels) {\
         \n  |           ============== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
