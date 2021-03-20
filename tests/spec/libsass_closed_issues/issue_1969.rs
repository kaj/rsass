//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1969.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$base-text-color: #666;\
            \n\
            \n@function calcNavbarTextColor ($base-text-color) {\
            \n        @return $base-text-color;\
            \n}\
            \n\
            \n$header-text-color: calcNavbarTextColor($base-text-color);\
            \n\
            \n.test_class {\
            \n        color: lighten($header-text-color, 20%);\
            \n}"
        )
        .unwrap(),
        ".test_class {\
        \n  color: #999999;\
        \n}\
        \n"
    );
}
