//! Tests auto-converted from "sass-spec/spec/directives/use/member/nested_global_variable.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .mock_file("direct/other.scss", "x {\n  @if false {\n    // Even though this assignment is deeply nested and never evaluated, it\n    // creates a variable slot in the module that defaults to null. This ensures\n    // that a module will always expose the same members regardless of how it's\n    // evaluated.\n    $member: value !global;\n  }\n}\n")
        .mock_file("through_import/imported.scss", "x {\n  @if false {\n    // Even though this assignment is deeply nested and never evaluated, it\n    // creates a variable slot in the module that defaults to null. This ensures\n    // that a module will always expose the same members regardless of how it's\n    // evaluated.\n    $member: value !global;\n  }\n}\n")
        .mock_file("through_import/used.scss", "@import \"imported\";\n")
}

#[test]
#[ignore] // unexepected error
fn direct() {
    let runner = runner().with_cwd("direct");
    assert_eq!(
        runner.ok("@use \"other\";\n\
             \na {b: inspect(other.$member)}\n"),
        "a {\
         \n  b: null;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn through_import() {
    let runner = runner().with_cwd("through_import");
    assert_eq!(
        runner.ok("@use \"used\";\n\
             \na {b: inspect(used.$member)}\n"),
        "a {\
         \n  b: null;\
         \n}\n"
    );
}
