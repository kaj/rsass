//! Tests auto-converted from "sass-spec/spec/core_functions/modules/map.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn map_get() {
        assert_eq!(
            runner().err(
                "@use \"sass:map\";\
             \na {b: map.map-get((c: d), c)}\n"
            ),
            "Error: Undefined function.\
         \n  ,\
         \n2 | a {b: map.map-get((c: d), c)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn map_has_key() {
        assert_eq!(
            runner().err(
                "@use \"sass:map\";\
             \na {b: map.map-has-key((c: d), c)}\n"
            ),
            "Error: Undefined function.\
         \n  ,\
         \n2 | a {b: map.map-has-key((c: d), c)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn map_keys() {
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
    #[test]
    fn map_merge() {
        assert_eq!(
            runner().err(
                "@use \"sass:map\";\
             \n@use \"sass:meta\";\
             \na {b: meta.map-inspect(map.merge((c: d), (e: f)))}\n"
            ),
            "Error: Undefined function.\
         \n  ,\
         \n3 | a {b: meta.map-inspect(map.merge((c: d), (e: f)))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:7  root stylesheet",
        );
    }
    #[test]
    fn map_remove() {
        assert_eq!(
            runner().err(
                "@use \"sass:map\";\
             \n@use \"sass:meta\";\
             \na {b: meta.map-inspect(map.remove((c: d), c))}\n"
            ),
            "Error: Undefined function.\
         \n  ,\
         \n3 | a {b: meta.map-inspect(map.remove((c: d), c))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:7  root stylesheet",
        );
    }
    #[test]
    fn map_values() {
        assert_eq!(
            runner().err(
                "@use \"sass:map\";\
             \na {b: map.map-values((c: d), c)}\n"
            ),
            "Error: Undefined function.\
         \n  ,\
         \n2 | a {b: map.map-values((c: d), c)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
#[test]
fn get() {
    assert_eq!(
        runner().ok("@use \"sass:map\";\
             \na {b: map.get((c: d), c)}\n"),
        "a {\
         \n  b: d;\
         \n}\n"
    );
}
#[test]
fn has_key() {
    assert_eq!(
        runner().ok("@use \"sass:map\";\
             \na {b: map.has-key((c: d), c)}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
fn keys() {
    assert_eq!(
        runner().ok("@use \"sass:map\";\
             \na {b: map.keys((c: d))}\n"),
        "a {\
         \n  b: c;\
         \n}\n"
    );
}
#[test]
fn merge() {
    assert_eq!(
        runner().ok("@use \"sass:map\";\
             \n@use \"sass:meta\";\
             \na {b: meta.inspect(map.merge((c: d), (e: f)))}\n"),
        "a {\
         \n  b: (c: d, e: f);\
         \n}\n"
    );
}
#[test]
fn remove() {
    assert_eq!(
        runner().ok("@use \"sass:map\";\
             \n@use \"sass:meta\";\
             \na {b: meta.inspect(map.remove((c: d), c))}\n"),
        "a {\
         \n  b: ();\
         \n}\n"
    );
}
#[test]
fn values() {
    assert_eq!(
        runner().ok("@use \"sass:map\";\
             \na {b: map.values((c: d))}\n"),
        "a {\
         \n  b: d;\
         \n}\n"
    );
}
