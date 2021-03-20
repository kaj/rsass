//! Tests auto-converted from "sass-spec/spec/core_functions/modules/map.hrx"

mod error {

    // Ignoring "map_get", error tests are not supported yet.

    // Ignoring "map_has_key", error tests are not supported yet.

    // Ignoring "map_keys", error tests are not supported yet.

    // Ignoring "map_merge", error tests are not supported yet.

    // Ignoring "map_remove", error tests are not supported yet.

    // Ignoring "map_values", error tests are not supported yet.
}
#[test]
fn get() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:map\";\
            \na {b: map.get((c: d), c)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: d;\
        \n}\
        \n"
    );
}
#[test]
fn has_key() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:map\";\
            \na {b: map.has-key((c: d), c)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: true;\
        \n}\
        \n"
    );
}
#[test]
fn keys() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:map\";\
            \na {b: map.keys((c: d))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: c;\
        \n}\
        \n"
    );
}
#[test]
fn merge() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:map\";\
            \n@use \"sass:meta\";\
            \na {b: meta.inspect(map.merge((c: d), (e: f)))}\
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
fn remove() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:map\";\
            \n@use \"sass:meta\";\
            \na {b: meta.inspect(map.remove((c: d), c))}\
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
fn values() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:map\";\
            \na {b: map.values((c: d))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: d;\
        \n}\
        \n"
    );
}
