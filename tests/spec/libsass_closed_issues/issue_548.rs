//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_548.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".parent-sel-value {\
            \n  font-family: &;\
            \n  .parent-sel-interpolation {\
            \n    font-family: #{&};\
            \n     .parent-sel-value-concat {\
            \n        font-family: \"Current parent: \" + &;\
            \n     }\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ".parent-sel-value {\
        \n  font-family: .parent-sel-value;\
        \n}\
        \n.parent-sel-value .parent-sel-interpolation {\
        \n  font-family: .parent-sel-value .parent-sel-interpolation;\
        \n}\
        \n.parent-sel-value .parent-sel-interpolation .parent-sel-value-concat {\
        \n  font-family: \"Current parent: .parent-sel-value .parent-sel-interpolation .parent-sel-value-concat\";\
        \n}\
        \n"
    );
}
