//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1260.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$EQ-Selectors: ();\
            \n\
            \n.el {\
            \n    $EQ-Selectors: append($EQ-Selectors, &, \'comma\') !global;\
            \n}\
            \n\
            \nhtml:before {\
            \n  content: \"#{$EQ-Selectors}\";\
            \n}"
        )
        .unwrap(),
        "html:before {\
        \n  content: \".el\";\
        \n}\
        \n"
    );
}
