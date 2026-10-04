//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2374.hrx"

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
         \n  background-color: rgb(100%, 99.9119647859%, 99.137254902%);\
         \n}\
         \n.yellow-200 {\
         \n  background-color: rgb(100%, 99.5038015206%, 95.137254902%);\
         \n}\
         \n.yellow-300 {\
         \n  background-color: rgb(100%, 99.0956382553%, 91.137254902%);\
         \n}\
         \n.yellow-400 {\
         \n  background-color: rgb(100%, 98.68747499%, 87.137254902%);\
         \n}\
         \n.yellow-500 {\
         \n  background-color: rgb(100%, 98.2793117247%, 83.137254902%);\
         \n}\
         \n.yellow-600 {\
         \n  background-color: rgb(100%, 97.8711484594%, 79.137254902%);\
         \n}\
         \n.yellow-700 {\
         \n  background-color: rgb(100%, 97.4629851941%, 75.137254902%);\
         \n}\
         \n.yellow-800 {\
         \n  background-color: rgb(100%, 97.0548219288%, 71.137254902%);\
         \n}\
         \n.yellow-900 {\
         \n  background-color: rgb(100%, 96.6466586635%, 67.137254902%);\
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
