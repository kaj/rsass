//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1169"
#[allow(unused)]
use super::rsass;

mod error;

// From "sass-spec/spec/libsass-closed-issues/issue_1169/functioncall.hrx"
#[test]
#[ignore] // wrong result
fn functioncall() {
    assert_eq!(
        rsass(
            "$count: 0;\
            \n\
            \n@function counter() {\
            \n  $count: $count + 1 !global;\
            \n  @return $count;\
            \n}\
            \n\
            \n$map: (\
            \n  counter(): \'bar\',\
            \n  counter(): \'foo\',\
            \n);\
            \n\
            \n.foo {\
            \n  content: inspect($map);\
            \n}"
        )
        .unwrap(),
        ".foo {\
        \n  content: (1: \"bar\", 2: \"foo\");\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1169/interpolated.hrx"
#[test]
#[ignore] // wrong result
fn interpolated() {
    assert_eq!(
        rsass(
            "$map1: ( red: \'literal\', transparent: \'literal\' );\
            \n$map2: ( \'red\': \'quoted\', transparent: \'quoted\' );\
            \n$map3: ( #{re}#{d}: \'interpolated\', #{trans}#{parent}: \'quoted\' );\
            \n\
            \nfoo {\
            \n  content: inspect($map1);\
            \n  content: inspect($map2);\
            \n  content: inspect($map3);\
            \n}\
            \n\
            \n$merge1: map-merge($map1, $map2);\
            \n$merge2: map-merge($map1, $map3);\
            \n$merge3: map-merge($map2, $map3);\
            \n\
            \nbar {\
            \n  content: inspect($merge1);\
            \n  content: inspect($merge2);\
            \n  content: inspect($merge3);\
            \n}"
        )
        .unwrap(),
        "foo {\
        \n  content: (red: \"literal\", transparent: \"literal\");\
        \n  content: (\"red\": \"quoted\", transparent: \"quoted\");\
        \n  content: (red: \"interpolated\", transparent: \"quoted\");\
        \n}\
        \nbar {\
        \n  content: (red: \"literal\", transparent: \"quoted\", \"red\": \"quoted\");\
        \n  content: (red: \"literal\", transparent: \"literal\", red: \"interpolated\", transparent: \"quoted\");\
        \n  content: (\"red\": \"interpolated\", transparent: \"quoted\", transparent: \"quoted\");\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1169/simple.hrx"
#[test]
fn simple() {
    assert_eq!(
        rsass(
            "$map1: (\r\
            \n   red: \'bar\',\r\
            \n  \'red\': \'foo\',\r\
            \n);\r\
            \n\r\
            \n$map2: (\r\
            \n   red: \'bar\',\r\
            \n  \'red\': #{red},\r\
            \n);\r\
            \n\r\
            \n.foo {\r\
            \n  content: inspect($map1);\r\
            \n  content: inspect($map2);\r\
            \n}"
        )
        .unwrap(),
        ".foo {\
        \n  content: (red: \"bar\", \"red\": \"foo\");\
        \n  content: (red: \"bar\", \"red\": red);\
        \n}\
        \n"
    );
}
