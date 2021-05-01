//! Tests auto-converted from "sass-spec/spec/directives/use/extend.hrx"

mod diamond_dependency {
    #[test]
    #[ignore] // unexepected error
    fn with_midstream_extend() {
        assert_eq!(
            crate::rsass(
                "@use \"left\";\
            \n@use \"right\";\
            \n"
            )
            .unwrap(),
            "in-upstream, in-midstream, in-right, in-left {\
        \n  a: b;\
        \n}\
        \nin-left {\
        \n  w: x;\
        \n}\
        \nin-right {\
        \n  y: z;\
        \n}\
        \n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn diamond_merge() {
    assert_eq!(
        crate::rsass(
            "// Sibling modules can\'t extend one another\'s selectors, but they can be merged\
            \n// together into the same selector list if they extend the same thing. If they\
            \n// are, they should be optimized with respect to one another.\
            \n//\
            \n// In this case, _left.scss causes the selector \".a.a\" to be generated, which is\
            \n// simplified to \".a\". Then _right.scss causes \".a.b\" to be generated. \".a\" is a\
            \n// superselector of \".a.b\" and \".a\" has the same specificity as the extender,\
            \n// \".b\", so \".a.b\" can (and should) be optimized away.\
            \n@use \"left\";\
            \n@use \"right\";\
            \n"
        )
        .unwrap(),
        ".a {\
        \n  x: y;\
        \n}\
        \n"
    );
}
mod extended {
    #[test]
    #[ignore] // unexepected error
    fn from_other_file() {
        assert_eq!(
            crate::rsass(
                "@use \"midstream\";\
            \n\
            \nin-input {@extend in-midstream}\
            \n"
            )
            .unwrap(),
            "in-upstream, in-midstream, in-input {\
        \n  x: y;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn from_same_file() {
        assert_eq!(
            crate::rsass(
                "@use \"other\";\
            \n\
            \nin-input {@extend in-other-extender}\
            \n"
            )
            .unwrap(),
            "in-other-extendee, in-other-extender, in-input {\
        \n  x: y;\
        \n}\
        \n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn far_upstream() {
    assert_eq!(
        crate::rsass(
            "@use \"midstream\";\
            \n\
            \nin-input {@extend in-upstream}\
            \n"
        )
        .unwrap(),
        "in-upstream, in-input {\
        \n  x: y;\
        \n}\
        \n"
    );
}
mod midstream_extend_within_pseudoselector {
    #[test]
    #[ignore] // unexepected error
    fn matches() {
        assert_eq!(
            crate::rsass(
                "@use \"midstream\";\
            \nin-input {\
            \n  @extend in-midstream;\
            \n  y: z;\
            \n}\
            \n"
            )
            .unwrap(),
            "in-upstream, :matches(in-midstream, in-input) {\
        \n  a: b;\
        \n}\
        \nin-input {\
        \n  y: z;\
        \n}\
        \n"
        );
    }
}
mod optional_and_mandatory {
    #[test]
    #[ignore] // unexepected error
    fn different_files() {
        assert_eq!(
            crate::rsass(
                "@use \"optional\";\
            \n@use \"mandatory\";\
            \n"
            )
            .unwrap(),
            "in-other, downstream {\
        \n  x: y;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn same_file() {
        assert_eq!(
            crate::rsass(
                "@use \"other\";\
            \n\
            \nin-input {\
            \n  @extend in-other !optional;\
            \n  @extend in-other;\
            \n}\
            \n"
            )
            .unwrap(),
            "in-other, in-input {\
        \n  x: y;\
        \n}\
        \n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn placeholder() {
    assert_eq!(
        crate::rsass(
            "@use \"other\";\
            \n\
            \nin-input {@extend %in-other}\
            \n"
        )
        .unwrap(),
        "in-input {\
        \n  x: y;\
        \n}\
        \n"
    );
}
mod scope {
    #[test]
    #[ignore] // unexepected error
    fn diamond() {
        assert_eq!(
        crate::rsass(
            "// Even though left-extendee and right-extendee both end up in the style rule\
            \n// defined in _shared.scss, they aren\'t extended by the other file because those\
            \n// files don\'t use one another.\
            \n@use \"left\";\
            \n@use \"right\";\
            \n"
        )
        .unwrap(),
        "in-shared, right-extendee, left-extendee {\
        \n  x: y;\
        \n}\
        \n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn downstream() {
        assert_eq!(
            crate::rsass(
                "@use \"other\";\
            \n\
            \nin-input {x: y}\
            \n"
            )
            .unwrap(),
            "in-input {\
        \n  x: y;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn isolated_through_import() {
        assert_eq!(
            crate::rsass(
                "@use \"used-by-input\";\
            \n@import \"imported\";\
            \n"
            )
            .unwrap(),
            ".in-shared, .in-used-by-input {\
        \n  a: b;\
        \n}\
        \n.in-shared, .in-used-by-imported {\
        \n  a: b;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn private() {
        assert_eq!(
            crate::rsass(
                "@use \"other\";\
            \n\
            \nin-input {@extend %-in-other !optional}\
            \n"
            )
            .unwrap(),
            "in-other {\
        \n  x: y;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn sibling() {
        assert_eq!(
            crate::rsass(
                "@use \"left\";\
            \n@use \"right\";\
            \n"
            )
            .unwrap(),
            "left-extendee {\
        \n  in: left;\
        \n}\
        \nright-extendee {\
        \n  in: right;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn use_and_import_into_diamond_extend() {
        assert_eq!(
            crate::rsass(
                "@use \"downstream\";\
            \n@import \"downstream\";\
            \n@import \"imported\";\
            \n"
            )
            .unwrap(),
            "in-shared, right-extendee, left-extendee {\
        \n  x: y;\
        \n}\
        \nin-shared, right-extendee, left-extendee {\
        \n  x: y;\
        \n}\
        \nin-shared, right-extendee, left-extendee {\
        \n  x: y;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn use_into_use_and_import_into_import() {
        assert_eq!(
            crate::rsass(
                "@use \"used\";\
            \n@import \"imported\";\
            \n"
            )
            .unwrap(),
            "shared, in-used, in-imported {\
        \n  x: y;\
        \n}\
        \nshared, in-imported {\
        \n  x: y;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn use_into_use_and_import_into_use() {
        assert_eq!(
            crate::rsass(
                "@use \"used\";\
            \n@import \"imported\";\
            \n"
            )
            .unwrap(),
            "shared, in-used, in-imported {\
        \n  x: y;\
        \n}\
        \nshared, in-imported {\
        \n  x: y;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn use_into_use_and_use_into_import() {
        assert_eq!(
            crate::rsass(
                "@use \"user\";\
            \n@use \"importer\";\
            \n"
            )
            .unwrap(),
            "shared, in-user {\
        \n  x: y;\
        \n}\
        \nshared, in-importer {\
        \n  x: y;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn use_into_use_and_use_into_import_into_use() {
        assert_eq!(
            crate::rsass(
                "@use \"importer\";\
            \n@use \"used\";\
            \n"
            )
            .unwrap(),
            "shared, in-imported {\
        \n  x: y;\
        \n}\
        \nshared, in-used {\
        \n  x: y;\
        \n}\
        \n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn upstream() {
    assert_eq!(
        crate::rsass(
            "@use \"other\";\
            \n\
            \nin-input {@extend in-other}\
            \n"
        )
        .unwrap(),
        "in-other, in-input {\
        \n  x: y;\
        \n}\
        \n"
    );
}
