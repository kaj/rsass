//! Tests auto-converted from "sass-spec/spec/libsass-todo-issues/issue_2818.hrx"

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
         \n  color: rgb(89.5176470588%, 87.1882352941%, 87.1882352941%);\
         \n}\n"
    );
}
