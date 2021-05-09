//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_893.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("$gutter: 20px;\n\
             \n.row {\
             \n  margin: 0 -$gutter;\
             \n}"),
        ".row {\
         \n  margin: -20px;\
         \n}\n"
    );
}
