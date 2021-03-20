//! Tests auto-converted from "sass-spec/spec/values/maps/key_equality.hrx"

mod infinity {
    #[test]
    fn negative() {
        assert_eq!(
            crate::rsass(
                "a {b: inspect(map-get(((-1/0): b), -1/0))}\
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
    fn positive() {
        assert_eq!(
            crate::rsass(
                "a {b: inspect(map-get(((1/0): b), 1/0))}\
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
#[test]
fn nan() {
    assert_eq!(
        crate::rsass(
            "a {b: inspect(map-get(((0/0): b), 0/0))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: null;\
        \n}\
        \n"
    );
}
