//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1255.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1255")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(
            "@use \"sass:list\";\
             \n@function double($value) {\
             \n  @return $value * 2;\
             \n}\n\
             \n@mixin dummy-bug($args...) {\
             \n  @for $i from 1 through list.length($args) {\
             \n    $args: list.set-nth($args, $i, double(list.nth($args, $i)));\
             \n  }\n\
             \n  content: $args;\
             \n}\n\
             \n.foo {\
             \n  @include dummy-bug(1, 2, 3, 4);\
             \n}"
        ),
        ".foo {\
         \n  content: 2, 4, 6, 8;\
         \n}\n"
    );
}
