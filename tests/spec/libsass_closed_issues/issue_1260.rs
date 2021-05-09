//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1260.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "$EQ-Selectors: ();\n\
             \n.el {\
             \n    $EQ-Selectors: append($EQ-Selectors, &, \'comma\') !global;\
             \n}\n\
             \nhtml:before {\
             \n  content: \"#{$EQ-Selectors}\";\
             \n}"
        ),
        "html:before {\
         \n  content: \".el\";\
         \n}\n"
    );
}
