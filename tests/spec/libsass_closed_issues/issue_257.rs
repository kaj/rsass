//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_257.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass("body{background:blue; a{color:black;}}").unwrap(),
        "body {\
        \n  background: blue;\
        \n}\
        \nbody a {\
        \n  color: black;\
        \n}\
        \n"
    );
}
