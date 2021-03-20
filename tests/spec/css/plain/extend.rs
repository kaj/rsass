//! Tests auto-converted from "sass-spec/spec/css/plain/extend.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "@import \"plain\";\
            \n\
            \na {@extend b}\
            \n"
        )
        .unwrap(),
        "b, a {\
        \n  c: d;\
        \n}\
        \n"
    );
}
