//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/passing_all_as_keyword_args_in_opposite_order.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("passing_all_as_keyword_args_in_opposite_order")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "@mixin a-mixin($required, $arg1: default-val1, $arg2: default-val2) {\
             \n  required: $required;\
             \n  arg1: $arg1;\
             \n  arg2: $arg2; }\
             \n.mixed { @include a-mixin($arg2: non-default-val2, $arg1: non-default-val1, $required: foo); }\n"
        ),
        ".mixed {\
         \n  required: foo;\
         \n  arg1: non-default-val1;\
         \n  arg2: non-default-val2;\
         \n}\n"
    );
}
