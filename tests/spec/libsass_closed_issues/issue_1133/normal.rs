//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1133/normal.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
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
