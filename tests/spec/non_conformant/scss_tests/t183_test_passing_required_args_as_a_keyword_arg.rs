//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/183_test_passing_required_args_as_a_keyword_arg.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin a-mixin($required, $arg1: default-val1, $arg2: default-val2) {\
            \n  required: $required;\
            \n  arg1: $arg1;\
            \n  arg2: $arg2; }\
            \n.mixed { @include a-mixin($required: foo); }\
            \n"
        )
        .unwrap(),
        ".mixed {\
        \n  required: foo;\
        \n  arg1: default-val1;\
        \n  arg2: default-val2;\
        \n}\
        \n"
    );
}
