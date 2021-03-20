//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2472.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@function dark(\r\
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
            \n}"
        )
        .unwrap(),
        ".single {\
        \n  direct: #0a131d;\
        \n  arg: #0a131d;\
        \n  call: #0a131d;\
        \n  function: #0a131d;\
        \n  function2: #0a131d;\
        \n}\
        \n"
    );
}
