//! Tests auto-converted from "sass-spec/spec/non_conformant/mixin/content/arguments"
#[allow(unused)]
use super::rsass;

mod error;

// From "sass-spec/spec/non_conformant/mixin/content/arguments/none.hrx"
#[test]
#[ignore] // unexepected error
fn none() {
    assert_eq!(
        rsass(
            "// `@content()` and `@content` should behave identically, as should\
            \n// `@include foo` and `@include foo using ()`.\
            \n\
            \nno-parens {\
            \n  @mixin mixin {\
            \n    @content;\
            \n  }\
            \n\
            \n  empty-using {\
            \n    @include mixin using () {\
            \n      x: y;\
            \n    }\
            \n  }\
            \n\
            \n  defaults {\
            \n    @include mixin using ($arg1: value1, $arg2: value2) {\
            \n      arg1: $arg1;\
            \n      arg2: $arg2;\
            \n    }\
            \n  }\
            \n}\
            \n\
            \nparens {\
            \n  @mixin mixin {\
            \n    @content();\
            \n  }\
            \n\
            \n  no-using {\
            \n    @include mixin {\
            \n      x: y;\
            \n    }\
            \n  }\
            \n\
            \n  empty-using {\
            \n    @include mixin using () {\
            \n      x: y;\
            \n    }\
            \n  }\
            \n\
            \n  defaults {\
            \n    @include mixin using ($arg1: value1, $arg2: value2) {\
            \n      arg1: $arg1;\
            \n      arg2: $arg2;\
            \n    }\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "no-parens empty-using {\
        \n  x: y;\
        \n}\
        \nno-parens defaults {\
        \n  arg1: value1;\
        \n  arg2: value2;\
        \n}\
        \nparens no-using {\
        \n  x: y;\
        \n}\
        \nparens empty-using {\
        \n  x: y;\
        \n}\
        \nparens defaults {\
        \n  arg1: value1;\
        \n  arg2: value2;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/mixin/content/arguments/passing.hrx"
#[test]
#[ignore] // unexepected error
fn passing() {
    assert_eq!(
        rsass(
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

// From "sass-spec/spec/non_conformant/mixin/content/arguments/receiving.hrx"
#[test]
#[ignore] // unexepected error
fn receiving() {
    assert_eq!(
        rsass(
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

// From "sass-spec/spec/non_conformant/mixin/content/arguments/scope.hrx"
#[test]
#[ignore] // unexepected error
fn scope() {
    assert_eq!(
        rsass(
            "// Arguments passed to @content blocks are lexically-scoped within those blocks.\
            \n\
            \n$var: top-level;\
            \n\
            \n@mixin mixin($var) {\
            \n  mixin-var-before: $var;\
            \n  @content(content-argument);\
            \n  mixin-var-after: $var;\
            \n}\
            \n\
            \n@mixin inner {\
            \n  var-in-inner: $var;\
            \n}\
            \n\
            \nscope {\
            \n  var-before: $var;\
            \n  @include mixin(mixin-argument) using ($var) {\
            \n    content-var-before: $var;\
            \n    @include inner;\
            \n    content-var-after: $var;\
            \n  }\
            \n  var-after: $var;\
            \n}\
            \n"
        )
        .unwrap(),
        "scope {\
        \n  var-before: top-level;\
        \n  mixin-var-before: mixin-argument;\
        \n  content-var-before: content-argument;\
        \n  var-in-inner: top-level;\
        \n  content-var-after: content-argument;\
        \n  mixin-var-after: mixin-argument;\
        \n  var-after: top-level;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/mixin/content/arguments/weird_syntax.hrx"
#[test]
#[ignore] // unexepected error
fn weird_syntax() {
    assert_eq!(
        rsass(
            "no-space-before-using {\
            \n  @mixin mixin {\
            \n    @content(value1, value2);\
            \n  }\
            \n\
            \n  @include mixin()using ($arg1, $arg2) {\
            \n    arg1: $arg1;\
            \n    arg2: $arg2;\
            \n  }\
            \n}\
            \n\
            \nno-space-after-using {\
            \n  @mixin mixin {\
            \n    @content(value1, value2);\
            \n  }\
            \n\
            \n  @include mixin using($arg1, $arg2) {\
            \n    arg1: $arg1;\
            \n    arg2: $arg2;\
            \n  }\
            \n}\
            \n\
            \nno-space-after-arglist {\
            \n  @mixin mixin {\
            \n    @content(value1, value2);\
            \n  }\
            \n\
            \n  @include mixin using($arg1, $arg2){\
            \n    arg1: $arg1;\
            \n    arg2: $arg2;\
            \n  }\
            \n}\
            \n\
            \nspace-after-content {\
            \n  @mixin mixin {\
            \n    @content (value1, value2);\
            \n  }\
            \n\
            \n  @include mixin using ($arg1, $arg2) {\
            \n    arg1: $arg1;\
            \n    arg2: $arg2;\
            \n  }\
            \n}\
            \n\
            \ncase-insensitive-using {\
            \n  @mixin mixin {\
            \n    @content(value1, value2);\
            \n  }\
            \n\
            \n  @include mixin UsInG ($arg1, $arg2) {\
            \n    arg1: $arg1;\
            \n    arg2: $arg2;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "no-space-before-using {\
        \n  arg1: value1;\
        \n  arg2: value2;\
        \n}\
        \nno-space-after-using {\
        \n  arg1: value1;\
        \n  arg2: value2;\
        \n}\
        \nno-space-after-arglist {\
        \n  arg1: value1;\
        \n  arg2: value2;\
        \n}\
        \nspace-after-content {\
        \n  arg1: value1;\
        \n  arg2: value2;\
        \n}\
        \ncase-insensitive-using {\
        \n  arg1: value1;\
        \n  arg2: value2;\
        \n}\
        \n"
    );
}
