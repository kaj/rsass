//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load_css/twice.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .mock_file("load_css/different_extend/_left.scss", "@use \"sass:meta\";\n@include meta.load-css(\"other\");\nleft {@extend a}\n")
        .mock_file("load_css/different_extend/_other.scss", "a {b: c}\n")
        .mock_file("load_css/different_extend/_right.scss", "@use \"sass:meta\";\n@include meta.load-css(\"other\");\nright {@extend a}\n")
        .mock_file("load_css/different_nesting/_other.scss", "c {d: e}\n")
        .mock_file("load_css/runs_once/_other.scss", "@debug \"in other\";\n")
        .mock_file("shares_state/_other.scss", "@use \"shared\";\n\nshared.$b: value set by other;\n")
        .mock_file("shares_state/_shared.scss", "$b: default value;\n")
        .mock_file("use/different_extend/_midstream.scss", "@use \"other\";\n\n// This extension should only apply to the copy of `_other.scss` loaded from\n// `@use`, *not* the copy loaded from `load-css()`.\na {@extend b}\n")
        .mock_file("use/different_extend/_other.scss", "b {c: d}\n")
        .mock_file("use/different_nesting/_other.scss", "b {c: d}\n")
        .mock_file("use/runs_once/different_text/_other.scss", "@debug \"in other\";\n")
        .mock_file("use/runs_once/same_text/_other.scss", "@debug \"in other\";\n")
}

mod load_css {
    #[allow(unused)]
    use super::runner;
    #[test]
    #[ignore] // unexepected error
    fn different_extend() {
        assert_eq!(
            runner().ok("@use \"left\";\
             \n@use \"right\";\n"),
            "a, left {\
         \n  b: c;\
         \n}\
         \na, right {\
         \n  b: c;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn different_nesting() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\
             \na {@include meta.load-css(\"other\")}\
             \nb {@include meta.load-css(\"other\")}\n"),
            "a c {\
         \n  d: e;\
         \n}\
         \nb c {\
         \n  d: e;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn runs_once() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\
             \n@include meta.load-css(\"other\");\
             \n@include meta.load-css(\"other\");\n\
             \n/* No output other than this */\n"),
            "/* No output other than this */\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn shares_state() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \n@use \"shared\";\
             \n@include meta.load-css(\"other\");\n\
             \na {shared-b: shared.$b}\n"),
        "a {\
         \n  shared-b: value set by other;\
         \n}\n"
    );
}
mod test_use {
    #[allow(unused)]
    use super::runner;
    #[test]
    #[ignore] // unexepected error
    fn different_extend() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\
             \n@use \"midstream\";\
             \n@include meta.load-css(\"other\")\n"),
            "b, a {\
         \n  c: d;\
         \n}\
         \nb {\
         \n  c: d;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn different_nesting() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\
             \n@use \"other\";\
             \na {@include meta.load-css(\"other\")}\n"),
            "b {\
         \n  c: d;\
         \n}\
         \na b {\
         \n  c: d;\
         \n}\n"
        );
    }
    mod runs_once {
        #[allow(unused)]
        use super::runner;
        #[test]
        #[ignore] // unexepected error
        fn different_text() {
            assert_eq!(
                runner().ok("@use \"sass:meta\";\
             \n@use \"other\";\
             \n@include meta.load-css(\"_other\");\n\
             \n/* No output other than this */\n"),
                "/* No output other than this */\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn same_text() {
            assert_eq!(
                runner().ok("@use \"sass:meta\";\
             \n@use \"other\";\
             \n@include meta.load-css(\"other\");\n\
             \n/* No output other than this */\n"),
                "/* No output other than this */\n"
            );
        }
    }
}
