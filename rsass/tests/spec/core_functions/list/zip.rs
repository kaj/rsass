//! Tests auto-converted from "sass-spec/spec/core_functions/list/zip.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("zip")
}

mod map {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn empty() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \n@use \"sass:map\";\
             \n@use \"sass:meta\";\
             \na {b: meta.inspect(list.zip(map.remove((c: d), c)))}\n"),
            "a {\
         \n  b: ();\
         \n}\n"
        );
    }
    #[test]
    fn non_empty() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \na {b: meta.inspect(list.zip((c: d, e: f, g: h), 1 2 3))}\n"),
            "a {\
         \n  b: (c d) 1, (e f) 2, (g h) 3;\
         \n}\n"
        );
    }
}
#[test]
fn no_lists() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \n@use \"core_functions/list/utils\";\n\
             \n$result: list.zip();\
             \na {\
             \n  value: meta.inspect($result);\
             \n  separator: utils.real-separator($result);\
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
        runner().ok("@use \"sass:list\";\
             \na {b: list.zip(c, d, e)}\n"),
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
            runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \n@use \"core_functions/list/utils\";\n\
             \n$result: list.zip([1 2 3]);\
             \n$element: list.nth($result, 2);\n\
             \na {\
             \n  value: $result;\
             \n  element: $element {\
             \n    type: meta.type-of($element);\
             \n    separator: utils.real-separator($element);\
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
    fn comma() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \n@use \"core_functions/list/utils\";\n\
             \n$result: list.zip((1, 2, 3));\
             \n$element: list.nth($result, 2);\n\
             \na {\
             \n  value: $result;\
             \n  element: $element {\
             \n    type: meta.type-of($element);\
             \n    separator: utils.real-separator($element);\
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
            runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \n@use \"core_functions/list/utils\";\n\
             \n$result: list.zip(());\
             \na {\
             \n  value: meta.inspect($result);\
             \n  separator: utils.real-separator($result);\
             \n}\n"),
            "a {\
         \n  value: ();\
         \n  separator: comma;\
         \n}\n"
        );
    }
    #[test]
    fn space() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \n@use \"core_functions/list/utils\";\n\
             \n$result: list.zip(1 2 3);\
             \n$element: list.nth($result, 2);\n\
             \na {\
             \n  value: $result;\
             \n  element: $element {\
             \n    type: meta.type-of($element);\
             \n    separator: utils.real-separator($element);\
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
        runner().ok("@use \"sass:list\";\
             \na {b: list.zip(1 2 3, c d e, red green blue)}\n"),
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
            runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \na {b: meta.inspect(list.zip((), 1 2 3))}\n"),
            "a {\
         \n  b: ();\
         \n}\n"
        );
    }
    #[test]
    fn first_longer() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: list.zip(1 2 3 4, c d)}\n"),
            "a {\
         \n  b: 1 c, 2 d;\
         \n}\n"
        );
    }
    #[test]
    fn same_length() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: list.zip(1 2 3, c d e)}\n"),
            "a {\
         \n  b: 1 c, 2 d, 3 e;\
         \n}\n"
        );
    }
    #[test]
    fn second_empty() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \na {b: meta.inspect(list.zip(1 2 3, ()))}\n"),
            "a {\
         \n  b: ();\
         \n}\n"
        );
    }
    #[test]
    fn second_longer() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: list.zip(1 2, c d e f)}\n"),
            "a {\
         \n  b: 1 c, 2 d;\
         \n}\n"
        );
    }
}
