//! Tests auto-converted from "sass-spec/spec/core_functions/list/length.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("length")
}

#[test]
fn t0() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \na {b: list.length(())}\n"),
        "a {\
         \n  b: 0;\
         \n}\n"
    );
}
#[test]
fn t1() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \na {b: list.length(list.join((), 1))}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
#[test]
fn t2() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \na {b: list.length(c d)}\n"),
        "a {\
         \n  b: 2;\
         \n}\n"
    );
}
mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:list\";\
             \na {b: list.length()}\n"
            ),
            "Error: Missing argument $list.\
         \n  ,--> input.scss\
         \n2 | a {b: list.length()}\
         \n  |       ^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:list\
         \n1 | @function length($list) {\
         \n  |           ============= declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:list\";\
             \na {b: list.length(1, 2)}\n"
            ),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: list.length(1, 2)}\
         \n  |       ^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:list\
         \n1 | @function length($list) {\
         \n  |           ============= declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
#[test]
fn many() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \na {b: list.length((1, 2, 3, 4, 5))}\n"),
        "a {\
         \n  b: 5;\
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
             \n@use \"core_functions/list/utils\";\
             \na {b: list.length(utils.$empty-map)}\n"),
            "a {\
         \n  b: 0;\
         \n}\n"
        );
    }
    #[test]
    fn non_empty() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: list.length((1: 2, 3: 4))}\n"),
            "a {\
         \n  b: 2;\
         \n}\n"
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \na {b: list.length($list: 1 2 3)}\n"),
        "a {\
         \n  b: 3;\
         \n}\n"
    );
}
#[test]
fn non_list() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \na {b: list.length(c)}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
#[test]
fn null_list_item() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \n// regression test for scssphp/scssphp#403\
             \na {b: list.length((null))}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
