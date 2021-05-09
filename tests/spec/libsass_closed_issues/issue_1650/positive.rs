//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1650/positive.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(":nth-of-type(2n+1),\
             \n:nth-of-type(2n+  1),\
             \n:nth-of-type(2n  +1),\
             \n:nth-of-type(2n  +  1),\
             \n:nth-of-type( 2n  +  1 )\
             \n{ color: red; }"),
        ":nth-of-type(2n+1),\
         \n:nth-of-type(2n+ 1),\
         \n:nth-of-type(2n +1),\
         \n:nth-of-type(2n + 1),\
         \n:nth-of-type(2n + 1) {\
         \n  color: red;\
         \n}\n"
    );
}
