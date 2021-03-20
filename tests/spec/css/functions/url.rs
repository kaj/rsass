//! Tests auto-converted from "sass-spec/spec/css/functions/url.hrx"

mod exclam {
    #[test]
    fn middle() {
        assert_eq!(
            crate::rsass(
                "a {b: url(http://c.d/e!f)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: url(http://c.d/e!f);\
        \n}\
        \n"
        );
    }
    #[test]
    fn only() {
        assert_eq!(
            crate::rsass(
                "a {b: url(!)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: url(!);\
        \n}\
        \n"
        );
    }
}
