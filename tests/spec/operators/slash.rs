//! Tests auto-converted from "sass-spec/spec/operators/slash.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("slash")
        .mock_file("namespaced_variables/other.scss", "$a: 1;\n$b: 2;\n")
}

#[test]
fn namespaced_variables() {
    let runner = runner().with_cwd("namespaced_variables");
    assert_eq!(
        runner.ok("@use \"other\";\
             \na {b: other.$a / other.$b}\n"),
        "a {\
         \n  b: 0.5;\
         \n}\n"
    );
}
mod separator {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("separator")
    }

    mod calculation {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("calculation")
        }

        mod preserved {
            #[allow(unused)]
            fn runner() -> crate::TestRunner {
                super::runner().with_cwd("preserved")
            }

            #[test]
            fn both() {
                let runner = runner().with_cwd("both");
                assert_eq!(
                    runner.ok("a {b: calc(1px + 1%)/calc(2px + 2%)}\n"),
                    "a {\
         \n  b: calc(1px + 1%)/calc(2px + 2%);\
         \n}\n"
                );
            }
            #[test]
            fn left() {
                let runner = runner().with_cwd("left");
                assert_eq!(
                    runner.ok("a {b: calc(1px + 1%)/2}\n"),
                    "a {\
         \n  b: calc(1px + 1%)/2;\
         \n}\n"
                );
            }
            #[test]
            fn right() {
                let runner = runner().with_cwd("right");
                assert_eq!(
                    runner.ok("a {b: 2/calc(1px + 1%)}\n"),
                    "a {\
         \n  b: 2/calc(1px + 1%);\
         \n}\n"
                );
            }
        }
        mod simplified {
            #[allow(unused)]
            fn runner() -> crate::TestRunner {
                super::runner().with_cwd("simplified")
            }

            #[test]
            #[ignore] // wrong result
            fn both() {
                let runner = runner().with_cwd("both");
                assert_eq!(
                    runner.ok("a {b: calc(1)/calc(2)}\n"),
                    "a {\
         \n  b: 1/2;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn left() {
                let runner = runner().with_cwd("left");
                assert_eq!(
                    runner.ok("a {b: calc(1)/2}\n"),
                    "a {\
         \n  b: 1/2;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn right() {
                let runner = runner().with_cwd("right");
                assert_eq!(
                    runner.ok("a {b: 1/calc(2)}\n"),
                    "a {\
         \n  b: 1/2;\
         \n}\n"
                );
            }
        }
    }
}
