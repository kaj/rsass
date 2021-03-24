//! Tests auto-converted from "sass-spec/spec/core_functions/map/merge.hrx"

#[test]
fn different_keys() {
    assert_eq!(
        crate::rsass(
            "a {b: inspect(map-merge((c: d, e: f), (1: 2, 3: 4)))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: (c: d, e: f, 1: 2, 3: 4);\
        \n}\
        \n"
    );
}
mod empty {
    #[test]
    fn both() {
        assert_eq!(
            crate::rsass(
                "a {b: inspect(map-merge((), ()))}\
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
                "a {b: inspect(map-merge((), (c: d, e: f)))}\
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
                "a {b: inspect(map-merge((c: d, e: f), ()))}\
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
mod error {

    // Ignoring "one_arg", error tests are not supported yet.
    mod test_type {

        // Ignoring "map1", error tests are not supported yet.

        // Ignoring "map2", error tests are not supported yet.
        mod nested {

            // Ignoring "map1", error tests are not supported yet.

            // Ignoring "map2", error tests are not supported yet.
        }
    }

    // Ignoring "zero_args", error tests are not supported yet.
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: inspect(map-merge($map1: (c: d), $map2: (1: 2)))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: (c: d, 1: 2);\
        \n}\
        \n"
    );
}
mod nested {
    #[test]
    fn different_keys() {
        assert_eq!(
        crate::rsass(
            "a {b: inspect(map-merge((c: (d: e, f: g)), c, (1: 2, 3: 4)))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: (c: (d: e, f: g, 1: 2, 3: 4));\
        \n}\
        \n"
    );
    }
    mod empty {
        #[test]
        fn both() {
            assert_eq!(
                crate::rsass(
                    "a {b: inspect(map-merge((c: ()), c, ()))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: (c: ());\
        \n}\
        \n"
            );
        }
        #[test]
        fn first() {
            assert_eq!(
                crate::rsass(
                    "a {b: inspect(map-merge((c: ()), c, (d: e, f: g)))}\
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
        fn second() {
            assert_eq!(
                crate::rsass(
                    "a {b: inspect(map-merge((c: (d: e, f: g)), c, ()))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: (c: (d: e, f: g));\
        \n}\
        \n"
            );
        }
    }
    #[test]
    fn intermediate_value_is_not_a_map() {
        assert_eq!(
            crate::rsass(
                "a {b: inspect(map-merge((c: 1), c, d, (e: f)))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: (c: (d: (e: f)));\
        \n}\
        \n"
        );
    }
    #[test]
    fn leaf_value_is_not_a_map() {
        assert_eq!(
            crate::rsass(
                "a {b: inspect(map-merge((c: 1), c, (d: e)))}\
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
    fn multiple_keys() {
        assert_eq!(
        crate::rsass(
            "a {b: inspect(map-merge((c: (d: (e: (f: (g: h))))), c, d, e, f, (g: 1)))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: (c: (d: (e: (f: (g: 1)))));\
        \n}\
        \n"
    );
    }
    #[test]
    fn overlapping_keys() {
        assert_eq!(
        crate::rsass(
            "a {b: inspect(map-merge((c: (d: e, f: g, h: i)), c, (j: 1, f: 2, k: 3)))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: (c: (d: e, f: 2, h: i, j: 1, k: 3));\
        \n}\
        \n"
    );
    }
    #[test]
    fn same_keys() {
        assert_eq!(
        crate::rsass(
            "a {b: inspect(map-merge((c: (d: e, f: g)), c, (d: 1, f: 2)))}\
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
#[test]
fn overlapping_keys() {
    assert_eq!(
        crate::rsass(
            "a {b: inspect(map-merge((c: d, e: f, g: h), (i: 1, e: 2, j: 3)))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: (c: d, e: 2, g: h, i: 1, j: 3);\
        \n}\
        \n"
    );
}
#[test]
fn same_keys() {
    assert_eq!(
        crate::rsass(
            "a {b: inspect(map-merge((c: d, e: f), (c: 1, e: 2)))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: (c: 1, e: 2);\
        \n}\
        \n"
    );
}
