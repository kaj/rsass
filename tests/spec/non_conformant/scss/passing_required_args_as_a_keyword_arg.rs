//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/passing_required_args_as_a_keyword_arg.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("passing_required_args_as_a_keyword_arg")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "@mixin a-mixin($required, $arg1: default-val1, $arg2: default-val2) {\
             \n  required: $required;\
             \n  arg1: $arg1;\
             \n  arg2: $arg2; }\
             \n.mixed { @include a-mixin($required: foo); }\n"
        ),
        ".mixed {\
         \n  required: foo;\
         \n  arg1: default-val1;\
         \n  arg2: default-val2;\
         \n}\n"
    );
}
