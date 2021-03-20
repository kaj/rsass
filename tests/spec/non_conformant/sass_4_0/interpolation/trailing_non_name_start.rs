//! Tests auto-converted from "sass-spec/spec/non_conformant/sass_4_0/interpolation/trailing_non_name_start.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".trailing-non-name-start {\
            \n  // We want to treat interpolation as though it\'s a valid identifier character,\
            \n  // which means that if it\'s followed by a [name character][] that\'s not a\
            \n  // [name start character][] it should still treat that as part of the\
            \n  // identifier.\
            \n  //\
            \n  // [name character]: https://drafts.csswg.org/css-syntax-3/#name-code-point\
            \n  // [name start character]: https://drafts.csswg.org/css-syntax-3/#name-start-code-point\
            \n  digit: foo#{bar}12;\
            \n  hyphen: foo#{bar}-12;\
            \n  double-hyphen: foo#{bar}--12;\
            \n}\
            \n"
        )
        .unwrap(),
        ".trailing-non-name-start {\
        \n  digit: foobar12;\
        \n  hyphen: foobar-12;\
        \n  double-hyphen: foobar--12;\
        \n}\
        \n"
    );
}
