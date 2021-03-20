//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2374.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "$colors: (\r\
            \n    yellow: #ffeb3b\r\
            \n);\r\
            \n@each $name, $color in $colors {\r\
            \n    $amount: 40;\r\
            \n    @for $i from 0 through 9 {\r\
            \n        .#{$name}-#{($i*100)} { background-color: lighten($color, $amount) };\r\
            \n        $amount: $amount - 2;\r\
            \n    }\r\
            \n}\r\
            \n\r\
            \n$colors: (\r\
            \n    yellow: yellow,\r\
            \n    red: red,\r\
            \n    blue: blue,\r\
            \n    \r\
            \n);\r\
            \n@each $name, $color in $colors {\r\
            \n    @for $i from 0 through 2 {\r\
            \n        .#{$name}-#{($i*100)} { \r\
            \n          background-color: lighten($color, 10) \r\
            \n        };\r\
            \n    }\r\
            \n}\r\
            \n\r\
            \n"
        )
        .unwrap(),
        ".yellow-0 {\
        \n  background-color: white;\
        \n}\
        \n.yellow-100 {\
        \n  background-color: #fffffd;\
        \n}\
        \n.yellow-200 {\
        \n  background-color: #fffef3;\
        \n}\
        \n.yellow-300 {\
        \n  background-color: #fffde8;\
        \n}\
        \n.yellow-400 {\
        \n  background-color: #fffcde;\
        \n}\
        \n.yellow-500 {\
        \n  background-color: #fffbd4;\
        \n}\
        \n.yellow-600 {\
        \n  background-color: #fffaca;\
        \n}\
        \n.yellow-700 {\
        \n  background-color: #fff9c0;\
        \n}\
        \n.yellow-800 {\
        \n  background-color: #fff7b5;\
        \n}\
        \n.yellow-900 {\
        \n  background-color: #fff6ab;\
        \n}\
        \n.yellow-0 {\
        \n  background-color: #ffff33;\
        \n}\
        \n.yellow-100 {\
        \n  background-color: #ffff33;\
        \n}\
        \n.yellow-200 {\
        \n  background-color: #ffff33;\
        \n}\
        \n.red-0 {\
        \n  background-color: #ff3333;\
        \n}\
        \n.red-100 {\
        \n  background-color: #ff3333;\
        \n}\
        \n.red-200 {\
        \n  background-color: #ff3333;\
        \n}\
        \n.blue-0 {\
        \n  background-color: #3333ff;\
        \n}\
        \n.blue-100 {\
        \n  background-color: #3333ff;\
        \n}\
        \n.blue-200 {\
        \n  background-color: #3333ff;\
        \n}\
        \n"
    );
}
