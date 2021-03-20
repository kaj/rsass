//! Tests auto-converted from "sass-spec/spec/libsass/selector-functions/simple-selector.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\r\
            \n  test-01: simple-selectors(\".foo.bar\");\r\
            \n  test-02: simple-selectors(\".foo.bar.baz\");\r\
            \n}"
        )
        .unwrap(),
        "foo {\
        \n  test-01: .foo, .bar;\
        \n  test-02: .foo, .bar, .baz;\
        \n}\
        \n"
    );
}
