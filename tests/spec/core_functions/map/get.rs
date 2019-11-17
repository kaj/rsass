//! Tests auto-converted from "sass-spec/spec/core_functions/map/get.hrx"

mod error {
    #[test]
    fn too_few_args() {
        assert_eq!(
            crate::rsass(
                "a {b: map-get(1)}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing argument $key.\
         \n  ,--> input.scss\
         \n1 | a {b: map-get(1)}\
         \n  |       ^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:map\
         \n1 | @function get($map, $key, $keys...) {\
         \n  |           ========================= declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    mod test_type {
        #[test]
        fn map() {
            assert_eq!(
                crate::rsass(
                    "a {b: map-get(1, 2)}\
             \n"
                )
                .unwrap_err(),
                "Error: $map: 1 is not a map.\
         \n  ,\
         \n1 | a {b: map-get(1, 2)}\
         \n  |       ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
            );
        }
    }
}
mod found {
    #[test]
    fn first() {
        assert_eq!(
            crate::rsass(
                "a {b: map-get((1: 2, 3: 4, 5: 6), 1)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 2;\
        \n}\
        \n"
        );
    }
    #[test]
    fn last() {
        assert_eq!(
            crate::rsass(
                "a {b: map-get((1: 2, 3: 4, 5: 6), 5)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 6;\
        \n}\
        \n"
        );
    }
    #[test]
    fn middle() {
        assert_eq!(
            crate::rsass(
                "a {b: map-get((1: 2, 3: 4, 5: 6), 3)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 4;\
        \n}\
        \n"
        );
    }
    #[test]
    fn single() {
        assert_eq!(
            crate::rsass(
                "a {b: map-get((c: d), c)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: d;\
        \n}\
        \n"
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: map-get($map: (c: d), $key: c)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: d;\
        \n}\
        \n"
    );
}
mod nested {
    mod found {
        #[test]
        fn full_path() {
            assert_eq!(
                crate::rsass(
                    "a {b: map-get((c: (d: (e: f))), c, d, e)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: f;\
        \n}\
        \n"
            );
        }
        #[test]
        fn partial_path() {
            assert_eq!(
                crate::rsass(
                    "a {b: inspect(map-get((c: (d: (e: f))), c, d))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: (e: f);\
        \n}\
        \n"
            );
        }
    }
    mod not_found {
        #[test]
        fn deep() {
            assert_eq!(
                crate::rsass(
                    "a {b: inspect(map-get((c: (d: (e: f))), c, d, g))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: null;\
        \n}\
        \n"
            );
        }
        #[test]
        fn too_many_keys() {
            assert_eq!(
                crate::rsass(
                    "a {b: inspect(map-get((c: (d: (e: f))), c, d, e, f))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: null;\
        \n}\
        \n"
            );
        }
        #[test]
        fn top_level() {
            assert_eq!(
                crate::rsass(
                    "a {b: inspect(map-get((c: (d: (e: f))), d))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: null;\
        \n}\
        \n"
            );
        }
    }
}
mod not_found {
    #[test]
    fn dash_sensitive() {
        assert_eq!(
            crate::rsass(
                "a {b: inspect(map-get((c-d: e), c_d))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: null;\
        \n}\
        \n"
        );
    }
    #[test]
    fn empty() {
        assert_eq!(
            crate::rsass(
                "a {b: inspect(map-get((), 1))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: null;\
        \n}\
        \n"
        );
    }
    #[test]
    fn non_empty() {
        assert_eq!(
            crate::rsass(
                "a {b: inspect(map-get((c: d), d))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: null;\
        \n}\
        \n"
        );
    }
}
