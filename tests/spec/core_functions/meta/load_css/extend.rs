//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load_css/extend.hrx"

mod in_input {
    #[test]
    #[ignore] // unexepected error
    fn after() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:meta\";\
            \n@include meta.load-css(\"other\");\
            \n\
            \nd {@extend a}\
            \n"
            )
            .unwrap(),
            "a, d {\
        \n  b: c;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn before() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:meta\";\
            \n\
            \nd {@extend a}\
            \n@include meta.load-css(\"other\");\
            \n"
            )
            .unwrap(),
            "a, d {\
        \n  b: c;\
        \n}\
        \n"
        );
    }
}
mod in_other {
    #[test]
    #[ignore] // unexepected error
    fn after() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:meta\";\
            \n\
            \n@include meta.load-css(\"other\");\
            \na {b: c}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: c;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn before() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:meta\";\
            \n\
            \na {b: c}\
            \n@include meta.load-css(\"other\");\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: c;\
        \n}\
        \n"
        );
    }
}
