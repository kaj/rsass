//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load_css/error/too_many_args.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("too_many_args")
}

#[test]
fn test() {
    assert_eq!(
        runner().err(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"other\", (), \"a\");\n"
        ),
        "Error: Only 2 arguments allowed, but 3 were passed.\
         \n  ,--> input.scss\
         \n2 | @include meta.load-css(\"other\", (), \"a\");\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @mixin load-css($url, $with: null) {\
         \n  |        =========================== declaration\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
    );
}
