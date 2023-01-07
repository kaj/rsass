//! Tests auto-converted from "sass-spec/spec/css/plain/boolean_operations.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("boolean_operations")
        .mock_file("plain.css", "a {\n  and: true and false;\n  or: true or false;\n  not: not true;\n}\n")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@import \"plain\";\n"),
        "a {\
         \n  and: true and false;\
         \n  or: true or false;\
         \n  not: not true;\
         \n}\n"
    );
}
