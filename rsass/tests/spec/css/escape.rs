//! Tests auto-converted from "sass-spec/spec/css/escape.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("escape")
}

mod surrogate {
    use super::runner;

    #[test]
    fn leading() {
        assert_eq!(
            runner().ok("a {b: \\D800}\n"),
            "@charset \"UTF-8\";\
         \na {\
         \n  b: �;\
         \n}\n"
        );
    }
    #[test]
    fn trailing() {
        assert_eq!(
            runner().ok("a {b: \\DC00}\n"),
            "@charset \"UTF-8\";\
         \na {\
         \n  b: �;\
         \n}\n"
        );
    }
}
#[test]
fn too_high() {
    assert_eq!(
        runner().ok("// Regression test for sass/dart-sass#1030.\
             \na {b: \\110000}\n"),
        "@charset \"UTF-8\";\
         \na {\
         \n  b: �;\
         \n}\n"
    );
}
mod zero {
    use super::runner;

    #[test]
    fn alone() {
        assert_eq!(
            runner().ok("a {b: \\0}\n"),
            "a {\
         \n  b: \\0 ;\
         \n}\n"
        );
    }
    #[test]
    fn mid() {
        assert_eq!(
            runner().ok("a {b: x\\0y}\n"),
            "@charset \"UTF-8\";\
         \na {\
         \n  b: x�y;\
         \n}\n"
        );
    }
    #[test]
    fn start() {
        assert_eq!(
            runner().ok("a {b: \\0xyz}\n"),
            "@charset \"UTF-8\";\
         \na {\
         \n  b: �xyz;\
         \n}\n"
        );
    }
}
