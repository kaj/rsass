//! Tests auto-converted from "sass-spec/spec/core_functions/list/index.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("index")
}

mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:list\";\
             \na {b: list.index(c d e)}\n"
            ),
            "Error: Missing argument $value.\
         \n  ,--> input.scss\
         \n2 | a {b: list.index(c d e)}\
         \n  |       ^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:list\
         \n1 | @function index($list, $value) {\
         \n  |           ==================== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:list\";\
             \na {b: list.index(c d e, d, e)}\n"
            ),
            "Error: Only 2 arguments allowed, but 3 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: list.index(c d e, d, e)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:list\
         \n1 | @function index($list, $value) {\
         \n  |           ==================== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
mod found {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn first() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: list.index(a b c, a)}\n"),
            "a {\
         \n  b: 1;\
         \n}\n"
        );
    }
    #[test]
    fn last() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: list.index(a b c, c)}\n"),
            "a {\
         \n  b: 3;\
         \n}\n"
        );
    }
    #[test]
    fn map() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: list.index((c: d, e: f, g: h), e f)}\n"),
            "a {\
         \n  b: 2;\
         \n}\n"
        );
    }
    #[test]
    fn multiple() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: list.index(a b c a b c, b)}\n"),
            "a {\
         \n  b: 2;\
         \n}\n"
        );
    }
    #[test]
    fn non_list() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: list.index(c, c)}\n"),
            "a {\
         \n  b: 1;\
         \n}\n"
        );
    }
    #[test]
    fn sass_equality() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: list.index(1px 1in 1cm, 96px)}\n"),
            "a {\
         \n  b: 2;\
         \n}\n"
        );
    }
    #[test]
    fn single() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: list.index([c], c)}\n"),
            "a {\
         \n  b: 1;\
         \n}\n"
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \na {b: list.index($list: c d e, $value: d)}\n"),
        "a {\
         \n  b: 2;\
         \n}\n"
    );
}
mod not_found {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn empty() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \na {b: meta.inspect(list.index((), c))}\n"),
            "a {\
         \n  b: null;\
         \n}\n"
        );
    }
    mod map {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn empty() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \n@use \"core_functions/list/utils\";\
             \na {b: meta.inspect(list.index(utils.$empty-map, e))}\n"),
                "a {\
         \n  b: null;\
         \n}\n"
            );
        }
        #[test]
        fn non_empty() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \na {b: meta.inspect(list.index((c: d, e: f, g: h), e))}\n"),
                "a {\
         \n  b: null;\
         \n}\n"
            );
        }
    }
    #[test]
    fn non_empty() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \na {b: meta.inspect(list.index(c d e, f))}\n"),
            "a {\
         \n  b: null;\
         \n}\n"
        );
    }
    #[test]
    fn non_list() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \na {b: meta.inspect(list.index(c, d))}\n"),
            "a {\
         \n  b: null;\
         \n}\n"
        );
    }
}
