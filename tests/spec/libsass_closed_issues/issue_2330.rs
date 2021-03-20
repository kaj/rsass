//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2330.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@function test () {\r\
            \n  $m: ();\r\
            \n  $abc: (a b c d e f g h i j k);\r\
            \n\r\
            \n  @for $index from 1 through length($abc) {;\r\
            \n    $m: map-merge($m, (nth($abc, $index):$index) );\r\
            \n  }\r\
            \n\r\
            \n  @return $m;\r\
            \n}\r\
            \n\r\
            \ntest {\r\
            \n  content: inspect(test());\r\
            \n}"
        )
        .unwrap(),
        "test {\
        \n  content: (a: 1, b: 2, c: 3, d: 4, e: 5, f: 6, g: 7, h: 8, i: 9, j: 10, k: 11);\
        \n}\
        \n"
    );
}
