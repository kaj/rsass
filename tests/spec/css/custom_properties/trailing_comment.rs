//! Tests auto-converted from "sass-spec/spec/css/custom_properties/trailing_comment.hrx"

mod sass {}
mod scss {
    #[test]
    #[ignore] // wrong result
    fn loud() {
        assert_eq!(
            crate::rsass(
                "a {\
            \n  --b: c /* comment */;\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  --b: c /* comment */;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn silent() {
        assert_eq!(
            crate::rsass(
                "a {\
            \n  --b: c // comment;\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  --b: c // comment;\
        \n}\
        \n"
        );
    }
}
