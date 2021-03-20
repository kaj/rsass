//! Tests auto-converted from "sass-spec/spec/css/keyframes.hrx"

mod bubble {
    #[test]
    #[ignore] // wrong result
    fn empty() {
        assert_eq!(
            crate::rsass(
                "// Regression test for sass/dart-sass#611.\
            \na {\
            \n  @keyframes {/**/}\
            \n}\
            \n"
            )
            .unwrap(),
            "@keyframes {\
        \n  /**/\
        \n}\
        \n"
        );
    }
}
