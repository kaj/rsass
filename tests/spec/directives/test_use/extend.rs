//! Tests auto-converted from "sass-spec/spec/directives/use/extend.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .mock_file("diamond_dependency/with_midstream_extend/_left.scss", "@use \"midstream\";\nin-left {\n  @extend in-midstream;\n  w: x;\n}\n")
        .mock_file("diamond_dependency/with_midstream_extend/_midstream.scss", "@use \"upstream\";\nin-midstream {@extend in-upstream}\n")
        .mock_file("diamond_dependency/with_midstream_extend/_right.scss", "@use \"midstream\";\nin-right {\n  @extend in-midstream;\n  y: z;\n}\n")
        .mock_file("diamond_dependency/with_midstream_extend/_upstream.scss", "in-upstream {a: b}\n")
        .mock_file("diamond_merge/_left.scss", "@use \"other\";\n\n.a {@extend %in-other}\n")
        .mock_file("diamond_merge/_other.scss", "%in-other.a {x: y}\n")
        .mock_file("diamond_merge/_right.scss", "@use \"other\";\n\n.b {@extend %in-other}\n")
        .mock_file("extended/from_other_file/_midstream.scss", "@use \"upstream\";\n\nin-midstream {@extend in-upstream}\n")
        .mock_file("extended/from_other_file/_upstream.scss", "in-upstream {x: y}\n")
        .mock_file("extended/from_same_file/_other.scss", "in-other-extender {@extend in-other-extendee}\n\nin-other-extendee {x: y}\n")
        .mock_file("far_upstream/_midstream.scss", "@use \"upstream\";\n")
        .mock_file("far_upstream/_upstream.scss", "in-upstream {x: y}\n")
        .mock_file("midstream_extend_within_pseudoselector/is/_midstream.scss", "@use \"upstream\";\n:is(in-midstream) {@extend in-upstream}\n")
        .mock_file("midstream_extend_within_pseudoselector/is/_upstream.scss", "in-upstream {a: b}\n")
        .mock_file("midstream_extend_within_pseudoselector/matches/_midstream.scss", "@use \"upstream\";\n:matches(in-midstream) {@extend in-upstream}\n")
        .mock_file("midstream_extend_within_pseudoselector/matches/_upstream.scss", "in-upstream {a: b}\n")
        .mock_file("optional_and_mandatory/different_files/_mandatory.scss", "@use \"shared\";\n\ndownstream {@extend in-other};\n")
        .mock_file("optional_and_mandatory/different_files/_optional.scss", "@use \"shared\";\n\ndownstream {@extend in-other !optional};\n")
        .mock_file("optional_and_mandatory/different_files/_shared.scss", "in-other {x: y}\n")
        .mock_file("optional_and_mandatory/same_file/_other.scss", "in-other {x: y}\n")
        .mock_file("placeholder/_other.scss", "%in-other {x: y}\n")
        .mock_file("scope/diamond/_left.scss", "@use \"shared\";\n\nleft-extendee {@extend in-shared}\nleft-extender {@extend right-extendee !optional}\n")
        .mock_file("scope/diamond/_right.scss", "@use \"shared\";\n\nright-extendee {@extend in-shared}\nright-extender {@extend left-extendee !optional}\n")
        .mock_file("scope/diamond/_shared.scss", "in-shared {x: y}\n")
        .mock_file("scope/downstream/_other.scss", "in-other {@extend in-input !optional}\n")
        .mock_file("scope/isolated_through_import/_imported.scss", "@use \"used-by-imported\";\n")
        .mock_file("scope/isolated_through_import/_shared.scss", "// This should appear twice in the output: once when it\'s used directly, and\n// once when it\'s used through @import (since @import copies its CSS). Each copy\n// should be extended exactly once.\n.in-shared {a: b}\n")
        .mock_file("scope/isolated_through_import/_used-by-imported.scss", "@use \"shared\";\n\n.in-used-by-imported {@extend .in-shared}\n")
        .mock_file("scope/private/_other.scss", "%-in-other {x: y}\n\nin-other {@extend %-in-other}\n")
        .mock_file("scope/sibling/_left.scss", "left-extendee {in: left}\nleft-extender {@extend right-extendee !optional}\n")
        .mock_file("scope/sibling/_right.scss", "right-extendee {in: right}\nright-extender {@extend left-extendee !optional}\n")
        .mock_file("scope/use_and_import_into_diamond_extend/_downstream.scss", "// Even though left-extendee and right-extendee both end up in the style rule\n// defined in _shared.scss, they aren\'t extended by the other file because those\n// files don\'t use one another. This is true even though they\'re imported, which\n// eagerly resolves extensions.\n@use \"left\";\n@use \"right\";\n")
        .mock_file("scope/use_and_import_into_diamond_extend/_imported.scss", "@use \"downstream\";\n")
        .mock_file("scope/use_and_import_into_diamond_extend/_left.scss", "@use \"shared\";\n\nleft-extendee {@extend in-shared}\nleft-extender {@extend right-extendee !optional}\n")
        .mock_file("scope/use_and_import_into_diamond_extend/_right.scss", "@use \"shared\";\n\nright-extendee {@extend in-shared}\nright-extender {@extend left-extendee !optional}\n")
        .mock_file("scope/use_and_import_into_diamond_extend/_shared.scss", "in-shared {x: y}\n")
        .mock_file("scope/use_into_use_and_import_into_import/_imported.scss", "@import \"shared\";\n\nin-imported {@extend shared}\n")
        .mock_file("scope/use_into_use_and_import_into_import/_shared.scss", "// When this module is used by _imported.scss, its CSS is copied. The used\n// @extend only applies to the original, while the imported @extend applies to\n// both (since the imported extend is downstream of the used module).\nshared {x: y}\n")
        .mock_file("scope/use_into_use_and_import_into_import/_used.scss", "@use \"shared\";\n\nin-used {@extend shared}\n")
        .mock_file("scope/use_into_use_and_import_into_use/_imported.scss", "@use \"shared\";\n\nin-imported {@extend shared}\n")
        .mock_file("scope/use_into_use_and_import_into_use/_shared.scss", "// When this module is used by _imported.scss, its CSS is copied. The used\n// @extend only applies to the original, while the imported @extend applies to\n// both (since the imported extend is downstream of the used module).\n\nshared {x: y}\n")
        .mock_file("scope/use_into_use_and_import_into_use/_used.scss", "@use \"shared\";\n\nin-used {@extend shared}\n")
        .mock_file("scope/use_into_use_and_use_into_import/_importer.scss", "@import \"shared\";\n\nin-importer {@extend shared}\n")
        .mock_file("scope/use_into_use_and_use_into_import/_shared.scss", "// When this module is imported by _importer.scss, its CSS is copied. The\n// imported @extend only applies to the copy, and the used @extend only applies\n// to the original.\nshared {x: y}\n")
        .mock_file("scope/use_into_use_and_use_into_import/_user.scss", "@use \"shared\";\n\nin-user {@extend shared}\n")
        .mock_file("scope/use_into_use_and_use_into_import_into_use/_imported.scss", "@use \"shared\";\n\nin-imported {@extend shared}\n")
        .mock_file("scope/use_into_use_and_use_into_import_into_use/_importer.scss", "@import \"imported\";\n")
        .mock_file("scope/use_into_use_and_use_into_import_into_use/_shared.scss", "// When this module is used by _imported.scss, its CSS is copied. The imported\n// @extend only applies to the copy, and the used @extend only applies to the\n// original.\nshared {x: y}\n")
        .mock_file("scope/use_into_use_and_use_into_import_into_use/_used.scss", "@use \"shared\";\n\nin-used {@extend shared}\n")
        .mock_file("upstream/_other.scss", "in-other {x: y}\n")
}

