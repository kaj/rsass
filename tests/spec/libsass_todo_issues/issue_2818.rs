//! Tests auto-converted from "sass-spec/spec/libsass-todo-issues/issue_2818.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "$map: (\"lightness\": 10%, \"saturation\": 10%);\
            \n$base: call(get-function(\'scale-color\'), #dedede, $map...);\
            \ntest { color: $base; }\
            \n"
        )
        .unwrap(),
        "test {\
        \n  color: #e4dede;\
        \n}\
        \n"
    );
}
