//! Tests auto-converted from "sass-spec/spec/non_conformant/misc/warn-directive.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "h1 { color: blue; } \
            \n@warn \"Don\'t crash the ambulance, whatever you do\"\
            \n"
        )
        .unwrap(),
        "h1 {\
        \n  color: blue;\
        \n}\
        \n"
    );
}
