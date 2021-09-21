//! Tests auto-converted from "sass-spec/spec/directives/use/extend/scope.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .mock_file("diamond/_left.scss", "@use \"shared\";\n\nleft-extendee {@extend in-shared}\nleft-extender {@extend right-extendee !optional}\n")
        .mock_file("diamond/_right.scss", "@use \"shared\";\n\nright-extendee {@extend in-shared}\nright-extender {@extend left-extendee !optional}\n")
        .mock_file("diamond/_shared.scss", "in-shared {x: y}\n")
        .mock_file("downstream/_other.scss", "in-other {@extend in-input !optional}\n")
        .mock_file("isolated_through_import/_imported.scss", "@use \"used-by-imported\";\n")
        .mock_file("isolated_through_import/_shared.scss", "// This should appear twice in the output: once when it's used directly, and\n// once when it's used through @import (since @import copies its CSS). Each copy\n// should be extended exactly once.\n.in-shared {a: b}\n")
        .mock_file("isolated_through_import/_used-by-imported.scss", "@use \"shared\";\n\n.in-used-by-imported {@extend .in-shared}\n")
        .mock_file("private/_other.scss", "%-in-other {x: y}\n\nin-other {@extend %-in-other}\n")
        .mock_file("sibling/_left.scss", "left-extendee {in: left}\nleft-extender {@extend right-extendee !optional}\n")
        .mock_file("sibling/_right.scss", "right-extendee {in: right}\nright-extender {@extend left-extendee !optional}\n")
        .mock_file("use_and_import_into_diamond_extend/_downstream.scss", "// Even though left-extendee and right-extendee both end up in the style rule\n// defined in _shared.scss, they aren't extended by the other file because those\n// files don't use one another. This is true even though they're imported, which\n// eagerly resolves extensions.\n@use \"left\";\n@use \"right\";\n")
        .mock_file("use_and_import_into_diamond_extend/_imported.scss", "@use \"downstream\";\n")
        .mock_file("use_and_import_into_diamond_extend/_left.scss", "@use \"shared\";\n\nleft-extendee {@extend in-shared}\nleft-extender {@extend right-extendee !optional}\n")
        .mock_file("use_and_import_into_diamond_extend/_right.scss", "@use \"shared\";\n\nright-extendee {@extend in-shared}\nright-extender {@extend left-extendee !optional}\n")
        .mock_file("use_and_import_into_diamond_extend/_shared.scss", "in-shared {x: y}\n")
        .mock_file("use_into_use_and_import_into_import/_imported.scss", "@import \"shared\";\n\nin-imported {@extend shared}\n")
        .mock_file("use_into_use_and_import_into_import/_shared.scss", "// When this module is used by _imported.scss, its CSS is copied. The used\n// @extend only applies to the original, while the imported @extend applies to\n// both (since the imported extend is downstream of the used module).\nshared {x: y}\n")
        .mock_file("use_into_use_and_import_into_import/_used.scss", "@use \"shared\";\n\nin-used {@extend shared}\n")
        .mock_file("use_into_use_and_import_into_use/_imported.scss", "@use \"shared\";\n\nin-imported {@extend shared}\n")
        .mock_file("use_into_use_and_import_into_use/_shared.scss", "// When this module is used by _imported.scss, its CSS is copied. The used\n// @extend only applies to the original, while the imported @extend applies to\n// both (since the imported extend is downstream of the used module).\n\nshared {x: y}\n")
        .mock_file("use_into_use_and_import_into_use/_used.scss", "@use \"shared\";\n\nin-used {@extend shared}\n")
        .mock_file("use_into_use_and_use_into_import/_importer.scss", "@import \"shared\";\n\nin-importer {@extend shared}\n")
        .mock_file("use_into_use_and_use_into_import/_shared.scss", "// When this module is imported by _importer.scss, its CSS is copied. The\n// imported @extend only applies to the copy, and the used @extend only applies\n// to the original.\nshared {x: y}\n")
        .mock_file("use_into_use_and_use_into_import/_user.scss", "@use \"shared\";\n\nin-user {@extend shared}\n")
        .mock_file("use_into_use_and_use_into_import_into_use/_imported.scss", "@use \"shared\";\n\nin-imported {@extend shared}\n")
        .mock_file("use_into_use_and_use_into_import_into_use/_importer.scss", "@import \"imported\";\n")
        .mock_file("use_into_use_and_use_into_import_into_use/_shared.scss", "// When this module is used by _imported.scss, its CSS is copied. The imported\n// @extend only applies to the copy, and the used @extend only applies to the\n// original.\nshared {x: y}\n")
        .mock_file("use_into_use_and_use_into_import_into_use/_used.scss", "@use \"shared\";\n\nin-used {@extend shared}\n")
}

