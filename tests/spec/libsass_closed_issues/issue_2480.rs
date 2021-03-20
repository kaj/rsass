//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2480.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin main(\r\
            \n  $param1: param1,\r\
            \n  $param2: param2,\r\
            \n  $param3: param3\r\
            \n) {\r\
            \n  param1-value: $param1;\r\
            \n  param2-value: $param2;\r\
            \n  param3-value: $param3;\r\
            \n}\r\
            \n\r\
            \n@mixin router($args...) {\r\
            \n  @if (true) { @include main($args...) }\r\
            \n  @else { @include main2($args...) }\r\
            \n}\r\
            \n\r\
            \n@mixin helper($args...) {\r\
            \n  @include router($param2: param__2, $args...)\r\
            \n}\r\
            \n\r\
            \n.ordinal-arguments {\r\
            \n  @include helper(param___1);\r\
            \n}\r\
            \n\r\
            \n.named-arguments {\r\
            \n  @include helper($param1: param___1);\r\
            \n}"
        )
        .unwrap(),
        ".ordinal-arguments {\
        \n  param1-value: param___1;\
        \n  param2-value: param__2;\
        \n  param3-value: param3;\
        \n}\
        \n.named-arguments {\
        \n  param1-value: param___1;\
        \n  param2-value: param__2;\
        \n  param3-value: param3;\
        \n}\
        \n"
    );
}
