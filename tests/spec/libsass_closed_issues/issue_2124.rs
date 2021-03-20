//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2124.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "test {\r\
            \n  test-01: #{if(&, \'true\', \'false\')};\r\
            \n  test-01: #{if(0, \'true\', \'false\')};\r\
            \n  test-01: #{if(\'\', \'true\', \'false\')};\r\
            \n  test-01: #{if(\'0\', \'true\', \'false\')};\r\
            \n  test-01: #{if(null, \'true\', \'false\')};\r\
            \n  test-01: #{if(false, \'true\', \'false\')};\r\
            \n}\r\
            \n\r\
            \n#{if(&, \'has-parent\', \'parentless\')} {\r\
            \n  test: parent;\r\
            \n}\r\
            \n\r\
            \n@mixin with-js() {\r\
            \n  .js:root #{if(&, \'&\', \'\')} {\r\
            \n    @content;\r\
            \n  }\r\
            \n}\r\
            \n\r\
            \n@include with-js() {\r\
            \n  .bou {\r\
            \n    content: \'bar\';\r\
            \n  }\r\
            \n}\r\
            \n\r\
            \n.bou {\r\
            \n  @include with-js() {\r\
            \n    .bar {\r\
            \n      content: \'baz\';\r\
            \n    }\r\
            \n  }\r\
            \n}\r\
            \n\r\
            \n"
        )
        .unwrap(),
        "test {\
        \n  test-01: true;\
        \n  test-01: true;\
        \n  test-01: true;\
        \n  test-01: true;\
        \n  test-01: false;\
        \n  test-01: false;\
        \n}\
        \nparentless {\
        \n  test: parent;\
        \n}\
        \n.js:root .bou {\
        \n  content: \"bar\";\
        \n}\
        \n.js:root .bou .bar {\
        \n  content: \"baz\";\
        \n}\
        \n"
    );
}
