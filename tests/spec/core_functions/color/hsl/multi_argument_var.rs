//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsl/multi_argument_var.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn t1_of_1() {
    assert_eq!(
        runner().ok("a {b: hsl(var(--foo))}\n"),
        "a {\
         \n  b: hsl(var(--foo));\
         \n}\n"
    );
}
#[test]
fn t1_of_2() {
    assert_eq!(
        runner().ok("a {b: hsl(var(--foo), 0.4)}\n"),
        "a {\
         \n  b: hsl(var(--foo), 0.4);\
         \n}\n"
    );
}
#[test]
fn t1_of_3() {
    assert_eq!(
        runner().ok(
            "// var() is substituted before parsing, so it may contain multiple arguments.\
             \na {b: hsl(var(--foo), 3%, 0.4)}\n"
        ),
        "a {\
         \n  b: hsl(var(--foo), 3%, 0.4);\
         \n}\n"
    );
}
#[test]
fn t2_of_2() {
    assert_eq!(
        runner().ok("a {b: hsl(1, var(--foo))}\n"),
        "a {\
         \n  b: hsl(1, var(--foo));\
         \n}\n"
    );
}
#[test]
fn t2_of_3() {
    assert_eq!(
        runner().ok("a {b: hsl(1, var(--foo), 0.4)}\n"),
        "a {\
         \n  b: hsl(1, var(--foo), 0.4);\
         \n}\n"
    );
}
#[test]
fn t3_of_3() {
    assert_eq!(
        runner().ok("a {b: hsl(1, 2%, var(--foo))}\n"),
        "a {\
         \n  b: hsl(1, 2%, var(--foo));\
         \n}\n"
    );
}
