//! Tests auto-converted from "sass-spec/spec/core_functions/map/set.hrx"

#[test]
fn empty() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:map\";\
            \na {b: inspect(map.set((), c, d))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: (c: d);\
        \n}\
        \n"
    );
}
mod error {

    // Ignoring "one_arg", error tests are not supported yet.

    // Ignoring "two_args", error tests are not supported yet.

    // Ignoring "test_type", error tests are not supported yet.

    // Ignoring "zero_args", error tests are not supported yet.
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:map\";\
            \na {b: inspect(map.set($map: (c: d), $key: c, $value: e))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: (c: e);\
        \n}\
        \n"
    );
}
mod nested {
    #[test]
    fn empty() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:map\";\
            \na {b: inspect(map.set((c: ()), c, d, e, f))}\
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
    fn long() {
        assert_eq!(
        crate::rsass(
            "@use \"sass:map\";\
            \na {b: inspect(map.set((c: (d: (e: (f: (g: h))))), c, d, e, f, g, i))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: (c: (d: (e: (f: (g: i)))));\
        \n}\
        \n"
    );
    }
    #[test]
    fn new_key() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:map\";\
            \na {b: inspect(map.set((c: (d: e)), c, f, g))}\
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
    fn update_existing_key() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:map\";\
            \na {b: inspect(map.set((c: (d: e)), c, d, f))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: (c: (d: f));\
        \n}\
        \n"
        );
    }
    #[test]
    fn value_is_not_a_map() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:map\";\
            \na {b: inspect(map.set((c: 1), c, d, f))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: (c: (d: f));\
        \n}\
        \n"
        );
    }
}
#[test]
fn new_key() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:map\";\
            \na {b: inspect(map.set((c: d), e, f))}\
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
fn update_existing_key() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:map\";\
            \na {b: inspect(map.set((c: d), c, e))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: (c: e);\
        \n}\
        \n"
    );
}
