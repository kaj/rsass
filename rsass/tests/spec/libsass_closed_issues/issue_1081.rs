//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1081.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("issue_1081")
        .mock_file("_import.scss", "import-before {\n  foo: $foo;\n}\n\n$foo: baz !global !default;\n\nimport-after {\n  foo: $foo;\n}\n")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("$foo: foo !global !default;\n\
             \ndefault {\
             \n  foo: $foo;\
             \n}\n\
             \n$foo: bar;\n\
             \nafter {\
             \n  @import \"import\";\
             \n  foo: $foo;\
             \n}\n"),
        "default {\
         \n  foo: foo;\
         \n}\
         \nafter import-before {\
         \n  foo: bar;\
         \n}\
         \nafter import-after {\
         \n  foo: bar;\
         \n}\
         \nafter {\
         \n  foo: bar;\
         \n}\n"
    );
}
