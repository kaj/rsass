//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1650/positive.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            ":nth-of-type(2n+1),\
            \n:nth-of-type(2n+  1),\
            \n:nth-of-type(2n  +1),\
            \n:nth-of-type(2n  +  1),\
            \n:nth-of-type( 2n  +  1 )\
            \n{ color: red; }"
        )
        .unwrap(),
        ":nth-of-type(2n+1),\
        \n:nth-of-type(2n+ 1),\
        \n:nth-of-type(2n +1),\
        \n:nth-of-type(2n + 1),\
        \n:nth-of-type(2n + 1) {\
        \n  color: red;\
        \n}\
        \n"
    );
}
