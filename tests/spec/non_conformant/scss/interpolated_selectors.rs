//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/interpolated-selectors.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo#{bar} hux {\
            \n  color: red;\
            \n}"
        )
        .unwrap(),
        "foobar hux {\
        \n  color: red;\
        \n}\
        \n"
    );
}
