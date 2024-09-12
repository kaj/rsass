//! Tests auto-converted from "sass-spec/spec/core_functions/map/has_key.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("has_key")
}

mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:map\";\
             \na {b: map.has-key(1)}\n"
            ),
            "Error: Missing argument $key.\
         \n  ,--> input.scss\
         \n2 | a {b: map.has-key(1)}\
         \n  |       ^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:map\
         \n1 | @function has-key($map, $key, $keys...) {\
         \n  |           ============================= declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    mod test_type {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn map() {
            assert_eq!(
                runner().err(
                    "@use \"sass:map\";\
             \na {b: map.has-key(1, 2)}\n"
                ),
                "Error: $map: 1 is not a map.\
         \n  ,\
         \n2 | a {b: map.has-key(1, 2)}\
         \n  |       ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
    }
    #[test]
    fn wrong_name() {
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
}
mod found {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn first() {
        assert_eq!(
            runner().ok("@use \"sass:map\";\
             \na {b: map.has-key((1: 2, 3: 4, 5: 6), 1)}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
    #[test]
    fn last() {
        assert_eq!(
            runner().ok("@use \"sass:map\";\
             \na {b: map.has-key((1: 2, 3: 4, 5: 6), 5)}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
    #[test]
    fn middle() {
        assert_eq!(
            runner().ok("@use \"sass:map\";\
             \na {b: map.has-key((1: 2, 3: 4, 5: 6), 3)}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
    #[test]
    fn single() {
        assert_eq!(
            runner().ok("@use \"sass:map\";\
             \na {b: map.has-key((c: d), c)}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("@use \"sass:map\";\
             \na {b: map.has-key($map: (c: d), $key: c)}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
mod nested {
    #[allow(unused)]
    use super::runner;

    mod found {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn full_path() {
            assert_eq!(
                runner().ok("@use \"sass:map\";\
             \na {b: map.has-key((c: (d: (e: f))), c, d, e)}\n"),
                "a {\
         \n  b: true;\
         \n}\n"
            );
        }
        #[test]
        fn partial_path() {
            assert_eq!(
                runner().ok("@use \"sass:map\";\
             \na {b: map.has-key((c: (d: (e: f))), c, d)}\n"),
                "a {\
         \n  b: true;\
         \n}\n"
            );
        }
    }
    mod not_found {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn deep() {
            assert_eq!(
                runner().ok("@use \"sass:map\";\
             \na {b: map.has-key((c: (d: (e: f))), c, d, g)}\n"),
                "a {\
         \n  b: false;\
         \n}\n"
            );
        }
        #[test]
        fn too_many_keys() {
            assert_eq!(
                runner().ok("@use \"sass:map\";\
             \na {b: map.has-key((c: (d: (e: f))), c, d, e, f)}\n"),
                "a {\
         \n  b: false;\
         \n}\n"
            );
        }
        #[test]
        fn top_level() {
            assert_eq!(
                runner().ok("@use \"sass:map\";\
             \na {b: map.has-key((c: (d: (e: f))), d)}\n"),
                "a {\
         \n  b: false;\
         \n}\n"
            );
        }
    }
}
mod not_found {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn empty() {
        assert_eq!(
            runner().ok("@use \"sass:map\";\
             \na {b: map.has-key((), 1)}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
    #[test]
    fn non_empty() {
        assert_eq!(
            runner().ok("@use \"sass:map\";\
             \na {b: map.has-key((c: d), d)}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
}
