//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2330.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2330")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "@use \"sass:list\";\
             \n@use \"sass:map\";\
             \n@use \"sass:meta\";\n\
             \n@function test () {\
             \n  $m: ();\
             \n  $abc: (a b c d e f g h i j k);\n\
             \n  @for $index from 1 through list.length($abc) {;\
             \n    $m: map.merge($m, (list.nth($abc, $index):$index) );\
             \n  }\n\
             \n  @return $m;\
             \n}\n\
             \ntest {\
             \n  content: meta.inspect(test());\
             \n}"
        ),
        "test {\
         \n  content: (a: 1, b: 2, c: 3, d: 4, e: 5, f: 6, g: 7, h: 8, i: 9, j: 10, k: 11);\
         \n}\n"
    );
}
