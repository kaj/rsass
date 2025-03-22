//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_672.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_672")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \n@mixin test($arglist...) {\
             \n    $map: meta.keywords($arglist);\
             \n    answer: if($map, \"Yep\", \"Nope\");\
             \n}\n\
             \nwith-keyword-args{\
             \n    @include test($arg1: one, $arg2: two, $arg3: three);\
             \n}\
             \nwith-no-args {\
             \n    @include test();\
             \n}\
             \nwithout-keyword-args {\
             \n    @include test(not-a-keyword-arg-1 , not-a-keyword-arg-2);\
             \n}\n"),
        "with-keyword-args {\
         \n  answer: \"Yep\";\
         \n}\
         \nwith-no-args {\
         \n  answer: \"Yep\";\
         \n}\
         \nwithout-keyword-args {\
         \n  answer: \"Yep\";\
         \n}\n"
    );
}
