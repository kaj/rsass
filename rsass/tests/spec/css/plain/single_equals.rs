//! Tests auto-converted from "sass-spec/spec/css/plain/single_equals.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("single_equals").mock_file(
        "plain.css",
        "a {\n  single-equals: alpha(opacity=65);\n}\n",
    )
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"plain\";\n"),
        "a {\
         \n  single-equals: alpha(opacity=65);\
         \n}\n"
    );
}
