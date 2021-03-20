//! Tests auto-converted from "sass-spec/spec/css/custom_properties/name_interpolation.hrx"

#[test]
fn nested_properties() {
    assert_eq!(
        crate::rsass(
            "// Regression test for sass/dart-sass#1095\
            \na {\
            \n  #{--b}: {c: d}\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  --b-c: d;\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // wrong result
fn non_conformant() {
    assert_eq!(
        crate::rsass(
            "// TODO: rewrite these test cases to follow the style guide.\
            \n\
            \n.name-interpolation {\
            \n  // If the entire name is interpolated, SassScript is allowed on the\
            \n  // right-hand side because we don\'t know it\'s a custom property at parse time.\
            \n  #{--entire}: 1 + 2;\
            \n\
            \n  // Same if the first hyphen is interpolated.\
            \n  -#{-first-hyphen}: 1 + 2;\
            \n\
            \n  // But if the name is interpolated, the right-hand side is static.\
            \n  --#{only-name}: 1 + 2;\
            \n  // However, interpolation is still allowed on the right-hand side.\
            \n  --#{only-name-interp-value}: #{1 + 2};\
            \n\
            \n  // The name can also be partially interpolated.\
            \n  --#{initial}-interp: 1 + 2;\
            \n  --midd#{le-int}erp: 1 + 2;\
            \n  --final-#{interp}: 1 + 2;\
            \n  --#{doub}le-int#{erp}: 1 + 2;\
            \n}\
            \n"
        )
        .unwrap(),
        ".name-interpolation {\
        \n  --entire: 3;\
        \n  --first-hyphen: 3;\
        \n  --only-name: 1 + 2;\
        \n  --only-name-interp-value: 3;\
        \n  --initial-interp: 1 + 2;\
        \n  --middle-interp: 1 + 2;\
        \n  --final-interp: 1 + 2;\
        \n  --double-interp: 1 + 2;\
        \n}\
        \n"
    );
}
