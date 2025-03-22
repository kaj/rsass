//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1269.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1269")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\n\
             \n@function push($list, $items...) {\
             \n  @return list.join($list, $items, $separator: auto);\
             \n}\n\
             \n.test {\
             \n  $list: push(1 2 3, 4, 5);\
             \n  list: meta.inspect($list);\
             \n  value: list.nth($list, 4);\
             \n}"),
        ".test {\
         \n  list: 1 2 3 4 5;\
         \n  value: 4;\
         \n}\n"
    );
}
