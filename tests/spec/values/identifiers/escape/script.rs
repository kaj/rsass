//! Tests auto-converted from "sass-spec/spec/values/identifiers/escape/script.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "// SassScript functions operate on the normalized form of the identifier.\
            \n.script {\
            \n  unescaped-str-length: str-length(ax) str-length(\\61x) str-length(\\00061 x);\
            \n  escaped-str-length: str-length(\\1Ax) str-length(\\0001A x);\
            \n\
            \n  unescaped-slice: str-slice(xaz, 2, 2) str-slice(x\\61z, 2, 2) str-slice(x\\00061 z, 2, 2);\
            \n  escaped-slice: str-slice(x\\1Az, 2, 5) str-slice(x\\0001A z, 2, 5);\
            \n\
            \n  unescaped-quote: quote(ax) quote(\\61x) quote(\\00061 x);\
            \n  escaped-quote: quote(\\1Ax) quote(\\0001A x);\
            \n}\
            \n"
        )
        .unwrap(),
        ".script {\
        \n  unescaped-str-length: 2 2 2;\
        \n  escaped-str-length: 5 5;\
        \n  unescaped-slice: a a a;\
        \n  escaped-slice: \\1a  \\1a ;\
        \n  unescaped-quote: \"ax\" \"ax\" \"ax\";\
        \n  escaped-quote: \"\\\\1a x\" \"\\\\1a x\";\
        \n}\
        \n"
    );
}
