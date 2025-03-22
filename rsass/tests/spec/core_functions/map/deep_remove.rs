//! Tests auto-converted from "sass-spec/spec/core_functions/map/deep_remove.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("deep_remove")
}

mod error {
    use super::runner;

    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err(
                "@use \'sass:map\';\
             \na {b: map.deep-remove((c: d))}\n"
            ),
            "Error: Missing argument $key.\
         \n  ,--> input.scss\
         \n2 | a {b: map.deep-remove((c: d))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:map\
         \n1 | @function deep-remove($map, $key, $keys...) {\
         \n  |           ================================= declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            runner().err(
                "@use \'sass:map\';\
             \na {b: map.deep-remove(1, 2)}\n"
            ),
            "Error: $map: 1 is not a map.\
         \n  ,\
         \n2 | a {b: map.deep-remove(1, 2)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
mod found {
    use super::runner;

    mod nested {
        use super::runner;

        #[test]
        fn first() {
            assert_eq!(
        runner().ok(
            "@use \"sass:meta\";\
             \n@use \'sass:map\';\
             \na {b: meta.inspect(map.deep-remove((c: (d: e, f: g, h: i)), c, d))}\n"
        ),
        "a {\
         \n  b: (c: (f: g, h: i));\
         \n}\n"
    );
        }
        #[test]
        fn last() {
            assert_eq!(
        runner().ok(
            "@use \"sass:meta\";\
             \n@use \'sass:map\';\
             \na {b: meta.inspect(map.deep-remove((c: (d: e, f: g, h: i)), c, h))}\n"
        ),
        "a {\
         \n  b: (c: (d: e, f: g));\
         \n}\n"
    );
        }
        #[test]
        fn middle() {
            assert_eq!(
        runner().ok(
            "@use \"sass:meta\";\
             \n@use \'sass:map\';\
             \na {b: meta.inspect(map.deep-remove((c: (d: e, f: g, h: i)), c, f))}\n"
        ),
        "a {\
         \n  b: (c: (d: e, h: i));\
         \n}\n"
    );
        }
        #[test]
        fn single() {
            assert_eq!(
                runner().ok("@use \"sass:meta\";\
             \n@use \'sass:map\';\
             \na {b: meta.inspect(map.deep-remove((c: (d: e)), c, d))}\n"),
                "a {\
         \n  b: (c: ());\
         \n}\n"
            );
        }
    }
    #[test]
    fn top_level() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\
             \n@use \'sass:map\';\
             \na {b: meta.inspect(map.deep-remove((c: d), c))}\n"),
            "a {\
         \n  b: ();\
         \n}\n"
        );
    }
}
mod not_found {
    use super::runner;

    #[test]
    fn empty() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\
             \n@use \'sass:map\';\
             \na {b: meta.inspect(map.deep-remove((), 1))}\n"),
            "a {\
         \n  b: ();\
         \n}\n"
        );
    }
    #[test]
    fn extra_keys() {
        assert_eq!(
        runner().ok(
            "@use \"sass:meta\";\
             \n@use \'sass:map\';\
             \na {b: meta.inspect(map.deep-remove((c: (d: e)), c, d, e, f, g))}\n"
        ),
        "a {\
         \n  b: (c: (d: e));\
         \n}\n"
    );
    }
    #[test]
    fn nested() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\
             \n@use \'sass:map\';\
             \na {b: meta.inspect(map.deep-remove((c: (d: e)), c, e))}\n"),
            "a {\
         \n  b: (c: (d: e));\
         \n}\n"
        );
    }
    #[test]
    fn not_a_map() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\
             \n@use \'sass:map\';\
             \na {b: meta.inspect(map.deep-remove((c: (d: e)), c, d, e))}\n"),
            "a {\
         \n  b: (c: (d: e));\
         \n}\n"
        );
    }
    #[test]
    fn top_level() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\
             \n@use \'sass:map\';\
             \na {b: meta.inspect(map.deep-remove((c: d), d))}\n"),
            "a {\
         \n  b: (c: d);\
         \n}\n"
        );
    }
}
