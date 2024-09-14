//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2472.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2472")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("@function dark(\r\
             \n  $color,\r\
             \n  $args...\r\
             \n) {\r\
             \n  @return call(\'darken\', $color, $args...);\r\
             \n}\r\
             \n\r\
             \n@function dark2(\r\
             \n  $args...\r\
             \n) {\r\
             \n  @return call(\'darken\', $args...);\r\
             \n}\r\
             \n\r\
             \n$arg: join((), 5%);\r\
             \n\r\
             \n.single {\r\
             \n  direct: darken(#102030, 5%);\r\
             \n  arg: darken(#102030, $arg...);\r\
             \n  call: call(\'darken\', #102030, $arg...);\r\
             \n  function: dark(#102030, 5%);\r\
             \n  function2: dark2(#102030, 5%);\r\
             \n}"),
        ".single {\
         \n  direct: rgb(9.625, 19.25, 28.875);\
         \n  arg: rgb(9.625, 19.25, 28.875);\
         \n  call: rgb(9.625, 19.25, 28.875);\
         \n  function: rgb(9.625, 19.25, 28.875);\
         \n  function2: rgb(9.625, 19.25, 28.875);\
         \n}\n"
    );
}
