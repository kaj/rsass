//! Tests auto-converted from "sass-spec/spec/libsass/Sáss-UŢF8.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("span.utf8-in-path {\
             \n  margin: auto;\
             \n}\n"),
        "span.utf8-in-path {\
         \n  margin: auto;\
         \n}\n"
    );
}
