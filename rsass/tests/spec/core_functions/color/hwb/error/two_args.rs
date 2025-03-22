//! Tests auto-converted from "sass-spec/spec/core_functions/color/hwb/error/two_args.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("two_args")
}

#[test]
fn test() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.hwb(#123, 0.5)}\n"
        ),
        "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: color.hwb(#123, 0.5)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,\
         \n1 | @function hwb($channels) {\
         \n  |           ============== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
