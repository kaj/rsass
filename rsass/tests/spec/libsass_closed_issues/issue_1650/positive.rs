//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1650/positive.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("positive")
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
