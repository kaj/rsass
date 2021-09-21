//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgb/one_arg/special_functions/slash_list.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn alpha() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \na {b: rgb(list.slash(1 2 3, var(--c)))}\n"),
        "a {\
         \n  b: rgb(1, 2, 3, var(--c));\
         \n}\n"
    );
}
#[test]
fn channels() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \na {b: rgb(list.slash(var(--foo), 0.4))}\n"),
        "a {\
         \n  b: rgb(var(--foo) / 0.4);\
         \n}\n"
    );
}
#[test]
fn some_channels() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \na {b: rgb(list.slash(1 var(--foo), 0.4))}\n"),
        "a {\
         \n  b: rgb(1 var(--foo) / 0.4);\
         \n}\n"
    );
}
