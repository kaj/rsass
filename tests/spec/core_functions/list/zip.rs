//! Tests auto-converted from "sass-spec/spec/core_functions/list/zip.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod map {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn empty() {
        assert_eq!(
            runner().ok("a {b: inspect(zip(map-remove((c: d), c)))}\n"),
            "a {\
         \n  b: ();\
         \n}\n"
        );
    }
    #[test]
    fn non_empty() {
        assert_eq!(
            runner().ok("a {b: inspect(zip((c: d, e: f, g: h), 1 2 3))}\n"),
            "a {\
         \n  b: (c d) 1, (e f) 2, (g h) 3;\
         \n}\n"
        );
    }
}
#[test]
fn no_lists() {
    assert_eq!(
        runner().ok("@import \"core_functions/list/utils\";\n\
             \n$result: zip();\
             \na {\
             \n  value: inspect($result);\
             \n  separator: real-separator($result);\
             \n}\n"),
        "a {\
         \n  value: ();\
         \n  separator: comma;\
         \n}\n"
    );
}
#[test]
fn non_list() {
    assert_eq!(
        runner().ok("a {b: zip(c, d, e)}\n"),
        "a {\
         \n  b: c d e;\
         \n}\n"
    );
}
mod one_list {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn bracketed() {
        assert_eq!(
            runner().ok("@import \"core_functions/list/utils\";\n\
             \n$result: zip([1 2 3]);\
             \n$element: nth($result, 2);\n\
             \na {\
             \n  value: $result;\
             \n  element: $element {\
             \n    type: type-of($element);\
             \n    separator: real-separator($element);\
             \n  }\
             \n}\n"),
            "a {\
         \n  value: 1, 2, 3;\
         \n  element: 2;\
         \n  element-type: list;\
         \n  element-separator: space;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn comma() {
        assert_eq!(
            runner().ok("@import \"core_functions/list/utils\";\n\
             \n$result: zip((1, 2, 3));\
             \n$element: nth($result, 2);\n\
             \na {\
             \n  value: $result;\
             \n  element: $element {\
             \n    type: type-of($element);\
             \n    separator: real-separator($element);\
             \n  }\
             \n}\n"),
            "a {\
         \n  value: 1, 2, 3;\
         \n  element: 2;\
         \n  element-type: list;\
         \n  element-separator: space;\
         \n}\n"
        );
    }
    #[test]
    fn empty() {
        assert_eq!(
            runner().ok("@import \"core_functions/list/utils\";\n\
             \n$result: zip(());\
             \na {\
             \n  value: inspect($result);\
             \n  separator: real-separator($result);\
             \n}\n"),
            "a {\
         \n  value: ();\
         \n  separator: comma;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn space() {
        assert_eq!(
            runner().ok("@import \"core_functions/list/utils\";\n\
             \n$result: zip(1 2 3);\
             \n$element: nth($result, 2);\n\
             \na {\
             \n  value: $result;\
             \n  element: $element {\
             \n    type: type-of($element);\
             \n    separator: real-separator($element);\
             \n  }\
             \n}\n"),
            "a {\
         \n  value: 1, 2, 3;\
         \n  element: 2;\
         \n  element-type: list;\
         \n  element-separator: space;\
         \n}\n"
        );
    }
}
#[test]
fn three_lists() {
    assert_eq!(
        runner().ok("a {b: zip(1 2 3, c d e, red green blue)}\n"),
        "a {\
         \n  b: 1 c red, 2 d green, 3 e blue;\
         \n}\n"
    );
}
mod two_lists {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn first_empty() {
        assert_eq!(
            runner().ok("a {b: inspect(zip((), 1 2 3))}\n"),
            "a {\
         \n  b: ();\
         \n}\n"
        );
    }
    #[test]
    fn first_longer() {
        assert_eq!(
            runner().ok("a {b: zip(1 2 3 4, c d)}\n"),
            "a {\
         \n  b: 1 c, 2 d;\
         \n}\n"
        );
    }
    #[test]
    fn same_length() {
        assert_eq!(
            runner().ok("a {b: zip(1 2 3, c d e)}\n"),
            "a {\
         \n  b: 1 c, 2 d, 3 e;\
         \n}\n"
        );
    }
    #[test]
    fn second_empty() {
        assert_eq!(
            runner().ok("a {b: inspect(zip(1 2 3, ()))}\n"),
            "a {\
         \n  b: ();\
         \n}\n"
        );
    }
    #[test]
    fn second_longer() {
        assert_eq!(
            runner().ok("a {b: zip(1 2, c d e f)}\n"),
            "a {\
         \n  b: 1 c, 2 d;\
         \n}\n"
        );
    }
}