#[test]
#[ignore] // wrong result
fn diamond() {
    let runner = runner().with_cwd("diamond");
    assert_eq!(
        runner.ok(
            "// Even though left-extendee and right-extendee both end up in the style rule\
             \n// defined in _shared.scss, they aren\'t extended by the other file because those\
             \n// files don\'t use one another.\
             \n@use \"left\";\
             \n@use \"right\";\n"
        ),
        "in-shared, right-extendee, left-extendee {\
         \n  x: y;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn downstream() {
    let runner = runner().with_cwd("downstream");
    assert_eq!(
        runner.ok("@use \"other\";\n\
             \nin-input {x: y}\n"),
        "in-input {\
         \n  x: y;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn isolated_through_import() {
    let runner = runner().with_cwd("isolated_through_import");
    assert_eq!(
        runner.ok("@use \"used-by-input\";\
             \n@import \"imported\";\n"),
        ".in-shared, .in-used-by-input {\
         \n  a: b;\
         \n}\
         \n.in-shared, .in-used-by-imported {\
         \n  a: b;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn private() {
    let runner = runner().with_cwd("private");
    assert_eq!(
        runner.ok("@use \"other\";\n\
             \nin-input {@extend %-in-other !optional}\n"),
        "in-other {\
         \n  x: y;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn sibling() {
    let runner = runner().with_cwd("sibling");
    assert_eq!(
        runner.ok("@use \"left\";\
             \n@use \"right\";\n"),
        "left-extendee {\
         \n  in: left;\
         \n}\
         \nright-extendee {\
         \n  in: right;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn use_and_import_into_diamond_extend() {
    let runner = runner().with_cwd("use_and_import_into_diamond_extend");
    assert_eq!(
        runner.ok("@use \"downstream\";\
             \n@import \"downstream\";\
             \n@import \"imported\";\n"),
        "in-shared, right-extendee, left-extendee {\
         \n  x: y;\
         \n}\
         \nin-shared, right-extendee, left-extendee {\
         \n  x: y;\
         \n}\
         \nin-shared, right-extendee, left-extendee {\
         \n  x: y;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn use_into_use_and_import_into_import() {
    let runner = runner().with_cwd("use_into_use_and_import_into_import");
    assert_eq!(
        runner.ok("@use \"used\";\
             \n@import \"imported\";\n"),
        "shared, in-used, in-imported {\
         \n  x: y;\
         \n}\
         \nshared, in-imported {\
         \n  x: y;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn use_into_use_and_import_into_use() {
    let runner = runner().with_cwd("use_into_use_and_import_into_use");
    assert_eq!(
        runner.ok("@use \"used\";\
             \n@import \"imported\";\n"),
        "shared, in-used, in-imported {\
         \n  x: y;\
         \n}\
         \nshared, in-imported {\
         \n  x: y;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn use_into_use_and_use_into_import() {
    let runner = runner().with_cwd("use_into_use_and_use_into_import");
    assert_eq!(
        runner.ok("@use \"user\";\
             \n@use \"importer\";\n"),
        "shared, in-user {\
         \n  x: y;\
         \n}\
         \nshared, in-importer {\
         \n  x: y;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn use_into_use_and_use_into_import_into_use() {
    let runner =
        runner().with_cwd("use_into_use_and_use_into_import_into_use");
    assert_eq!(
        runner.ok("@use \"importer\";\
             \n@use \"used\";\n"),
        "shared, in-imported {\
         \n  x: y;\
         \n}\
         \nshared, in-used {\
         \n  x: y;\
         \n}\n"
    );
}
