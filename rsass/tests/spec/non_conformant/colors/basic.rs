//! Tests auto-converted from "sass-spec/spec/non_conformant/colors/basic.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("basic")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("p {\
             \n  color: rgb(255, 128, 0);\
             \n  color: red green blue;\
             \n  color: (red) (green) (blue);\
             \n  color: red + hux;\
             \n  color: unquote(\"red\") + green;\
             \n  foo: rgb(200, 150%, 170%);\
             \n}"),
        "p {\
         \n  color: rgb(255, 128, 0);\
         \n  color: red green blue;\
         \n  color: red green blue;\
         \n  color: redhux;\
         \n  color: redgreen;\
         \n  foo: rgb(200, 255, 255);\
         \n}\n"
    );
}
