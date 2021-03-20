//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/composed-args.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin A($width: 0, $height: 0, $opacity: 0) {\
            \n  width: $width;\
            \n  height: $height;\
            \n  opacity: $opacity;\
            \n}\
            \n\
            \n@mixin B($args...) {\
            \n  @include A($args...);\
            \n}\
            \n\
            \n@mixin C($args...) {\
            \n  @include B($args...);\
            \n}\
            \n\
            \n.testOneLevelPassthrough {\
            \n  @include B(1px, 2px, 0.3);\
            \n}\
            \n\
            \n.testOneLevelNoArgs {\
            \n  @include B();\
            \n}\
            \n\
            \n.testOneLevelSingleArg {\
            \n  @include B(1px);\
            \n}\
            \n\
            \n.testOneLevelNamedSingleArg {\
            \n  @include B($opacity: 0.1);\
            \n}\
            \n\
            \n.testOneLevelNamedArgs {\
            \n  @include B($opacity: 0.3, $width: 1px, $height: 2px);\
            \n}\
            \n\
            \n.testTwoLevelPassthrough {\
            \n  @include C(1px, 2px, 0.3);\
            \n}\
            \n\
            \n.testTwoLevelNoArgs {\
            \n  @include C();\
            \n}\
            \n\
            \n.testTwoLevelSingleArg {\
            \n  @include C(1px);\
            \n}\
            \n\
            \n.testTwoLevelNamedSingleArg {\
            \n  @include C($opacity: 0.1);\
            \n}\
            \n\
            \n.testTwoLevelNamedArgs {\
            \n  @include C($opacity: 0.3, $width: 1px, $height: 2px);\
            \n}\
            \n"
        )
        .unwrap(),
        ".testOneLevelPassthrough {\
        \n  width: 1px;\
        \n  height: 2px;\
        \n  opacity: 0.3;\
        \n}\
        \n.testOneLevelNoArgs {\
        \n  width: 0;\
        \n  height: 0;\
        \n  opacity: 0;\
        \n}\
        \n.testOneLevelSingleArg {\
        \n  width: 1px;\
        \n  height: 0;\
        \n  opacity: 0;\
        \n}\
        \n.testOneLevelNamedSingleArg {\
        \n  width: 0;\
        \n  height: 0;\
        \n  opacity: 0.1;\
        \n}\
        \n.testOneLevelNamedArgs {\
        \n  width: 1px;\
        \n  height: 2px;\
        \n  opacity: 0.3;\
        \n}\
        \n.testTwoLevelPassthrough {\
        \n  width: 1px;\
        \n  height: 2px;\
        \n  opacity: 0.3;\
        \n}\
        \n.testTwoLevelNoArgs {\
        \n  width: 0;\
        \n  height: 0;\
        \n  opacity: 0;\
        \n}\
        \n.testTwoLevelSingleArg {\
        \n  width: 1px;\
        \n  height: 0;\
        \n  opacity: 0;\
        \n}\
        \n.testTwoLevelNamedSingleArg {\
        \n  width: 0;\
        \n  height: 0;\
        \n  opacity: 0.1;\
        \n}\
        \n.testTwoLevelNamedArgs {\
        \n  width: 1px;\
        \n  height: 2px;\
        \n  opacity: 0.3;\
        \n}\
        \n"
    );
}
