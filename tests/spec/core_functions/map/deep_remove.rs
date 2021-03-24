//! Tests auto-converted from "sass-spec/spec/core_functions/map/deep_remove.hrx"

mod error {

    // Ignoring "too_few_args", error tests are not supported yet.

    // Ignoring "test_type", error tests are not supported yet.
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
