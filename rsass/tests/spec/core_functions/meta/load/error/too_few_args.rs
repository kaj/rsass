//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load/error/too_few_args.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("too_few_args")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "@use \"sass:meta\";\
             \n$_: meta.load();\n"
        ),
        "Error: Missing argument $url.\
         \n  ,--> input.scss\
         \n2 | $_: meta.load();\
         \n  |     ^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function load($url, $with: null) {\
         \n  |           ======================= declaration\
         \n  \'\
         \n  input.scss 2:5  root stylesheet",
    );
}
