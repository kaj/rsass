//! Tests auto-converted from "sass-spec/spec/values/identifiers/escape/script.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("script")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "@use \"sass:string\";\
             \n// SassScript functions operate on the normalized form of the identifier.\
             \n.script {\
             \n  unescaped-str-length: string.length(ax) string.length(\\61x) string.length(\\00061 x);\
             \n  escaped-str-length: string.length(\\1Ax) string.length(\\0001A x);\n\
             \n  unescaped-slice: string.slice(xaz, 2, 2) string.slice(x\\61z, 2, 2) string.slice(x\\00061 z, 2, 2);\
             \n  escaped-slice: string.slice(x\\1Az, 2, 5) string.slice(x\\0001A z, 2, 5);\n\
             \n  unescaped-quote: string.quote(ax) string.quote(\\61x) string.quote(\\00061 x);\
             \n  escaped-quote: string.quote(\\1Ax) string.quote(\\0001A x);\
             \n}\n"
        ),
        ".script {\
         \n  unescaped-str-length: 2 2 2;\
         \n  escaped-str-length: 5 5;\
         \n  unescaped-slice: a a a;\
         \n  escaped-slice: \\1a  \\1a ;\
         \n  unescaped-quote: \"ax\" \"ax\" \"ax\";\
         \n  escaped-quote: \"\\\\1a x\" \"\\\\1a x\";\
         \n}\n"
    );
}