mod diamond_dependency {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("diamond_dependency")
    }

    #[test]
    #[ignore] // wrong result
    fn with_midstream_extend() {
        let runner = runner().with_cwd("with_midstream_extend");
        assert_eq!(
            runner.ok("@use \"left\";\
             \n@use \"right\";\n"),
            "in-upstream, in-midstream, in-right, in-left {\
         \n  a: b;\
         \n}\
         \nin-left {\
         \n  w: x;\
         \n}\
         \nin-right {\
         \n  y: z;\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // wrong result
fn diamond_merge() {
    let runner = runner().with_cwd("diamond_merge");
    assert_eq!(
        runner.ok(
            "// Sibling modules can\'t extend one another\'s selectors, but they can be merged\
             \n// together into the same selector list if they extend the same thing. If they\
             \n// are, they should be optimized with respect to one another.\
             \n//\
             \n// In this case, _left.scss causes the selector \".a.a\" to be generated, which is\
             \n// simplified to \".a\". Then _right.scss causes \".a.b\" to be generated. \".a\" is a\
             \n// superselector of \".a.b\" and \".a\" has the same specificity as the extender,\
             \n// \".b\", so \".a.b\" can (and should) be optimized away.\
             \n@use \"left\";\
             \n@use \"right\";\n"
        ),
        ".a {\
         \n  x: y;\
         \n}\n"
    );
}
mod extended {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("extended")
    }

    #[test]
    #[ignore] // wrong result
    fn from_other_file() {
        let runner = runner().with_cwd("from_other_file");
        assert_eq!(
            runner.ok("@use \"midstream\";\n\
             \nin-input {@extend in-midstream}\n"),
            "in-upstream, in-midstream, in-input {\
         \n  x: y;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn from_same_file() {
        let runner = runner().with_cwd("from_same_file");
        assert_eq!(
            runner.ok("@use \"other\";\n\
             \nin-input {@extend in-other-extender}\n"),
            "in-other-extendee, in-other-extender, in-input {\
         \n  x: y;\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // wrong result
fn far_upstream() {
    let runner = runner().with_cwd("far_upstream");
    assert_eq!(
        runner.ok("@use \"midstream\";\n\
             \nin-input {@extend in-upstream}\n"),
        "in-upstream, in-input {\
         \n  x: y;\
         \n}\n"
    );
}
mod midstream_extend_within_pseudoselector {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("midstream_extend_within_pseudoselector")
    }

    #[test]
    #[ignore] // wrong result
    fn is() {
        let runner = runner().with_cwd("is");
        assert_eq!(
            runner.ok("@use \"midstream\";\
             \nin-input {\
             \n  @extend in-midstream;\
             \n  y: z;\
             \n}\n"),
            "in-upstream, :is(in-midstream, in-input) {\
         \n  a: b;\
         \n}\
         \nin-input {\
         \n  y: z;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn matches() {
        let runner = runner().with_cwd("matches");
        assert_eq!(
            runner.ok("@use \"midstream\";\
             \nin-input {\
             \n  @extend in-midstream;\
             \n  y: z;\
             \n}\n"),
            "in-upstream, :matches(in-midstream, in-input) {\
         \n  a: b;\
         \n}\
         \nin-input {\
         \n  y: z;\
         \n}\n"
        );
    }
}
mod optional_and_mandatory {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("optional_and_mandatory")
    }

    #[test]
    #[ignore] // wrong result
    fn different_files() {
        let runner = runner().with_cwd("different_files");
        assert_eq!(
            runner.ok("@use \"optional\";\
             \n@use \"mandatory\";\n"),
            "in-other, downstream {\
         \n  x: y;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn same_file() {
        let runner = runner().with_cwd("same_file");
        assert_eq!(
            runner.ok("@use \"other\";\n\
             \nin-input {\
             \n  @extend in-other !optional;\
             \n  @extend in-other;\
             \n}\n"),
            "in-other, in-input {\
         \n  x: y;\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // wrong result
fn placeholder() {
    let runner = runner().with_cwd("placeholder");
    assert_eq!(
        runner.ok("@use \"other\";\n\
             \nin-input {@extend %in-other}\n"),
        "in-input {\
         \n  x: y;\
         \n}\n"
    );
}
mod scope {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("scope")
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
}
#[test]
#[ignore] // wrong result
fn upstream() {
    let runner = runner().with_cwd("upstream");
    assert_eq!(
        runner.ok("@use \"other\";\n\
             \nin-input {@extend in-other}\n"),
        "in-other, in-input {\
         \n  x: y;\
         \n}\n"
    );
}
