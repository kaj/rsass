//! Tests auto-converted from "sass-spec/spec/css/plain/extend.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().mock_file("_plain.css", "b {c: d}\n")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("@import \"plain\";\n\
             \na {@extend b}\n"),
        "b, a {\
         \n  c: d;\
         \n}\n"
    );
}
