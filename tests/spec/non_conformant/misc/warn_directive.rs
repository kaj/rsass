//! Tests auto-converted from "sass-spec/spec/non_conformant/misc/warn-directive.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("h1 { color: blue; } \
             \n@warn \"Don\'t crash the ambulance, whatever you do\"\n"),
        "h1 {\
         \n  color: blue;\
         \n}\n"
    );
}
