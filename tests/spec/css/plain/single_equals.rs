//! Tests auto-converted from "sass-spec/spec/css/plain/single_equals.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().mock_file(
        "plain.css",
        "a {\n  single-equals: alpha(opacity=65);\n}\n",
    )
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("@import \"plain\";\n"),
        "a {\
         \n  single-equals: alpha(opacity=65);\
         \n}\n"
    );
}
