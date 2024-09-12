//! Tests auto-converted from "sass-spec/spec/core_functions/map/keys.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("keys")
}

#[test]
fn empty() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \n@use \"sass:map\";\
             \n@use \"sass:meta\";\
             \n$result: map.keys(());\
             \na {\
             \n  value: meta.inspect($result);\
             \n  separator: list.separator($result);\
             \n}\n"),
        "a {\
         \n  value: ();\
         \n  separator: comma;\
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
                "@use \"sass:map\";\
             \na {b: map.keys()}\n"
            ),
            "Error: Missing argument $map.\
         \n  ,--> input.scss\
         \n2 | a {b: map.keys()}\
         \n  |       ^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:map\
         \n1 | @function keys($map) {\
         \n  |           ========== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:map\";\
             \na {b: map.keys((c: d), (e: f))}\n\n"
            ),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: map.keys((c: d), (e: f))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:map\
         \n1 | @function keys($map) {\
         \n  |           ========== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            runner().err(
                "@use \"sass:map\";\
             \na {b: map.keys(1)}\n"
            ),
            "Error: $map: 1 is not a map.\
         \n  ,\
         \n2 | a {b: map.keys(1)}\
         \n  |       ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn wrong_name() {
        assert_eq!(
            runner().err(
                "@use \"sass:map\";\
             \na {b: map.map-keys((c: d))}\n"
            ),
            "Error: Undefined function.\
         \n  ,\
         \n2 | a {b: map.map-keys((c: d))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
#[test]
fn multiple() {
    assert_eq!(
        runner().ok("@use \"sass:map\";\
             \na {b: map.keys((c: d, e: f, g: h))}\n"),
        "a {\
         \n  b: c, e, g;\
         \n}\n"
    );
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("@use \"sass:map\";\
             \na {b: map.keys($map: (1: 2, 3: 4))}\n"),
        "a {\
         \n  b: 1, 3;\
         \n}\n"
    );
}
#[test]
fn single() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \n@use \"sass:map\";\
             \n@use \"sass:meta\";\
             \n$result: map.keys((1: 2));\
             \na {\
             \n  value: $result;\
             \n  type: meta.type-of($result);\
             \n  separator: list.separator($result);\
             \n}\n"),
        "a {\
         \n  value: 1;\
         \n  type: list;\
         \n  separator: comma;\
         \n}\n"
    );
}
