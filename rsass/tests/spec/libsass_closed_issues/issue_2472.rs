//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2472.hrx"

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
         \n  direct: rgb(3.7745098039%, 7.5490196078%, 11.3235294118%);\
         \n  arg: rgb(3.7745098039%, 7.5490196078%, 11.3235294118%);\
         \n  call: rgb(3.7745098039%, 7.5490196078%, 11.3235294118%);\
         \n  function: rgb(3.7745098039%, 7.5490196078%, 11.3235294118%);\
         \n  function2: rgb(3.7745098039%, 7.5490196078%, 11.3235294118%);\
         \n}\n"
    );
}
