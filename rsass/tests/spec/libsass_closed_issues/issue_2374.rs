//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2374.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2374")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \n$colors: (\
             \n    yellow: #ffeb3b\
             \n);\
             \n@each $name, $color in $colors {\
             \n    $amount: 40%;\
             \n    @for $i from 0 through 9 {\
             \n        .#{$name}-#{($i*100)} { background-color: color.adjust($color, $lightness: $amount) };\
             \n        $amount: $amount - 2;\
             \n    }\
             \n}\n\
             \n$colors: (\
             \n    yellow: yellow,\
             \n    red: red,\
             \n    blue: blue,\n\
             \n);\
             \n@each $name, $color in $colors {\
             \n    @for $i from 0 through 2 {\
             \n        .#{$name}-#{($i*100)} {\
             \n          background-color: color.adjust($color, $lightness: 10%)\
             \n        };\
             \n    }\
             \n}\n\n"
        ),
        ".yellow-0 {\
         \n  background-color: hsl(53.8775510204, 100%, 101.568627451%);\
         \n}\
         \n.yellow-100 {\
         \n  background-color: rgb(255, 254.7755102041, 252.8);\
         \n}\
         \n.yellow-200 {\
         \n  background-color: rgb(255, 253.7346938776, 242.6);\
         \n}\
         \n.yellow-300 {\
         \n  background-color: rgb(255, 252.693877551, 232.4);\
         \n}\
         \n.yellow-400 {\
         \n  background-color: rgb(255, 251.6530612245, 222.2);\
         \n}\
         \n.yellow-500 {\
         \n  background-color: rgb(255, 250.612244898, 212);\
         \n}\
         \n.yellow-600 {\
         \n  background-color: rgb(255, 249.5714285714, 201.8);\
         \n}\
         \n.yellow-700 {\
         \n  background-color: rgb(255, 248.5306122449, 191.6);\
         \n}\
         \n.yellow-800 {\
         \n  background-color: rgb(255, 247.4897959184, 181.4);\
         \n}\
         \n.yellow-900 {\
         \n  background-color: rgb(255, 246.4489795918, 171.2);\
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
         \n}\n"
    );
}
