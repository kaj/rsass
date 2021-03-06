//! Tests auto-converted from "sass-spec/spec/libsass/calc.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$x: 2;\
             \nbody {\
             \n  width: calc($x + 2 - 3em / hoolabaloo);\
             \n  width: -moz-calc($x + 2 - 3em / hoolabaloo);\
             \n  width: -webkit-calc($x + 2 - 3em / hoolabaloo);\
             \n  width: -ms-calc($x + 2 - 3em / hoolabaloo);\
             \n  height: foo(2 + 2);\
             \n}\n"),
        "body {\
         \n  width: calc($x + 2 - 3em / hoolabaloo);\
         \n  width: -moz-calc($x + 2 - 3em / hoolabaloo);\
         \n  width: -webkit-calc($x + 2 - 3em / hoolabaloo);\
         \n  width: -ms-calc($x + 2 - 3em / hoolabaloo);\
         \n  height: foo(4);\
         \n}\n"
    );
}
