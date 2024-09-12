//! Tests auto-converted from "sass-spec/spec/libsass-todo-issues/issue_2818.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2818")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \n@use \"sass:meta\";\
             \n$map: (\"lightness\": 10%, \"saturation\": 10%);\
             \n$base: meta.call(meta.get-function(\'scale\', $module: \'color\'), #dedede, $map...);\
             \ntest { color: $base; }\n"
        ),
        "test {\
         \n  color: rgb(228.27, 222.33, 222.33);\
         \n}\n"
    );
}
