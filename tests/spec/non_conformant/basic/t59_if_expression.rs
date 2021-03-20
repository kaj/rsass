//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/59_if_expression.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$x: 0;\
            \n$if-false: whatever;\
            \n\
            \ndiv {\
            \n  foo: if($if-true: hey, $if-false: ho, $condition: true);\
            \n  foo: if($if-true: hey, $if-false: ho, $condition: false);\
            \n  foo: if($x != 0, if($x, true, false), unquote(\"x is zero\"));\
            \n  foo: if(false, 1/0, $if-false: $if-false);\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  foo: hey;\
        \n  foo: ho;\
        \n  foo: x is zero;\
        \n  foo: whatever;\
        \n}\
        \n"
    );
}
