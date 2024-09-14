//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1260.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1260")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "@use \"sass:list\";\
             \n$EQ-Selectors: ();\n\
             \n.el {\
             \n    $EQ-Selectors: list.append($EQ-Selectors, &, \'comma\') !global;\
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
