//! Tests auto-converted from "sass-spec/spec/directives/at_root.hrx"

mod keyframes {
    #[test]
    #[ignore] // unexepected error
    fn all() {
        assert_eq!(
            crate::rsass(
                "@keyframes a {\
            \n  @at-root (without: all) {\
            \n    b {c: d}\
            \n  }\
            \n}\
            \n"
            )
            .unwrap(),
            "@keyframes a {}\
        \nb {\
        \n  c: d;\
        \n}\
        \n"
        );
    }
}
