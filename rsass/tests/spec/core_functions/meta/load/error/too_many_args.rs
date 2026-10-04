//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load/error/too_many_args.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("too_many_args")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "@use \"sass:meta\";\
             \n$_: meta.load(\"other\", (), \"a\");\n"
        ),
        "Error: Only 2 arguments allowed, but 3 were passed.\
         \n  ,--> input.scss\
         \n2 | $_: meta.load(\"other\", (), \"a\");\
         \n  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function load($url, $with: null) {\
         \n  |           ======================= declaration\
         \n  \'\
         \n  input.scss 2:5  root stylesheet",
    );
}
