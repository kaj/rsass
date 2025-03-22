//! Tests auto-converted from "sass-spec/spec/core_functions/meta/inspect/number.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("number")
}

#[test]
fn unit() {
    assert_eq!(
        runner().ok(
            "@use \"sass:meta\";\
             \n// We explicitly don\'t test the inspect format for complex units. Their format\
             \n// isn\'t guaranteed by the spec, since they can\'t be written literally in Sass.\
             \n$result: meta.inspect(50px);\
             \na {\
             \n  value: $result;\
             \n  type: meta.type-of($result);\
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
        runner().ok("@use \"sass:meta\";\
             \n$result: meta.inspect(123.456);\
             \na {\
             \n  value: $result;\
             \n  type: meta.type-of($result);\
             \n}\n"),
        "a {\
         \n  value: 123.456;\
         \n  type: string;\
         \n}\n"
    );
}
