//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2975.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin test($name: false) {\
            \n    $a: \"\";\
            \n    $b: \"\";\
            \n    @if $name {\
            \n        $a: \"-#{$name}\";    // works as expected\
            \n        $b: -$name;         // here occurs the bug\
            \n    } @else {\
            \n        $a: \"\";\
            \n        $b: \"\";\
            \n    }\
            \n    \
            \n    .test-a#{$a} {\
            \n        display: block;\
            \n    }\
            \n    .test-b#{$b} {\
            \n        display: block;\
            \n    }\
            \n}\
            \n\
            \n@include test;\
            \n@include test(asdf);\
            \n@include test(foo1);\
            \n@include test(bar1);\
            \n// @include test(\"foo2\");\
            \n// @include test(\"bar2\");\
            \n"
        )
        .unwrap(),
        ".test-a {\
        \n  display: block;\
        \n}\
        \n.test-b {\
        \n  display: block;\
        \n}\
        \n.test-a-asdf {\
        \n  display: block;\
        \n}\
        \n.test-b-asdf {\
        \n  display: block;\
        \n}\
        \n.test-a-foo1 {\
        \n  display: block;\
        \n}\
        \n.test-b-foo1 {\
        \n  display: block;\
        \n}\
        \n.test-a-bar1 {\
        \n  display: block;\
        \n}\
        \n.test-b-bar1 {\
        \n  display: block;\
        \n}\
        \n"
    );
}
