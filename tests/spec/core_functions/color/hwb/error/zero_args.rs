//! Tests auto-converted from "sass-spec/spec/core_functions/color/hwb/error/zero_args.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "@use \'sass:color\';\
             \na {b: color.hwb()}\
             \n"
        )
        .unwrap_err(),
        "Error: Missing argument $channels.\
         \n  ,--> input.scss\
         \n2 | a {b: color.hwb()}\
         \n  |       ^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,\
         \n1 | @function hwb($channels) {\
         \n  |           ============== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
    );
}
