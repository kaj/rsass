//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2309.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "$button-sizes: (\r\
            \n  \'xs\': (\r\
            \n    \'line-height\': 16 / 12,\r\
            \n  ),\r\
            \n  \'s\': (\r\
            \n    \'line-height\': 18 / 14,\r\
            \n  ),\r\
            \n  \'m\': (\r\
            \n    \'line-height\': 18 / 14,\r\
            \n  ),\r\
            \n  \'l\': (\r\
            \n    \'line-height\': 22 / 16,\r\
            \n  )\r\
            \n);\r\
            \n\r\
            \n@each $size in $button-sizes {\r\
            \n  $size-metrics: nth($size, 2);\r\
            \n\r\
            \n  .c-button__icon {\r\
            \n    min-height: map-get($size-metrics, \'line-height\') * 1em;\r\
            \n  }\r\
            \n}"
        )
        .unwrap(),
        ".c-button__icon {\
        \n  min-height: 1.3333333333em;\
        \n}\
        \n.c-button__icon {\
        \n  min-height: 1.2857142857em;\
        \n}\
        \n.c-button__icon {\
        \n  min-height: 1.2857142857em;\
        \n}\
        \n.c-button__icon {\
        \n  min-height: 1.375em;\
        \n}\
        \n"
    );
}
