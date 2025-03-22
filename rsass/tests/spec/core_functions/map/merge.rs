//! Tests auto-converted from "sass-spec/spec/core_functions/map/merge.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("merge")
}

#[test]
fn different_keys() {
    assert_eq!(
        runner().ok("@use \"sass:map\";\
             \n@use \"sass:meta\";\
             \na {b: meta.inspect(map.merge((c: d, e: f), (1: 2, 3: 4)))}\n"),
        "a {\
         \n  b: (c: d, e: f, 1: 2, 3: 4);\
         \n}\n"
    );
}
mod empty {
    use super::runner;

    #[test]
    fn both() {
        assert_eq!(
            runner().ok("@use \"sass:map\";\
             \n@use \"sass:meta\";\
             \na {b: meta.inspect(map.merge((), ()))}\n"),
            "a {\
         \n  b: ();\
         \n}\n"
        );
    }
    #[test]
    fn first() {
        assert_eq!(
            runner().ok("@use \"sass:map\";\
             \n@use \"sass:meta\";\
             \na {b: meta.inspect(map.merge((), (c: d, e: f)))}\n"),
            "a {\
         \n  b: (c: d, e: f);\
         \n}\n"
        );
    }
    #[test]
    fn second() {
        assert_eq!(
            runner().ok("@use \"sass:map\";\
             \n@use \"sass:meta\";\
             \na {b: meta.inspect(map.merge((c: d, e: f), ()))}\n"),
            "a {\
         \n  b: (c: d, e: f);\
         \n}\n"
        );
    }
}
mod error {
    use super::runner;

