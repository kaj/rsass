//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_674.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_674")
}

#[test]
fn test() {
    assert_eq!(
        runner().err(
            "\
             \n$base-path:\'../images/\';\
             \n$base-attr:\'data-\';\n\
             \n@function url($src, $path:\'\'){\
             \n  @return unquote(\'url(\'+$base-path + $path+ $src +\')\');\
             \n}\
             \n@function url2($src, $path:\'\'){\
             \n  @return unquote(\'url(\'+ $base-path + $path+ $src +\')\');\
             \n}\
             \n@function attr($arg1, $arg2:\'\'){\
             \n  @return unquote(\'attr(\'+$base-attr + $arg1 + $arg2 +\')\');\
             \n}\n\
             \ndiv {\
             \n    background: url(\'image.png\');\
             \n    background: url(\'image.png\',\'img/\');\
             \n    background: url2(\'image.png\',\'img/\');\n\
             \n  &:after {\
             \n    content: attr(value);\
             \n    content: attr(value, -extra);\
             \n    content: url(\'icon.png\');\
             \n    content: url(\'icon.png\',\'gfx/\');\
             \n    content: url2(\'icon.png\',\'gfx/\');\
             \n  }\
             \n}\n"
        ),
        "Error: Invalid function name.\
         \n  ,\
         \n5 | @function url($src, $path:\'\'){\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 5:1  root stylesheet",
    );
}
