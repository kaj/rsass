//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load_css/error/too_few_args.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("too_few_args")
}

#[test]
fn test() {
    assert_eq!(
        runner().err(
            "@use \"sass:meta\";\
             \n@include meta.load-css();\n"
        ),
        "Error: Missing argument $url.\
         \n  ,--> input.scss\
         \n2 | @include meta.load-css();\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @mixin load-css($url, $with: null) {\
         \n  |        =========================== declaration\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
    );
}
