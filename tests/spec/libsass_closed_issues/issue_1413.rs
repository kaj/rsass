//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1413.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "div {\r\
            \n    foo: \'A\'#{B};\r\
            \n    foo: #{A}\'B\';\r\
            \n    foo: \'A\'#{B}\'C\';\r\
            \n    foo: #{A}\'B\'#{C};\r\
            \n    foo: A#{B}\'C\';\r\
            \n    foo: \'A\'#{B}C;\r\
            \n    foo: #{A}B\'C\';\r\
            \n    foo: \'A\'#{B}C\'D\';\r\
            \n    foo: \'A\'B#{C}D\'E\';\r\
            \n    foo: A\'B\'#{C}D\'E\';\r\
            \n    foo: #{A}\'B\'C\'D\'\'E\';\r\
            \n}\r\
            \n\r\
            \ndiv {\r\
            \n    foo: type-of(\'A\'#{B});\r\
            \n    foo: type-of(#{A}\'B\');\r\
            \n    foo: type-of(\'A\'#{B}\'C\');\r\
            \n    foo: type-of(#{A}\'B\'#{C});\r\
            \n    foo: type-of(A#{B}\'C\');\r\
            \n    foo: type-of(\'A\'#{B}C);\r\
            \n    foo: type-of(#{A}B\'C\');\r\
            \n    foo: type-of(\'A\'#{B}C\'D\');\r\
            \n    foo: type-of(\'A\'B#{C}D\'E\');\r\
            \n    foo: type-of(A\'B\'#{C}D\'E\');\r\
            \n    foo: type-of(#{A}\'B\'C\'D\'\'E\');\r\
            \n}\r\
            \n\r\
            \ndiv {\r\
            \n    foo: length(\'A\'#{B});\r\
            \n    foo: length(#{A}\'B\');\r\
            \n    foo: length(\'A\'#{B}\'C\');\r\
            \n    foo: length(#{A}\'B\'#{C});\r\
            \n    foo: length(A#{B}\'C\');\r\
            \n    foo: length(\'A\'#{B}C);\r\
            \n    foo: length(#{A}B\'C\');\r\
            \n    foo: length(\'A\'#{B}C\'D\');\r\
            \n    foo: length(\'A\'B#{C}D\'E\');\r\
            \n    foo: length(A\'B\'#{C}D\'E\');\r\
            \n    foo: length(#{A}\'B\'C\'D\'\'E\');\r\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  foo: \"A\" B;\
        \n  foo: A \"B\";\
        \n  foo: \"A\" B \"C\";\
        \n  foo: A \"B\" C;\
        \n  foo: AB \"C\";\
        \n  foo: \"A\" BC;\
        \n  foo: AB \"C\";\
        \n  foo: \"A\" BC \"D\";\
        \n  foo: \"A\" BCD \"E\";\
        \n  foo: A \"B\" CD \"E\";\
        \n  foo: A \"B\" C \"D\" \"E\";\
        \n}\
        \ndiv {\
        \n  foo: list;\
        \n  foo: list;\
        \n  foo: list;\
        \n  foo: list;\
        \n  foo: list;\
        \n  foo: list;\
        \n  foo: list;\
        \n  foo: list;\
        \n  foo: list;\
        \n  foo: list;\
        \n  foo: list;\
        \n}\
        \ndiv {\
        \n  foo: 2;\
        \n  foo: 2;\
        \n  foo: 3;\
        \n  foo: 3;\
        \n  foo: 2;\
        \n  foo: 2;\
        \n  foo: 2;\
        \n  foo: 3;\
        \n  foo: 3;\
        \n  foo: 4;\
        \n  foo: 5;\
        \n}\
        \n"
    );
}
