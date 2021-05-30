//! Tests auto-converted from "sass-spec/spec/directives/use/css/import.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .mock_file("import_into_use/_imported.scss", "@use \"used\";\n\nin-imported {a: b}\n")
        .mock_file("import_into_use/_used.scss", "in-used {a: b}\n")
        .mock_file("import_into_use_into_import/_imported-downstream.scss", "@use \"used\";\n\nin-imported-downstream {a: b}\n")
        .mock_file("import_into_use_into_import/_imported-upstream.scss", "in-imported-upstream {a: b}\n")
        .mock_file("import_into_use_into_import/_used.scss", "@import \"imported-upstream\";\n\nin-used {a: b}\n")
        .mock_file("import_module_imported_by_use/_shared.scss", "// This file is imported twice, so this should be printed twice, even though one\n// of those imports came from a use.\n@debug \"evaluating shared\";\n\na {b: c}\n")
        .mock_file("import_module_imported_by_use/_used.scss", "@import \"shared\";\n")
        .mock_file("nested_import_into_use/_imported.scss", "@use \"used\";\n\nin-imported {parent: inspect(&)}\n")
        .mock_file("nested_import_into_use/_used.scss", "// This parent selector will be `null`, because used modules are always\n// evaluated in a clean context, even if their CSS is then copied into an\n// imported file.\nin-used {parent: inspect(&)}\n")
        .mock_file("use_and_import_same/_other.scss", "// Every import always evaluates the file being imported, so this should be\n// printed twice.\n@debug \"evaluating other\";\n\na {b: c}\n")
        .mock_file("use_into_import/_imported.scss", "in-imported {a: b}\n")
        .mock_file("use_into_import/_used.scss", "@import \"imported\";\n\nin-used {a: b}\n")
        .mock_file("use_into_import_into_use/_imported.scss", "@use \"used-upstream\";\n\nin-imported {a: b}\n")
        .mock_file("use_into_import_into_use/_used-downstream.scss", "@import \"imported\";\n\nin-used-downstream {a: b}\n")
        .mock_file("use_into_import_into_use/_used-upstream.scss", "in-used-upstream {a: b}\n")
        .mock_file("use_module_used_by_import/_imported.scss", "@use \"shared\";\n")
        .mock_file("use_module_used_by_import/_shared.scss", "// This file is only used, so this should only be printed once, even though one\n// of those uses came from an import.\n@debug \"evaluating shared\";\n\n// However, imported CSS is always duplicated, so this should appear twice in\n// the output.\na {b: c}\n")
}

#[test]
fn import_into_use() {
    let runner = runner().with_cwd("import_into_use");
    assert_eq!(
        runner.ok("@import \"imported\";\n\
             \nin-input {a: b}\n"),
        "in-used {\
         \n  a: b;\
         \n}\
         \nin-imported {\
         \n  a: b;\
         \n}\
         \nin-input {\
         \n  a: b;\
         \n}\n"
    );
}
#[test]
fn import_into_use_into_import() {
    let runner = runner().with_cwd("import_into_use_into_import");
    assert_eq!(
        runner.ok("@import \"imported-downstream\";\n\
             \nin-input {a: b}\n"),
        "in-imported-upstream {\
         \n  a: b;\
         \n}\
         \nin-used {\
         \n  a: b;\
         \n}\
         \nin-imported-downstream {\
         \n  a: b;\
         \n}\
         \nin-input {\
         \n  a: b;\
         \n}\n"
    );
}
#[test]
fn import_module_imported_by_use() {
    let runner = runner().with_cwd("import_module_imported_by_use");
    assert_eq!(
        runner.ok("@use \"used\";\
             \n@import \"shared\";\n"),
        "a {\
         \n  b: c;\
         \n}\
         \na {\
         \n  b: c;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn nested_import_into_use() {
    let runner = runner().with_cwd("nested_import_into_use");
    assert_eq!(
        runner.ok("outer {@import \"imported\"}\n"),
        "outer in-used {\
         \n  parent: (in-used,);\
         \n}\
         \nouter in-imported {\
         \n  parent: (outer in-imported,);\
         \n}\n"
    );
}
#[test]
fn use_and_import_same() {
    let runner = runner().with_cwd("use_and_import_same");
    assert_eq!(
        runner.ok(
            "@use \"other\";\n\
             \n// @import always duplicates CSS, even when that CSS has been @used. In other\
             \n// words, @import\'s duplication takes precedence over @use\'s load-once policy.\
             \n@import \"other\";\n"
        ),
        "a {\
         \n  b: c;\
         \n}\
         \na {\
         \n  b: c;\
         \n}\n"
    );
}
#[test]
fn use_into_import() {
    let runner = runner().with_cwd("use_into_import");
    assert_eq!(
        runner.ok("@use \"used\";\n\
             \nin-input {a: b}\n"),
        "in-imported {\
         \n  a: b;\
         \n}\
         \nin-used {\
         \n  a: b;\
         \n}\
         \nin-input {\
         \n  a: b;\
         \n}\n"
    );
}
#[test]
fn use_into_import_into_use() {
    let runner = runner().with_cwd("use_into_import_into_use");
    assert_eq!(
        runner.ok("@use \"used-downstream\";\n\
             \nin-input {a: b}\n"),
        "in-used-upstream {\
         \n  a: b;\
         \n}\
         \nin-imported {\
         \n  a: b;\
         \n}\
         \nin-used-downstream {\
         \n  a: b;\
         \n}\
         \nin-input {\
         \n  a: b;\
         \n}\n"
    );
}
#[test]
fn use_module_used_by_import() {
    let runner = runner().with_cwd("use_module_used_by_import");
    assert_eq!(
        runner.ok("@use \"shared\";\
             \n@import \"imported\";\n"),
        "a {\
         \n  b: c;\
         \n}\
         \na {\
         \n  b: c;\
         \n}\n"
    );
}
