//! Tests auto-converted from "sass-spec/spec/non_conformant/mixin/content/arguments/passing.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "// Arguments can be passed to content blocks in all the same ways as they\'re\
            \n// passed to any other argument list.\
            \n\
            \nby-position {\
            \n  @mixin mixin {\
            \n    @content(value1, value2);\
            \n  }\
            \n\
            \n  @include mixin using ($arg1, $arg2) {\
            \n    arg1: $arg1;\
            \n    arg2: $arg2;\
            \n  }\
            \n}\
            \n\
            \nby-name {\
            \n  @mixin mixin {\
            \n    @content($arg1: value1, $arg2: value2);\
            \n  }\
            \n\
            \n  @include mixin using ($arg1, $arg2) {\
            \n    arg1: $arg1;\
            \n    arg2: $arg2;\
            \n  }\
            \n}\
            \n\
            \nlist-splat {\
            \n  @mixin mixin {\
            \n    @content((value1 value2)...);\
            \n  }\
            \n\
            \n  @include mixin using ($arg1, $arg2) {\
            \n    arg1: $arg1;\
            \n    arg2: $arg2;\
            \n  }\
            \n}\
            \n\
            \nmap-splat {\
            \n  @mixin mixin {\
            \n    @content((arg1: value1, arg2: value2)...);\
            \n  }\
            \n\
            \n  @include mixin using ($arg1, $arg2) {\
            \n    arg1: $arg1;\
            \n    arg2: $arg2;\
            \n  }\
            \n}\
            \n\
            \nmixed-splat {\
            \n  @mixin mixin {\
            \n    @content((value1,)..., (arg2: value2)...);\
            \n  }\
            \n\
            \n  @include mixin using ($arg1, $arg2) {\
            \n    arg1: $arg1;\
            \n    arg2: $arg2;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "by-position {\
        \n  arg1: value1;\
        \n  arg2: value2;\
        \n}\
        \nby-name {\
        \n  arg1: value1;\
        \n  arg2: value2;\
        \n}\
        \nlist-splat {\
        \n  arg1: value1;\
        \n  arg2: value2;\
        \n}\
        \nmap-splat {\
        \n  arg1: value1;\
        \n  arg2: value2;\
        \n}\
        \nmixed-splat {\
        \n  arg1: value1;\
        \n  arg2: value2;\
        \n}\
        \n"
    );
}
