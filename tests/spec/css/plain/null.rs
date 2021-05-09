//! Tests auto-converted from "sass-spec/spec/css/plain/null.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().mock_file("plain.css", "a {\n  x: null;\n}\n")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("@import \"plain\";\n"),
        "a {\
         \n  x: null;\
         \n}\n"
    );
}
