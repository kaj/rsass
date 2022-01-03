//! Tests auto-converted from "sass-spec/spec/core_functions/list/separator.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn bracketed() {
    assert_eq!(
        runner().ok("a {b: list-separator([c, d])}\n"),
        "a {\
         \n  b: comma;\
         \n}\n"
    );
}
mod empty {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn comma() {
        assert_eq!(
            runner().ok("a {b: list-separator(join((), (), comma))}\n"),
            "a {\
         \n  b: comma;\
         \n}\n"
        );
    }
    #[test]
    fn map() {
        assert_eq!(
            runner().ok("@import \"core_functions/list/utils\";\n\
             \na {b: list-separator($empty-map)}\n"),
            "a {\
         \n  b: space;\
         \n}\n"
        );
    }
    #[test]
    fn space() {
        assert_eq!(
            runner().ok("a {b: list-separator(())}\n"),
            "a {\
         \n  b: space;\
         \n}\n"
        );
    }
}
mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err("a {b: list-separator()}\n"),
            "Error: Missing argument $list.\
         \n  ,--> input.scss\
         \n1 | a {b: list-separator()}\
         \n  |       ^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:list\
         \n1 | @function separator($list) {\
         \n  |           ================ declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err("a {b: list-separator(c, d)}\n"),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: list-separator(c, d)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:list\
         \n1 | @function separator($list) {\
         \n  |           ================ declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
mod multi {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn comma() {
        assert_eq!(
            runner().ok("a {b: list-separator((1, 2, 3))}\n"),
            "a {\
         \n  b: comma;\
         \n}\n"
        );
    }
    #[test]
    fn map() {
        assert_eq!(
            runner().ok("a {b: list-separator((c: d, e: f, g: h))}\n"),
            "a {\
         \n  b: comma;\
         \n}\n"
        );
    }
    #[test]
    fn slash() {
        assert_eq!(
            runner().ok("@use \'sass:list\';\
             \na {b: list-separator(list.slash(1, 2, 3))}\n"),
            "a {\
         \n  b: slash;\
         \n}\n"
        );
    }
    #[test]
    fn space() {
        assert_eq!(
            runner().ok("a {b: list-separator(1 2 3)}\n"),
            "a {\
         \n  b: space;\
         \n}\n"
        );
    }
}
mod single {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn comma() {
        assert_eq!(
            runner().ok("a {b: list-separator((1,))}\n"),
            "a {\
         \n  b: comma;\
         \n}\n"
        );
    }
    #[test]
    fn non_list() {
        assert_eq!(
            runner().ok("a {b: list-separator(1)}\n"),
            "a {\
         \n  b: space;\
         \n}\n"
        );
    }
    #[test]
    fn slash() {
        assert_eq!(
            runner().ok(
                "a {b: list-separator(join(1, (), $separator: slash))}\n"
            ),
            "a {\
         \n  b: slash;\
         \n}\n"
        );
    }
    #[test]
    fn space() {
        assert_eq!(
            runner().ok("a {b: list-separator([1])}\n"),
            "a {\
         \n  b: space;\
         \n}\n"
        );
    }
}