    #[test]
    fn one_arg() {
        assert_eq!(
            runner().err(
                "@use \"sass:map\";\
             \na {b: map.merge((c: d))}\n"
            ),
            "Error: Expected $args to contain a key.\
         \n  ,\
         \n2 | a {b: map.merge((c: d))}\
         \n  |       ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    mod test_type {
        use super::runner;

        #[test]
        fn map1() {
            assert_eq!(
                runner().err(
                    "@use \"sass:map\";\
             \na {b: map.merge(1, (c: d))}\n"
                ),
                "Error: $map1: 1 is not a map.\
         \n  ,\
         \n2 | a {b: map.merge(1, (c: d))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        fn map2() {
            assert_eq!(
                runner().err(
                    "@use \"sass:map\";\
             \na {b: map.merge((c: d), 1)}\n"
                ),
                "Error: $map2: 1 is not a map.\
         \n  ,\
         \n2 | a {b: map.merge((c: d), 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        mod nested {
            use super::runner;

            #[test]
            fn map1() {
                assert_eq!(
                    runner().err(
                        "@use \"sass:map\";\
             \na {b: map.merge(1, c, (d: e))}\n"
                    ),
                    "Error: $map1: 1 is not a map.\
         \n  ,\
         \n2 | a {b: map.merge(1, c, (d: e))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
                );
            }
            #[test]
            fn map2() {
                assert_eq!(
                    runner().err(
                        "@use \"sass:map\";\
             \na {b: map.merge((c: d), e, 1)}\n"
                    ),
                    "Error: $map2: 1 is not a map.\
         \n  ,\
         \n2 | a {b: map.merge((c: d), e, 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
                );
            }
        }
    }
    #[test]
    fn wrong_name() {
        assert_eq!(
            runner().err(
                "@use \"sass:map\";\
             \na {b: map.map-merge((c: d), (1: 2))}\n"
            ),
            "Error: Undefined function.\
         \n  ,\
         \n2 | a {b: map.map-merge((c: d), (1: 2))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn zero_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:map\";\
             \na {b: map.merge()}\n"
            ),
            "Error: Missing argument $map1.\
         \n  ,--> input.scss\
         \n2 | a {b: map.merge()}\
         \n  |       ^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,\
         \n1 | @function merge($map1, $args...) {\
         \n  |           ====================== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        runner().ok(
            "@use \"sass:map\";\
             \n@use \"sass:meta\";\
             \na {b: meta.inspect(map.merge($map1: (c: d), $map2: (1: 2)))}\n"
        ),
        "a {\
         \n  b: (c: d, 1: 2);\
         \n}\n"
    );
}
mod nested {
    use super::runner;

    #[test]
    fn different_keys() {
        assert_eq!(
        runner().ok(
            "@use \"sass:map\";\
             \n@use \"sass:meta\";\
             \na {b: meta.inspect(map.merge((c: (d: e, f: g)), c, (1: 2, 3: 4)))}\n"
        ),
        "a {\
         \n  b: (c: (d: e, f: g, 1: 2, 3: 4));\
         \n}\n"
    );
    }
    mod empty {
        use super::runner;

        #[test]
        fn both() {
            assert_eq!(
                runner().ok("@use \"sass:map\";\
             \n@use \"sass:meta\";\
             \na {b: meta.inspect(map.merge((c: ()), c, ()))}\n"),
                "a {\
         \n  b: (c: ());\
         \n}\n"
            );
        }
        #[test]
        fn first() {
            assert_eq!(
                runner().ok("@use \"sass:map\";\
             \n@use \"sass:meta\";\
             \na {b: meta.inspect(map.merge((c: ()), c, (d: e, f: g)))}\n"),
                "a {\
         \n  b: (c: (d: e, f: g));\
         \n}\n"
            );
        }
        #[test]
        fn second() {
            assert_eq!(
                runner().ok("@use \"sass:map\";\
             \n@use \"sass:meta\";\
             \na {b: meta.inspect(map.merge((c: (d: e, f: g)), c, ()))}\n"),
                "a {\
         \n  b: (c: (d: e, f: g));\
         \n}\n"
            );
        }
    }
    #[test]
    fn intermediate_value_is_not_a_map() {
        assert_eq!(
            runner().ok("@use \"sass:map\";\
             \n@use \"sass:meta\";\
             \na {b: meta.inspect(map.merge((c: 1), c, d, (e: f)))}\n"),
            "a {\
         \n  b: (c: (d: (e: f)));\
         \n}\n"
        );
    }
    #[test]
    fn leaf_value_is_not_a_map() {
        assert_eq!(
            runner().ok("@use \"sass:map\";\
             \n@use \"sass:meta\";\
             \na {b: meta.inspect(map.merge((c: 1), c, (d: e)))}\n"),
            "a {\
         \n  b: (c: (d: e));\
         \n}\n"
        );
    }
    #[test]
    fn multiple_keys() {
        assert_eq!(
        runner().ok(
            "@use \"sass:map\";\
             \n@use \"sass:meta\";\
             \na {b: meta.inspect(map.merge((c: (d: (e: (f: (g: h))))), c, d, e, f, (g: 1)))}\n"
        ),
        "a {\
         \n  b: (c: (d: (e: (f: (g: 1)))));\
         \n}\n"
    );
    }
    #[test]
    fn overlapping_keys() {
        assert_eq!(
        runner().ok(
            "@use \"sass:map\";\
             \n@use \"sass:meta\";\
             \na {b: meta.inspect(map.merge((c: (d: e, f: g, h: i)), c, (j: 1, f: 2, k: 3)))}\n"
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
            "@use \"sass:map\";\
             \n@use \"sass:meta\";\
             \na {b: meta.inspect(map.merge((c: (d: e, f: g)), c, (d: 1, f: 2)))}\n"
        ),
        "a {\
         \n  b: (c: (d: 1, f: 2));\
         \n}\n"
    );
    }
}
#[test]
fn overlapping_keys() {
    assert_eq!(
        runner().ok(
            "@use \"sass:map\";\
             \n@use \"sass:meta\";\
             \na {b: meta.inspect(map.merge((c: d, e: f, g: h), (i: 1, e: 2, j: 3)))}\n"
        ),
        "a {\
         \n  b: (c: d, e: 2, g: h, i: 1, j: 3);\
         \n}\n"
    );
}
#[test]
fn same_keys() {
    assert_eq!(
        runner().ok("@use \"sass:map\";\
             \n@use \"sass:meta\";\
             \na {b: meta.inspect(map.merge((c: d, e: f), (c: 1, e: 2)))}\n"),
        "a {\
         \n  b: (c: 1, e: 2);\
         \n}\n"
    );
}
