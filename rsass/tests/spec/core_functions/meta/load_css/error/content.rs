//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load_css/error/content.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("content")
        .mock_file("_other.scss", "")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"other\") {};\n"
        ),
        "Error: Mixin doesn\'t accept a content block.\
         \n  ,--> input.scss\
         \n2 | @include meta.load-css(\"other\") {};\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @mixin load-css($url, $with: null) {\
         \n  |        =========================== declaration\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
    );
}
