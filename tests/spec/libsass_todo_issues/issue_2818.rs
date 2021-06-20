//! Tests auto-converted from "sass-spec/spec/libsass-todo-issues/issue_2818.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$map: (\"lightness\": 10%, \"saturation\": 10%);\
             \n$base: call(get-function(\'scale-color\'), #dedede, $map...);\
             \ntest { color: $base; }\n"),
        "test {\
         \n  color: #e4dede;\
         \n}\n"
    );
}
