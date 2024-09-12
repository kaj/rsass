//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2394.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2394")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "@use \"sass:map\";\
             \n@use \"sass:meta\";\
             \n@mixin brokenTest($color: red, $variableArguments...) {\r\
             \n  $width: map.get(meta.keywords($variableArguments), width);\r\
             \n  a {\r\
             \n    width: $width;\r\
             \n    color: $color;\r\
             \n  }\r\
             \n}\r\
             \n\r\
             \n@mixin workingTest($variableArguments...) {\r\
             \n  $width: map.get(meta.keywords($variableArguments), width);\r\
             \n  $color: map.get(meta.keywords($variableArguments), color);\r\
             \n  a {\r\
             \n    width: $width;\r\
             \n    color: $color;\r\
             \n  }\r\
             \n}\r\
             \n\r\
             \n@include brokenTest($width: 30px, $color: blue); // #1 fails\r\
             \n@include brokenTest($color: blue, $width: 30px); // #2 fails\r\
             \n@include brokenTest(blue, $width: 30px); // #3 works (!)\r\
             \n@include workingTest($width: 30px, $color: blue); // #4 works\r\
             \n@include workingTest($color: blue, $width: 30px); // #5 works\r\n"
        ),
        "a {\
         \n  width: 30px;\
         \n  color: blue;\
         \n}\
         \na {\
         \n  width: 30px;\
         \n  color: blue;\
         \n}\
         \na {\
         \n  width: 30px;\
         \n  color: blue;\
         \n}\
         \na {\
         \n  width: 30px;\
         \n  color: blue;\
         \n}\
         \na {\
         \n  width: 30px;\
         \n  color: blue;\
         \n}\n"
    );
}
