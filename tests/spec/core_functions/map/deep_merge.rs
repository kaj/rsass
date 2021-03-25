//! Tests auto-converted from "sass-spec/spec/core_functions/map/deep_merge.hrx"

mod deep {
    #[test]
    fn different_keys() {
        assert_eq!(
        crate::rsass(
            "@use \'sass:map\';\
            \na {b: inspect(map.deep-merge((c: (d: e, f: g)), (c: (1: 2, 3: 4))))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: (c: (1: 2, 3: 4, d: e, f: g));\
        \n}\
        \n"
    );
    }
    mod empty {
        #[test]
        fn first() {
            assert_eq!(
                crate::rsass(
                    "@use \'sass:map\';\
            \na {b: inspect(map.deep-merge((c: ()), (c: (d: e))))}\
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
        fn second() {
            assert_eq!(
                crate::rsass(
                    "@use \'sass:map\';\
            \na {b: inspect(map.deep-merge((c: (d: e)), (c: ())))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: (c: (d: e));\
        \n}\
        \n"
            );
        }
    }
    #[test]
    fn multiple_layers() {
        assert_eq!(
        crate::rsass(
            "@use \'sass:map\';\
            \na {b: inspect(map.deep-merge((c: (d: (e: (f: g)))), (c: (d: (e: (1: 2))))))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: (c: (d: (e: (1: 2, f: g))));\
        \n}\
        \n"
    );
    }
    #[test]
    fn overlapping_keys() {
        assert_eq!(
        crate::rsass(
            "@use \'sass:map\';\
            \na {b: inspect(map.deep-merge((c: (d: e, f: g, h: i)), (c: (j: 1, f: 2, k: 3))))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: (c: (j: 1, f: 2, k: 3, d: e, h: i));\
        \n}\
        \n"
    );
    }
    #[test]
    fn same_keys() {
        assert_eq!(
        crate::rsass(
            "@use \'sass:map\';\
            \na {b: inspect(map.deep-merge((c: (d: e, f: g)), (c: (d: 1, f: 2))))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: (c: (d: 1, f: 2));\
        \n}\
        \n"
    );
    }
}
mod error {

    // Ignoring "too_few_args", error tests are not supported yet.

    // Ignoring "too_many_args", error tests are not supported yet.
    mod test_type {

        // Ignoring "map1", error tests are not supported yet.

        // Ignoring "map2", error tests are not supported yet.
    }
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "@use \'sass:map\';\
            \na {b: inspect(map.deep-merge($map1: (c: d), $map2: (1: 2)))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: (1: 2, c: d);\
        \n}\
        \n"
    );
}
mod shallow {
    #[test]
    fn different_keys() {
        assert_eq!(
            crate::rsass(
                "@use \'sass:map\';\
            \na {b: inspect(map.deep-merge((c: d, e: f), (1: 2, 3: 4)))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: (1: 2, 3: 4, c: d, e: f);\
        \n}\
        \n"
        );
    }
    mod empty {
        #[test]
        fn both() {
            assert_eq!(
                crate::rsass(
                    "@use \'sass:map\';\
            \na {b: inspect(map.deep-merge((), ()))}\
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
        fn first() {
            assert_eq!(
                crate::rsass(
                    "@use \'sass:map\';\
            \na {b: inspect(map.deep-merge((), (c: d, e: f)))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: (c: d, e: f);\
        \n}\
        \n"
            );
        }
        #[test]
        fn second() {
            assert_eq!(
                crate::rsass(
                    "@use \'sass:map\';\
            \na {b: inspect(map.deep-merge((c: d, e: f), ()))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: (c: d, e: f);\
        \n}\
        \n"
            );
        }
    }
    #[test]
    fn overlapping_keys() {
        assert_eq!(
        crate::rsass(
            "@use \'sass:map\';\
            \na {b: inspect(map.deep-merge((c: d, e: f, g: h), (i: 1, e: 2, j: 3)))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: (i: 1, e: 2, j: 3, c: d, g: h);\
        \n}\
        \n"
    );
    }
    #[test]
    fn same_keys() {
        assert_eq!(
            crate::rsass(
                "@use \'sass:map\';\
            \na {b: inspect(map.deep-merge((c: d, e: f), (c: 1, e: 2)))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: (c: 1, e: 2);\
        \n}\
        \n"
        );
    }
}
