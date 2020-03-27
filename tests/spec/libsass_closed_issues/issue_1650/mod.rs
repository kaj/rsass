//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1650"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/libsass-closed-issues/issue_1650/negative.hrx"
#[test]
#[ignore] // wrong result
fn negative() {
    assert_eq!(
        rsass(
            ":nth-of-type(2n-1),\
            \n:nth-of-type(2n-  1),\
            \n:nth-of-type(2n  -1),\
            \n:nth-of-type(2n  -  1),\
            \n:nth-of-type( 2n  -  1 )\
            \n{ color: red; }\
            \n"
        )
        .unwrap(),
        ":nth-of-type(2n-1),\
        \n:nth-of-type(2n- 1),\
        \n:nth-of-type(2n -1),\
        \n:nth-of-type(2n - 1),\
        \n:nth-of-type(2n - 1) {\
        \n  color: red;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1650/positive.hrx"
#[test]
#[ignore] // wrong result
fn positive() {
    assert_eq!(
        rsass(
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
