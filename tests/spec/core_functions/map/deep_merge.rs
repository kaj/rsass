//! Tests auto-converted from "sass-spec/spec/core_functions/map/deep_merge.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("deep_merge")
}

mod deep {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn different_keys() {
        assert_eq!(
        runner().ok(
            "@use \'sass:map\';\
             \na {b: inspect(map.deep-merge((c: (d: e, f: g)), (c: (1: 2, 3: 4))))}\n"
        ),
        "a {\
         \n  b: (c: (d: e, f: g, 1: 2, 3: 4));\
         \n}\n"
    );
    }
    mod empty {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn first() {
            assert_eq!(
                runner().ok("@use \'sass:map\';\
             \na {b: inspect(map.deep-merge((c: ()), (c: (d: e))))}\n"),
                "a {\
         \n  b: (c: (d: e));\
         \n}\n"
            );
        }
        #[test]
        fn second() {
            assert_eq!(
                runner().ok("@use \'sass:map\';\
             \na {b: inspect(map.deep-merge((c: (d: e)), (c: ())))}\n"),
                "a {\
         \n  b: (c: (d: e));\
         \n}\n"
            );
        }
    }
    #[test]
    fn multiple_layers() {
        assert_eq!(
        runner().ok(
            "@use \'sass:map\';\
             \na {b: inspect(map.deep-merge((c: (d: (e: (f: g)))), (c: (d: (e: (1: 2))))))}\n"
        ),
        "a {\
         \n  b: (c: (d: (e: (f: g, 1: 2))));\
         \n}\n"
    );
    }
    #[test]
    fn overlapping_keys() {
        assert_eq!(
        runner().ok(
            "@use \'sass:map\';\
             \na {b: inspect(map.deep-merge((c: (d: e, f: g, h: i)), (c: (j: 1, f: 2, k: 3))))}\n"
        ),
        "a {\
         \n  b: (c: (d: e, f: 2, h: i, j: 1, k: 3));\
         \n}\n"
    );
    }
    #[test]
    fn same_keys() {
        assert_eq!(
        runner().ok(
            "@use \'sass:map\';\
             \na {b: inspect(map.deep-merge((c: (d: e, f: g)), (c: (d: 1, f: 2))))}\n"
        ),
        "a {\
         \n  b: (c: (d: 1, f: 2));\
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
            runner().err(
                "@use \'sass:map\';\
             \na {b: map.deep-merge((c: d))}\n"
            ),
            "Error: Missing argument $map2.\
         \n  ,--> input.scss\
         \n2 | a {b: map.deep-merge((c: d))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:map\
         \n1 | @function deep-merge($map1, $map2) {\
         \n  |           ======================== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err(
                "@use \'sass:map\';\
             \na {b: map.deep-merge((c: d), (e: f), (g: h))}\n"
            ),
            "Error: Only 2 arguments allowed, but 3 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: map.deep-merge((c: d), (e: f), (g: h))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:map\
         \n1 | @function deep-merge($map1, $map2) {\
         \n  |           ======================== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    mod test_type {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn map1() {
            assert_eq!(
                runner().err(
                    "@use \'sass:map\';\
             \na {b: map.deep-merge(1, (c: d))}\n"
                ),
                "Error: $map1: 1 is not a map.\
         \n  ,\
         \n2 | a {b: map.deep-merge(1, (c: d))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        fn map2() {
            assert_eq!(
                runner().err(
                    "@use \'sass:map\';\
             \na {b: map.deep-merge((c: d), 1)}\n"
                ),
                "Error: $map2: 1 is not a map.\
         \n  ,\
         \n2 | a {b: map.deep-merge((c: d), 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
    }
}
#[test]
fn named() {
    assert_eq!(
        runner().ok(
            "@use \'sass:map\';\
             \na {b: inspect(map.deep-merge($map1: (c: d), $map2: (1: 2)))}\n"
        ),
        "a {\
         \n  b: (c: d, 1: 2);\
         \n}\n"
    );
}
mod shallow {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn different_keys() {
        assert_eq!(
            runner().ok("@use \'sass:map\';\
             \na {b: inspect(map.deep-merge((c: d, e: f), (1: 2, 3: 4)))}\n"),
            "a {\
         \n  b: (c: d, e: f, 1: 2, 3: 4);\
         \n}\n"
        );
    }
    mod empty {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn both() {
            assert_eq!(
                runner().ok("@use \'sass:map\';\
             \na {b: inspect(map.deep-merge((), ()))}\n"),
                "a {\
         \n  b: ();\
         \n}\n"
            );
        }
        #[test]
        fn first() {
            assert_eq!(
                runner().ok("@use \'sass:map\';\
             \na {b: inspect(map.deep-merge((), (c: d, e: f)))}\n"),
                "a {\
         \n  b: (c: d, e: f);\
         \n}\n"
            );
        }
        #[test]
        fn second() {
            assert_eq!(
                runner().ok("@use \'sass:map\';\
             \na {b: inspect(map.deep-merge((c: d, e: f), ()))}\n"),
                "a {\
         \n  b: (c: d, e: f);\
         \n}\n"
            );
        }
    }
    #[test]
    fn overlapping_keys() {
        assert_eq!(
        runner().ok(
            "@use \'sass:map\';\
             \na {b: inspect(map.deep-merge((c: d, e: f, g: h), (i: 1, e: 2, j: 3)))}\n"
        ),
        "a {\
         \n  b: (c: d, e: 2, g: h, i: 1, j: 3);\
         \n}\n"
    );
    }
    #[test]
    fn same_keys() {
        assert_eq!(
            runner().ok("@use \'sass:map\';\
             \na {b: inspect(map.deep-merge((c: d, e: f), (c: 1, e: 2)))}\n"),
            "a {\
         \n  b: (c: 1, e: 2);\
         \n}\n"
        );
    }
}
