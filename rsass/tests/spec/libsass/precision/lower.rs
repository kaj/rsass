//! Tests auto-converted from "sass-spec/spec/libsass/precision/lower.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("lower")
}

#[test]
#[ignore] // wrong result
fn test() {
    let runner = runner().set_precision(4);
    assert_eq!(
        runner.ok("test {\r\
             \n  foo: 0.4999 round(0.4999);\r\
             \n  bar: 0.49999 round(0.49999);\r\
             \n  baz: 0.499999 round(0.499999);\r\
             \n}"),
        "test {\
         \n  foo: 0.4999 0;\
         \n  bar: 0.49999 0;\
         \n  baz: 0.499999 0;\
         \n}\n"
    );
}
