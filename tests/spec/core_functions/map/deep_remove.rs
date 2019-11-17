//! Tests auto-converted from "sass-spec/spec/core_functions/map/deep_remove.hrx"

mod error {
    #[test]
    fn too_few_args() {
        assert_eq!(
            crate::rsass(
                "@use \'sass:map\';\
             \na {b: map.deep-remove((c: d))}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing argument $key.\
         \n  ,--> input.scss\
         \n2 | a {b: map.deep-remove((c: d))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:map\
         \n1 | @function deep-remove($map, $key, $keys...) {\
         \n  |           ================================= declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            crate::rsass(
                "@use \'sass:map\';\
             \na {b: map.deep-remove(1, 2)}\
             \n"
            )
            .unwrap_err(),
            "Error: $map: 1 is not a map.\
         \n  ,\
         \n2 | a {b: map.deep-remove(1, 2)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
        );
    }
}
mod found {
    mod nested {
        #[test]
        fn first() {
            assert_eq!(
                crate::rsass(
                    "@use \'sass:map\';\
            \na {b: inspect(map.deep-remove((c: (d: e, f: g, h: i)), c, d))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: (c: (f: g, h: i));\
        \n}\
        \n"
            );
        }
        #[test]
        fn last() {
            assert_eq!(
                crate::rsass(
                    "@use \'sass:map\';\
            \na {b: inspect(map.deep-remove((c: (d: e, f: g, h: i)), c, h))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: (c: (d: e, f: g));\
        \n}\
        \n"
            );
        }
        #[test]
        fn middle() {
            assert_eq!(
                crate::rsass(
                    "@use \'sass:map\';\
            \na {b: inspect(map.deep-remove((c: (d: e, f: g, h: i)), c, f))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: (c: (d: e, h: i));\
        \n}\
        \n"
            );
        }
        #[test]
        fn single() {
            assert_eq!(
                crate::rsass(
                    "@use \'sass:map\';\
            \na {b: inspect(map.deep-remove((c: (d: e)), c, d))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: (c: ());\
        \n}\
        \n"
            );
        }
    }
    #[test]
    fn top_level() {
        assert_eq!(
            crate::rsass(
                "@use \'sass:map\';\
            \na {b: inspect(map.deep-remove((c: d), c))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: ();\
        \n}\
        \n"
        );
    }
}
mod not_found {
    #[test]
    fn empty() {
        assert_eq!(
            crate::rsass(
                "@use \'sass:map\';\
            \na {b: inspect(map.deep-remove((), 1))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: ();\
        \n}\
        \n"
        );
    }
    #[test]
    fn extra_keys() {
        assert_eq!(
            crate::rsass(
                "@use \'sass:map\';\
            \na {b: inspect(map.deep-remove((c: (d: e)), c, d, e, f, g))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: (c: (d: e));\
        \n}\
        \n"
        );
    }
    #[test]
    fn nested() {
        assert_eq!(
            crate::rsass(
                "@use \'sass:map\';\
            \na {b: inspect(map.deep-remove((c: (d: e)), c, e))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: (c: (d: e));\
        \n}\
        \n"
        );
    }
    #[test]
    fn not_a_map() {
        assert_eq!(
            crate::rsass(
                "@use \'sass:map\';\
            \na {b: inspect(map.deep-remove((c: (d: e)), c, d, e))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: (c: (d: e));\
        \n}\
        \n"
        );
    }
    #[test]
    fn top_level() {
        assert_eq!(
            crate::rsass(
                "@use \'sass:map\';\
            \na {b: inspect(map.deep-remove((c: d), d))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: (c: d);\
        \n}\
        \n"
        );
    }
}
