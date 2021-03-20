//! Tests auto-converted from "sass-spec/spec/non_conformant/sass_4_0/interpolation/after_hyphen.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".after-hyphen {\
            \n  // We want to treat interpolation as though it\'s a valid identifier character,\
            \n  // which means that if it comes after a hyphen or double-hyphen it should be\
            \n  // treated as part of an interpolated identifier that includes that hyphen.\
            \n  standalone-single: -#{foo};\
            \n  standalone-double: --#{foo};\
            \n\
            \n  // It also means that we shouldn\'t treat the hyphen as a subtraction\
            \n  // operation.\
            \n  adjacent-single: #{foo} -#{bar};\
            \n  adjacent-double: #{foo} --#{bar};\
            \n}\
            \n"
        )
        .unwrap(),
        ".after-hyphen {\
        \n  standalone-single: -foo;\
        \n  standalone-double: --foo;\
        \n  adjacent-single: foo -bar;\
        \n  adjacent-double: foo --bar;\
        \n}\
        \n"
    );
}
