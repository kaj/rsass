//! Tests auto-converted from "sass-spec/spec/non_conformant/mixin/content/arguments/scope.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
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
