//! Tests auto-converted from "sass-spec/spec/values/maps/length.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$map: (aaa: 100, bbb: 200, ccc: 300);\
            \n\
            \na {\
            \n  b: length($map);\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 3;\
        \n}\
        \n"
    );
}
