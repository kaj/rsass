//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/mixin_with_keyword_args.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("mixin_with_keyword_args")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "@mixin a-mixin($required, $arg1: default-val1, $arg2: default-val2) {\
             \n  required: $required;\
             \n  arg1: $arg1;\
             \n  arg2: $arg2;\
             \n}\
             \n.mixed { @include a-mixin(foo, $arg2: non-default-val2); }\n"
        ),
        ".mixed {\
         \n  required: foo;\
         \n  arg1: default-val1;\
         \n  arg2: non-default-val2;\
         \n}\n"
    );
}
