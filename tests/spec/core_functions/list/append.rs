//! Tests auto-converted from "sass-spec/spec/core_functions/list/append.hrx"

#[test]
fn auto() {
    assert_eq!(
        crate::rsass(
            "a {b: append(c d, e, $separator: auto)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: c d e;\
        \n}\
        \n"
    );
}
#[test]
fn bracketed() {
    assert_eq!(
        crate::rsass(
            "a {b: append([], 1)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: [1];\
        \n}\
        \n"
    );
}
mod comma {
    #[test]
    fn default() {
        assert_eq!(
            crate::rsass(
                "a {b: append((1, 2, 3), 4)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 1, 2, 3, 4;\
        \n}\
        \n"
        );
    }
    #[test]
    fn overridden() {
        assert_eq!(
            crate::rsass(
                "a {b: append(1 2 3, 4, $separator: comma)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 1, 2, 3, 4;\
        \n}\
        \n"
        );
    }
}
mod empty {
    #[test]
    fn comma() {
        assert_eq!(
            crate::rsass(
                "@import \"core_functions/list/utils\";\
            \n\
            \n$result: append($empty-comma-list, 1);\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n  separator: real-separator($result);\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  value: 1;\
        \n  type: list;\
        \n  separator: comma;\
        \n}\
        \n"
        );
    }
    #[test]
    fn space() {
        assert_eq!(
            crate::rsass(
                "@import \"core_functions/list/utils\";\
            \n\
            \n$result: append($empty-space-list, 1);\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n  separator: real-separator($result);\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  value: 1;\
        \n  type: list;\
        \n  separator: space;\
        \n}\
        \n"
        );
    }
    #[test]
    fn undecided() {
        assert_eq!(
            crate::rsass(
                "@import \"core_functions/list/utils\";\
            \n\
            \n$result: append((), 1);\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n  separator: real-separator($result);\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  value: 1;\
        \n  type: list;\
        \n  separator: space;\
        \n}\
        \n"
        );
    }
}
mod error {

    // Ignoring "too_few_args", error tests are not supported yet.

    // Ignoring "too_many_args", error tests are not supported yet.
    mod test_type {

        // Ignoring "separator", error tests are not supported yet.
    }

    // Ignoring "unknown_separator", error tests are not supported yet.
}
mod map {
    #[test]
    fn empty() {
        assert_eq!(
            crate::rsass(
                "@import \"core_functions/list/utils\";\
            \n\
            \n$result: append($empty-map, 1);\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n  separator: real-separator($result);\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  value: 1;\
        \n  type: list;\
        \n  separator: space;\
        \n}\
        \n"
        );
    }
    #[test]
    fn non_empty() {
        assert_eq!(
            crate::rsass(
                "a {b: append((c: d, e: f), g)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: c d, e f, g;\
        \n}\
        \n"
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: append($list: c d, $val: e, $separator: comma)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: c, d, e;\
        \n}\
        \n"
    );
}
#[test]
fn non_list() {
    assert_eq!(
        crate::rsass(
            "a {b: append(c, d)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: c d;\
        \n}\
        \n"
    );
}
mod single {
    #[test]
    fn comma() {
        assert_eq!(
            crate::rsass(
                "a {b: append((1,), 2)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 1, 2;\
        \n}\
        \n"
        );
    }
    #[test]
    fn space() {
        assert_eq!(
            crate::rsass(
                "@import \"core_functions/list/utils\";\
            \na {b: append(with-separator(1, space), 2)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 1 2;\
        \n}\
        \n"
        );
    }
    #[test]
    fn undecided() {
        assert_eq!(
            crate::rsass(
                "a {b: append(1, 2)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 1 2;\
        \n}\
        \n"
        );
    }
}
mod space {
    #[test]
    fn default() {
        assert_eq!(
            crate::rsass(
                "a {b: append(1 2 3, 4)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 1 2 3 4;\
        \n}\
        \n"
        );
    }
    #[test]
    fn overridden() {
        assert_eq!(
            crate::rsass(
                "a {b: append((1, 2, 3), 4, $separator: space)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 1 2 3 4;\
        \n}\
        \n"
        );
    }
}
