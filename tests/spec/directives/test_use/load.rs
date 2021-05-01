//! Tests auto-converted from "sass-spec/spec/directives/use/load.hrx"

mod explicit_extension {
    #[test]
    #[ignore] // unexepected error
    fn sass() {
        assert_eq!(
            crate::rsass(
                "@use \"other.sass\"\
            \n"
            )
            .unwrap(),
            "a {\
        \n  syntax: sass;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn scss() {
        assert_eq!(
            crate::rsass(
                "@use \"other.scss\"\
            \n"
            )
            .unwrap(),
            "a {\
        \n  syntax: scss;\
        \n}\
        \n"
        );
    }
}
mod index {
    #[test]
    #[ignore] // unexepected error
    fn dir_dot_foo() {
        assert_eq!(
            crate::rsass(
                "@use \"dir.foo\";\
            \n"
            )
            .unwrap(),
            ".foo {\
        \n  a: b;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
            crate::rsass(
                "@use \"dir\";\
            \n"
            )
            .unwrap(),
            ".foo {\
        \n  a: b;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn sass() {
        assert_eq!(
            crate::rsass(
                "@use \"dir\";\
            \n"
            )
            .unwrap(),
            ".foo {\
        \n  a: b;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn scss() {
        assert_eq!(
            crate::rsass(
                "@use \"dir\";\
            \n"
            )
            .unwrap(),
            ".foo {\
        \n  a: b;\
        \n}\
        \n"
        );
    }
}
mod precedence {
    #[test]
    #[ignore] // unexepected error
    fn ignores_import_only() {
        assert_eq!(
            crate::rsass(
                "@use \"other\";\
            \n"
            )
            .unwrap(),
            "a {\
        \n  import-only: false;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn normal_before_index() {
        assert_eq!(
            crate::rsass(
                "@use \"dir\";\
            \n"
            )
            .unwrap(),
            "a {\
        \n  index: false;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn sass_before_css() {
        assert_eq!(
            crate::rsass(
                "@use \"other\";\
            \n"
            )
            .unwrap(),
            "a {\
        \n  syntax: sass;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn scss_before_css() {
        assert_eq!(
            crate::rsass(
                "@use \"other\";\
            \n"
            )
            .unwrap(),
            "a {\
        \n  syntax: scss;\
        \n}\
        \n"
        );
    }
}
