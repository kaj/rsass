//! Tests auto-converted from "sass-spec/spec/libsass/precision/default.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "test {\r\
            \n  foo: 0.4999 round(0.4999);\r\
            \n  bar: 0.49999 round(0.49999);\r\
            \n  baz: 0.499999 round(0.499999);\r\
            \n  baz: 0.49999999999 round(0.49999999999);\r\
            \n}\r\
            \n"
        )
        .unwrap(),
        "test {\
        \n  foo: 0.4999 0;\
        \n  bar: 0.49999 0;\
        \n  baz: 0.499999 0;\
        \n  baz: 0.5 0;\
        \n}\
        \n"
    );
}
