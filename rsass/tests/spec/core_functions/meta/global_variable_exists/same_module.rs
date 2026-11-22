//! Tests auto-converted from "sass-spec/spec/core_functions/meta/global_variable_exists/same_module.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("same_module")
        .mock_file("through_import/other.scss", "$global-variable: null;\n")
}

#[test]
fn global() {
    let runner = runner().with_cwd("global");
    assert_eq!(
        runner.ok("@use \"sass:meta\";\
             \n$global-variable: null;\n\
             \na {b: meta.global-variable-exists(global-variable)}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
fn local() {
    let runner = runner().with_cwd("local");
    assert_eq!(
        runner.ok("@use \"sass:meta\";\
             \na {\
             \n  $local-variable: null;\
             \n  b: meta.global-variable-exists(local-variable);\
             \n}\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
#[test]
fn non_existent() {
    let runner = runner().with_cwd("non_existent");
    assert_eq!(
        runner.ok("@use \"sass:meta\";\
             \na {\
             \n  b: meta.global-variable-exists(non-existent);\
             \n}\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
#[test]
fn through_import() {
    let runner = runner().with_cwd("through_import");
    assert_eq!(
        runner.ok("@use \"sass:meta\";\
             \n@import \"other\";\
             \na {b: meta.global-variable-exists(global-variable)}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
