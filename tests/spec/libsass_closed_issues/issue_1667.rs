//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1667.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$map: (\r\
            \n  1: 1,\r\
            \n  1px: 1px\r\
            \n);\r\
            \n\r\
            \nfoo {\r\
            \n  a: map-get($map, 1);\r\
            \n  b: map-get($map, 1px);\r\
            \n}\r\
            \n\r\
            \n$type-scale: (\r\
            \n\t-15:0.066667rem,\r\
            \n\t-10:0.186rem,\r\
            \n\t-9:0.211rem,\r\
            \n\t-8:0.26rem,\r\
            \n\t-7:0.295rem,\r\
            \n\t-6:0.364rem,\r\
            \n\t-5:0.413rem,\r\
            \n\t-4:0.51rem,\r\
            \n\t-3:0.578rem,\r\
            \n\t-2:0.714rem,\r\
            \n\t-1:0.809rem,\r\
            \n\t0:1rem,\r\
            \n\t1:1.133rem,\r\
            \n\t2:1.4rem,\r\
            \n\t3:1.586rem,\r\
            \n\t4:1.96rem,\r\
            \n\t5:2.221rem,\r\
            \n\t6:2.744rem,\r\
            \n\t7:3.109rem,\r\
            \n\t8:3.842rem,\r\
            \n\t9:4.353rem,\r\
            \n\t10:5.378rem,\r\
            \n\t11:6.094rem,\r\
            \n\t12:7.53rem,\r\
            \n\t13:8.531rem,\r\
            \n\t14:10.541rem,\r\
            \n\t15:11.943rem,\r\
            \n\t16:14.758rem\r\
            \n);\r\
            \n\r\
            \n@function get-size($size) {\r\
            \n\t@if map-has-key($type-scale, $size) {\r\
            \n\t\t@return map-get($type-scale, $size);\r\
            \n\t}\r\
            \n\r\
            \n\t@warn \"Not a valid size.\";\r\
            \n\t@return null;\r\
            \n}\r\
            \n\r\
            \n@function scale-size($rem-size, $steps) {\r\
            \n\t$size-key: get-key-for-value($type-scale, $rem-size);\r\
            \n\r\
            \n\t@if $size-key {\r\
            \n\t\t$new-size: $size-key + $steps;\r\
            \n\t\t@return get-size($new-size);\r\
            \n\t}\r\
            \n\r\
            \n\t@warn \"Not able to find size for \" + $rem-size;\r\
            \n\t@return null;\r\
            \n}\r\
            \n\r\
            \n@function get-key-for-value($map, $value) {\r\
            \n\t@each $map-key, $map-value in $map {\r\
            \n\t\t@if $map-value == $value {\r\
            \n\t\t\t@return $map-key\r\
            \n\t\t}\r\
            \n\t}\r\
            \n\r\
            \n\t@warn $value + \" not found in \" + $map;\r\
            \n\t@return null;\r\
            \n}\r\
            \n\r\
            \n$h1-font-size: get-size(5);\r\
            \n\r\
            \n\r\
            \nh1 {\r\
            \n  font-size: $h1-font-size;\r\
            \n\r\
            \n  small {\r\
            \n    font-size: scale-size($h1-font-size, -2);\r\
            \n    color: red;\r\
            \n  }\r\
            \n}\r\
            \n\r\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: 1;\
        \n  b: 1px;\
        \n}\
        \nh1 {\
        \n  font-size: 2.221rem;\
        \n}\
        \nh1 small {\
        \n  font-size: 1.586rem;\
        \n  color: red;\
        \n}\
        \n"
    );
}
