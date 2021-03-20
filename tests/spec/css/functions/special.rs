//! Tests auto-converted from "sass-spec/spec/css/functions/special.hrx"

mod clamp {
    #[test]
    fn interpolation() {
        assert_eq!(
            crate::rsass(
                "a {b: clamp(#{0}, #{1}, #{2})}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: clamp(0, 1, 2);\
        \n}\
        \n"
        );
    }
    #[test]
    fn numbers() {
        assert_eq!(
            crate::rsass(
                "a {b: clamp(0, 1, 2)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: clamp(0, 1, 2);\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn punctuation() {
        assert_eq!(
            crate::rsass(
                "a {b: clamp(@#$%^&*({[]})_-+=|\\\\:\"\"\'\'<>,.?/)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: clamp(@#$%^&*({[]})_-+=|\\\\:\"\"\"\"<>,.?/);\
        \n}\
        \n"
        );
    }
}
