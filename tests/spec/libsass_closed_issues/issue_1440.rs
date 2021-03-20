//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1440.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "// works fine with plain @each directive\r\
            \n$i: 1;\r\
            \n$prop1: width;\r\
            \n$prop2: background-position;\r\
            \n$values: 132px,\r\
            \n    100px \"-100px -25px\",\r\
            \n    200px \"-500px -100px\";\r\
            \n\r\
            \n@each $value1, $value2 in $values{\r\
            \n  .okay#{$i} {\r\
            \n    #{$prop1}: #{$value1};\r\
            \n    #{$prop2}: #{$value2};\r\
            \n  }\r\
            \n  $i: ($i + 1);\r\
            \n}\r\
            \n\r\
            \n// when using @each inside @mixin with variable arguments($values...),\r\
            \n// $value2 is missing and no errors while compiling\r\
            \n@mixin eachProp($prop1, $prop2, $values...){\r\
            \n  $i: 1;\r\
            \n  @each $value1, $value2 in $values{\r\
            \n    .error#{$i} {\r\
            \n      #{$prop1}: #{$value1};\r\
            \n      #{$prop2}: #{$value2};\r\
            \n    }\r\
            \n    $i: ($i + 1);\r\
            \n  }\r\
            \n}\r\
            \n\r\
            \n@include eachProp($prop1, $prop2,\r\
            \n  132px,\r\
            \n    100px \"-100px -25px\",\r\
            \n    200px \"-500px -100px\"\r\
            \n);"
        )
        .unwrap(),
        ".okay1 {\
        \n  width: 132px;\
        \n}\
        \n.okay2 {\
        \n  width: 100px;\
        \n  background-position: -100px -25px;\
        \n}\
        \n.okay3 {\
        \n  width: 200px;\
        \n  background-position: -500px -100px;\
        \n}\
        \n.error1 {\
        \n  width: 132px;\
        \n}\
        \n.error2 {\
        \n  width: 100px;\
        \n  background-position: -100px -25px;\
        \n}\
        \n.error3 {\
        \n  width: 200px;\
        \n  background-position: -500px -100px;\
        \n}\
        \n"
    );
}
