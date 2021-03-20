//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1624.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@function foo($foo) {\
            \n  @return $foo;\
            \n}\
            \n\
            \n@function data($foo) {\
            \n  @return \'[data-\' + $foo + \']\';\
            \n}\
            \n\
            \n#{foo(foo)} {\
            \n  #{data(\'bar\')} {\
            \n    baz: bam;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "foo [data-bar] {\
        \n  baz: bam;\
        \n}\
        \n"
    );
}
