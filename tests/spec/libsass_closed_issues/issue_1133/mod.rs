//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1133"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/libsass-closed-issues/issue_1133/normal.hrx"
#[test]
fn normal() {
    assert_eq!(
        rsass(
            "@function foo($map) {\
            \n    @return $map;\
            \n}\
            \n\
            \na {\
            \n    $map: foo((this: is, my: map));\
            \n    @each $k, $v in $map {\
            \n        #{$k}: $v;\
            \n    }\
            \n}\
            \n\
            \nb {\
            \n    $map: call(\"foo\", (this: is, my: map));\
            \n    @each $k, $v in $map {\
            \n        #{$k}: $v;\
            \n    }\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  this: is;\
        \n  my: map;\
        \n}\
        \nb {\
        \n  this: is;\
        \n  my: map;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1133/vararg.hrx"
#[test]
#[ignore] // wrong result
fn vararg() {
    assert_eq!(
        rsass(
            "@function foo($this, $my) {\
            \n  @return (this: $this, my: $my);\
            \n}\
            \n\
            \na {\
            \n  $map: foo((this: is, my: map)...);\
            \n  @each $k, $v in $map {\
            \n    #{$k}: $v;\
            \n  }\
            \n}\
            \n\
            \nb {\
            \n  $map: call(\"foo\", (this: is, my: map)...);\
            \n  @each $k, $v in $map {\
            \n    #{$k}: $v;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  this: is;\
        \n  my: map;\
        \n}\
        \nb {\
        \n  this: is;\
        \n  my: map;\
        \n}\
        \n"
    );
}
