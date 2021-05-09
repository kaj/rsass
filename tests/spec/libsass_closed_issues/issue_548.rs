//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_548.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            ".parent-sel-value {\
             \n  font-family: &;\
             \n  .parent-sel-interpolation {\
             \n    font-family: #{&};\
             \n     .parent-sel-value-concat {\
             \n        font-family: \"Current parent: \" + &;\
             \n     }\
             \n  }\
             \n}\n"
        ),
        ".parent-sel-value {\
         \n  font-family: .parent-sel-value;\
         \n}\
         \n.parent-sel-value .parent-sel-interpolation {\
         \n  font-family: .parent-sel-value .parent-sel-interpolation;\
         \n}\
         \n.parent-sel-value .parent-sel-interpolation .parent-sel-value-concat {\
         \n  font-family: \"Current parent: .parent-sel-value .parent-sel-interpolation .parent-sel-value-concat\";\
         \n}\n"
    );
}
