//! Tests auto-converted from "sass-spec/spec/core_functions/map/set.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn empty() {
    assert_eq!(
        runner().ok("@use \"sass:map\";\
             \na {b: inspect(map.set((), c, d))}\n"),
        "a {\
         \n  b: (c: d);\
         \n}\n"
    );
}
mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn one_arg() {
        assert_eq!(
            runner().err(
                "@use \"sass:map\";\
             \na {b: map.set((c: d))}\n"
            ),
            "Error: Expected $args to contain a key.\
         \n  ,\
         \n2 | a {b: map.set((c: d))}\
         \n  |       ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn two_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:map\";\
             \na {b: map.set((c: d), e)}\n"
            ),
            "Error: Expected $args to contain a value.\
         \n  ,\
         \n2 | a {b: map.set((c: d), e)}\
         \n  |       ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            runner().err(
                "@use \"sass:map\";\
             \na {b: map.set(1, c, d)}\n"
            ),
            "Error: $map: 1 is not a map.\
         \n  ,\
         \n2 | a {b: map.set(1, c, d)}\
         \n  |       ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn zero_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:map\";\
             \na {b: map.set()}\n"
            ),
            "Error: Missing argument $map.\
         \n  ,--> input.scss\
         \n2 | a {b: map.set()}\
         \n  |       ^^^^^^^^^ invocation\
         \n  \'\
         \n  ,\
         \n1 | @function set($map, $args...) {\
         \n  |           =================== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("@use \"sass:map\";\
             \na {b: inspect(map.set($map: (c: d), $key: c, $value: e))}\n"),
        "a {\
         \n  b: (c: e);\
         \n}\n"
    );
}
mod nested {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn empty() {
        assert_eq!(
            runner().ok("@use \"sass:map\";\
             \na {b: inspect(map.set((c: ()), c, d, e, f))}\n"),
            "a {\
         \n  b: (c: (d: (e: f)));\
         \n}\n"
        );
    }
    #[test]
    fn long() {
        assert_eq!(
        runner().ok(
            "@use \"sass:map\";\
             \na {b: inspect(map.set((c: (d: (e: (f: (g: h))))), c, d, e, f, g, i))}\n"
        ),
        "a {\
         \n  b: (c: (d: (e: (f: (g: i)))));\
         \n}\n"
    );
    }
    #[test]
    fn new_key() {
        assert_eq!(
            runner().ok("@use \"sass:map\";\
             \na {b: inspect(map.set((c: (d: e)), c, f, g))}\n"),
            "a {\
         \n  b: (c: (d: e, f: g));\
         \n}\n"
        );
    }
    #[test]
    fn update_existing_key() {
        assert_eq!(
            runner().ok("@use \"sass:map\";\
             \na {b: inspect(map.set((c: (d: e)), c, d, f))}\n"),
            "a {\
         \n  b: (c: (d: f));\
         \n}\n"
        );
    }
    #[test]
    fn value_is_not_a_map() {
        assert_eq!(
            runner().ok("@use \"sass:map\";\
             \na {b: inspect(map.set((c: 1), c, d, f))}\n"),
            "a {\
         \n  b: (c: (d: f));\
         \n}\n"
        );
    }
}
#[test]
fn new_key() {
    assert_eq!(
        runner().ok("@use \"sass:map\";\
             \na {b: inspect(map.set((c: d), e, f))}\n"),
        "a {\
         \n  b: (c: d, e: f);\
         \n}\n"
    );
}
#[test]
fn update_existing_key() {
    assert_eq!(
        runner().ok("@use \"sass:map\";\
             \na {b: inspect(map.set((c: d), c, e))}\n"),
        "a {\
         \n  b: (c: e);\
         \n}\n"
    );
}
