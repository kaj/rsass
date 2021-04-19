//! Tests auto-converted from "sass-spec/spec/core_functions/map/keys.hrx"

#[test]
fn empty() {
    assert_eq!(
        crate::rsass(
            "$result: map-keys(());\
            \na {\
            \n  value: inspect($result);\
            \n  separator: list-separator($result);\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  value: ();\
        \n  separator: comma;\
        \n}\
        \n"
    );
}
mod error {
    #[test]
    fn too_few_args() {
        assert_eq!(
            crate::rsass(
                "a {b: map-keys()}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing argument $map.\
         \n  ,--> input.scss\
         \n1 | a {b: map-keys()}\
         \n  |       ^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:map\
         \n1 | @function keys($map) {\
         \n  |           ========== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            crate::rsass(
                "a {b: map-keys((c: d), (e: f))}\
             \n\
             \n"
            )
            .unwrap_err(),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: map-keys((c: d), (e: f))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:map\
         \n1 | @function keys($map) {\
         \n  |           ========== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            crate::rsass(
                "a {b: map-keys(1)}\
             \n"
            )
            .unwrap_err(),
            "Error: $map: 1 is not a map.\
         \n  ,\
         \n1 | a {b: map-keys(1)}\
         \n  |       ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
}
#[test]
fn multiple() {
    assert_eq!(
        crate::rsass(
            "a {b: map-keys((c: d, e: f, g: h))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: c, e, g;\
        \n}\
        \n"
    );
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: map-keys($map: (1: 2, 3: 4))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1, 3;\
        \n}\
        \n"
    );
}
#[test]
fn single() {
    assert_eq!(
        crate::rsass(
            "$result: map-keys((1: 2));\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n  separator: list-separator($result);\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  value: 1;\
        \n  type: list;\
        \n  separator: comma;\
        \n}\
        \n"
    );
}
