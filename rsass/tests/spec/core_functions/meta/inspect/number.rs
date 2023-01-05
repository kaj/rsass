//! Tests auto-converted from "sass-spec/spec/core_functions/meta/inspect/number.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("number")
}

#[test]
fn unit() {
    assert_eq!(
        runner().ok(
            "// We explicitly don\'t test the inspect format for complex units. Their format\
             \n// isn\'t guaranteed by the spec, since they can\'t be written literally in Sass.\
             \n$result: inspect(50px);\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"
        ),
        "a {\
         \n  value: 50px;\
         \n  type: string;\
         \n}\n"
    );
}
#[test]
fn unitless() {
    assert_eq!(
        runner().ok("$result: inspect(123.456);\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
        "a {\
         \n  value: 123.456;\
         \n  type: string;\
         \n}\n"
    );
}
