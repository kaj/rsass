//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2261.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$seven: 7;\
            \n\
            \n.test {\
            \n\
            \n  equal-01: (7 == 7);\
            \n  equal-02: (\'7\' == \'7\');\
            \n  equal-03: (\'#{7}\' == \'#{7}\');\
            \n\
            \n  equal-04: (7 == \'7\');\
            \n  equal-05: (\'7\' == 7);\
            \n  equal-06: (7 == \'#{7}\');\
            \n  equal-07: (\'#{7}\' == 7);\
            \n  equal-08: (\'7\' == \'#{7}\');\
            \n  equal-09: (\'#{7}\' == \'7\');\
            \n\
            \n  equal-10: ($seven == 7);\
            \n  equal-11: ($seven == \'7\');\
            \n  equal-13: ($seven == \'#{7}\');\
            \n\
            \n  equal-14: (7 == $seven);\
            \n  equal-15: (\'7\' == $seven);\
            \n  equal-16: (\'#{7}\' == $seven);\
            \n\
            \n  equal-17: (\'#{$seven}\' == 7);\
            \n  equal-18: (\'#{$seven}\' == \'7\');\
            \n  equal-19: (\'#{$seven}\' == \'#{7}\');\
            \n\
            \n  equal-20: (7 == \'#{$seven}\');\
            \n  equal-21: (\'7\' == \'#{$seven}\');\
            \n  equal-22: (\'#{7}\' == \'#{$seven}\');\
            \n\
            \n  equal-23: (\'#{$seven}\' == $seven);\
            \n  equal-24: (\'#{$seven}\' == \'#{$seven}\');\
            \n\
            \n}"
        )
        .unwrap(),
        ".test {\
        \n  equal-01: true;\
        \n  equal-02: true;\
        \n  equal-03: true;\
        \n  equal-04: false;\
        \n  equal-05: false;\
        \n  equal-06: false;\
        \n  equal-07: false;\
        \n  equal-08: true;\
        \n  equal-09: true;\
        \n  equal-10: true;\
        \n  equal-11: false;\
        \n  equal-13: false;\
        \n  equal-14: true;\
        \n  equal-15: false;\
        \n  equal-16: false;\
        \n  equal-17: false;\
        \n  equal-18: true;\
        \n  equal-19: true;\
        \n  equal-20: false;\
        \n  equal-21: true;\
        \n  equal-22: true;\
        \n  equal-23: false;\
        \n  equal-24: true;\
        \n}\
        \n"
    );
}
