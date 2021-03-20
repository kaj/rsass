//! Tests auto-converted from "sass-spec/spec/css/plain/hacks.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "@import \"plain\";\
            \n"
        )
        .unwrap(),
        ".hacks {\
        \n  *x: y;\
        \n  :x: y;\
        \n  #x: y;\
        \n  .x: y;\
        \n}\
        \n"
    );
}
