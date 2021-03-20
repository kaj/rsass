//! Tests auto-converted from "sass-spec/spec/non_conformant/mixin/content/arguments/receiving.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "// Content blocks\' argument lists can define arguments in in all the same ways\
            \n// as they\'re defined in any other argument list.\
            \n\
            \nwith-defaults {\
            \n  nothing-passed {\
            \n    @mixin mixin {\
            \n      @content;\
            \n    }\
            \n\
            \n    @include mixin using ($arg1: value1, $arg2: value2) {\
            \n      arg1: $arg1;\
            \n      arg2: $arg2;\
            \n    }\
            \n  }\
            \n\
            \n  partial-override {\
            \n    @mixin mixin {\
            \n      @content($arg2: other2);\
            \n    }\
            \n\
            \n    @include mixin using ($arg1: value1, $arg2: value2) {\
            \n      arg1: $arg1;\
            \n      arg2: $arg2;\
            \n    }\
            \n  }\
            \n\
            \n  total-override {\
            \n    @mixin mixin {\
            \n      @content(other1, other2);\
            \n    }\
            \n\
            \n    @include mixin using ($arg1: value1, $arg2: value2) {\
            \n      arg1: $arg1;\
            \n      arg2: $arg2;\
            \n    }\
            \n  }\
            \n}\
            \n\
            \nwith-splat {\
            \n  nothing-passed {\
            \n    @mixin mixin {\
            \n      @content;\
            \n    }\
            \n\
            \n    @include mixin using ($args...) {\
            \n      positional: inspect($args);\
            \n      keywords: inspect(keywords($args));\
            \n    }\
            \n  }\
            \n\
            \n  positional-passed {\
            \n    @mixin mixin {\
            \n      @content(value1, value2, value3);\
            \n    }\
            \n\
            \n    @include mixin using ($args...) {\
            \n      positional: inspect($args);\
            \n      keywords: inspect(keywords($args));\
            \n    }\
            \n  }\
            \n\
            \n  named-passed {\
            \n    @mixin mixin {\
            \n      @content($arg1: value1, $arg2: value2);\
            \n    }\
            \n\
            \n    @include mixin using ($args...) {\
            \n      positional: inspect($args);\
            \n      keywords: inspect(keywords($args));\
            \n    }\
            \n  }\
            \n\
            \n  both-passed {\
            \n    @mixin mixin {\
            \n      @content(value1, $arg2: value2);\
            \n    }\
            \n\
            \n    @include mixin using ($args...) {\
            \n      positional: inspect($args);\
            \n      keywords: inspect(keywords($args));\
            \n    }\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "with-defaults nothing-passed {\
        \n  arg1: value1;\
        \n  arg2: value2;\
        \n}\
        \nwith-defaults partial-override {\
        \n  arg1: value1;\
        \n  arg2: other2;\
        \n}\
        \nwith-defaults total-override {\
        \n  arg1: other1;\
        \n  arg2: other2;\
        \n}\
        \nwith-splat nothing-passed {\
        \n  positional: ();\
        \n  keywords: ();\
        \n}\
        \nwith-splat positional-passed {\
        \n  positional: value1, value2, value3;\
        \n  keywords: ();\
        \n}\
        \nwith-splat named-passed {\
        \n  positional: ();\
        \n  keywords: (arg1: value1, arg2: value2);\
        \n}\
        \nwith-splat both-passed {\
        \n  positional: (value1,);\
        \n  keywords: (arg2: value2);\
        \n}\
        \n"
    );
}
