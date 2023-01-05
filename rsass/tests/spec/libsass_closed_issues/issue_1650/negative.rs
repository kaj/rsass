//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1650/negative.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("negative")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(":nth-of-type(2n-1),\
             \n:nth-of-type(2n-  1),\
             \n:nth-of-type(2n  -1),\
             \n:nth-of-type(2n  -  1),\
             \n:nth-of-type( 2n  -  1 )\
             \n{ color: red; }\n"),
        ":nth-of-type(2n-1),\
         \n:nth-of-type(2n- 1),\
         \n:nth-of-type(2n -1),\
         \n:nth-of-type(2n - 1),\
         \n:nth-of-type(2n - 1) {\
         \n  color: red;\
         \n}\n"
    );
}
