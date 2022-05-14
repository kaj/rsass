//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2399.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2399")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(".thing {\r\
             \n\tcolor: black;\r\
             \n}\r\
             \n\r\
             \n.a,\r\
             \n.b,\r\
             \n.c,\r\
             \n.d,\r\
             \n.e {\r\
             \n\t&:not(.thing) { @extend .thing; }\r\
             \n}"),
        ".thing, .a:not(.thing),\
         \n.b:not(.thing),\
         \n.c:not(.thing),\
         \n.d:not(.thing),\
         \n.e:not(.thing) {\
         \n  color: black;\
         \n}\n"
    );
}
