//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgb/multi_argument_var.hrx"

#[test]
fn t1_of_1() {
    assert_eq!(
        crate::rsass(
            "a {b: rgb(var(--foo))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgb(var(--foo));\
        \n}\
        \n"
    );
}
#[test]
fn t1_of_2() {
    assert_eq!(
        crate::rsass(
            "a {b: rgb(var(--foo), 0.4)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgb(var(--foo), 0.4);\
        \n}\
        \n"
    );
}
#[test]
fn t1_of_3() {
    assert_eq!(
        crate::rsass(
            "// var() is substituted before parsing, so it may contain multiple arguments.\
            \na {b: rgb(var(--foo), 3, 0.4)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgb(var(--foo), 3, 0.4);\
        \n}\
        \n"
    );
}
#[test]
fn t2_of_2() {
    assert_eq!(
        crate::rsass(
            "a {b: rgb(1, var(--foo))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgb(1, var(--foo));\
        \n}\
        \n"
    );
}
#[test]
fn t2_of_3() {
    assert_eq!(
        crate::rsass(
            "a {b: rgb(1, var(--foo), 0.4)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgb(1, var(--foo), 0.4);\
        \n}\
        \n"
    );
}
#[test]
fn t3_of_3() {
    assert_eq!(
        crate::rsass(
            "a {b: rgb(1, 2, var(--foo))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgb(1, 2, var(--foo));\
        \n}\
        \n"
    );
}
