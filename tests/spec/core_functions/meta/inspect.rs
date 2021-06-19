//! Tests auto-converted from "sass-spec/spec/core_functions/meta/inspect.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .mock_file("color/generated/_utils.scss", "/// Returns a copy of `$color` that doesn't have color-literal metadata\n/// associated with it.\n@function generated-color($color) {\n  // This doesn't change the value of `$color` at all, but it does construct a\n  // new object.\n  @return scale-color($color, $blue: 0%);\n}\n")
}

mod boolean {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("boolean")
    }

    #[test]
    fn test_false() {
        let runner = runner().with_cwd("false");
        assert_eq!(
            runner.ok("$result: inspect(false);\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
            "a {\
         \n  value: false;\
         \n  type: string;\
         \n}\n"
        );
    }
    #[test]
    fn test_true() {
        let runner = runner().with_cwd("true");
        assert_eq!(
            runner.ok("$result: inspect(true);\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
            "a {\
         \n  value: true;\
         \n  type: string;\
         \n}\n"
        );
    }
}
mod color {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("color")
    }

    mod generated {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("generated")
        }

        #[test]
        fn alpha() {
            let runner = runner().with_cwd("alpha");
            assert_eq!(
                runner.ok("$result: inspect(rgba(1, 2, 3, 0.4));\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                "a {\
         \n  value: rgba(1, 2, 3, 0.4);\
         \n  type: string;\
         \n}\n"
            );
        }
        #[test]
        fn long_hex() {
            let runner = runner().with_cwd("long_hex");
            assert_eq!(
                runner.ok("@import \"../utils\";\
             \n$result: inspect(generated-color(#abcdef));\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                "a {\
         \n  value: #abcdef;\
         \n  type: string;\
         \n}\n"
            );
        }
        #[test]
        fn named() {
            let runner = runner().with_cwd("named");
            assert_eq!(
                runner.ok("@import \"../utils\";\
             \n$result: inspect(generated-color(#00f));\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                "a {\
         \n  value: blue;\
         \n  type: string;\
         \n}\n"
            );
        }
        #[test]
        fn short_hex() {
            let runner = runner().with_cwd("short_hex");
            assert_eq!(
                runner.ok("@import \"../utils\";\
             \n$result: inspect(generated-color(#abc));\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                "a {\
         \n  value: #aabbcc;\
         \n  type: string;\
         \n}\n"
            );
        }
        #[test]
        fn transparent() {
            let runner = runner().with_cwd("transparent");
            assert_eq!(
                runner.ok("@import \"../utils\";\
             \n$result: inspect(generated-color(transparent));\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                "a {\
         \n  value: rgba(0, 0, 0, 0);\
         \n  type: string;\
         \n}\n"
            );
        }
    }
    mod literal {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("literal")
        }

        #[test]
        fn long_hex() {
            let runner = runner().with_cwd("long_hex");
            assert_eq!(
                runner.ok("$result: inspect(#0000ff);\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                "a {\
         \n  value: #0000ff;\
         \n  type: string;\
         \n}\n"
            );
        }
        #[test]
        fn named() {
            let runner = runner().with_cwd("named");
            assert_eq!(
                runner.ok("$result: inspect(blue);\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                "a {\
         \n  value: blue;\
         \n  type: string;\
         \n}\n"
            );
        }
        #[test]
        fn short_hex() {
            let runner = runner().with_cwd("short_hex");
            assert_eq!(
                runner.ok("$result: inspect(#00f);\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                "a {\
         \n  value: #00f;\
         \n  type: string;\
         \n}\n"
            );
        }
        #[test]
        fn transparent() {
            let runner = runner().with_cwd("transparent");
            assert_eq!(
                runner.ok("$result: inspect(transparent);\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                "a {\
         \n  value: transparent;\
         \n  type: string;\
         \n}\n"
            );
        }
    }
}
mod error {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("error")
    }

    #[test]
    fn too_few_args() {
        let runner = runner().with_cwd("too_few_args");
        assert_eq!(
            runner.err("a {a: inspect()}\n"),
            "Error: Missing argument $value.\
         \n  ,--> input.scss\
         \n1 | a {a: inspect()}\
         \n  |       ^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function inspect($value) {\
         \n  |           =============== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        let runner = runner().with_cwd("too_many_args");
        assert_eq!(
            runner.err("a {a: inspect(1, 2)}\n"),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n1 | a {a: inspect(1, 2)}\
         \n  |       ^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function inspect($value) {\
         \n  |           =============== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
#[test]
fn function() {
    let runner = runner().with_cwd("function");
    assert_eq!(
        runner.ok("$result: inspect(get-function(\"get-function\"));\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
        "a {\
         \n  value: get-function(\"get-function\");\
         \n  type: string;\
         \n}\n"
    );
}
mod list {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("list")
    }

    #[test]
    fn bracketed() {
        let runner = runner().with_cwd("bracketed");
        assert_eq!(
            runner.ok("$result: inspect([1, 2, 3]);\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
            "a {\
         \n  value: [1, 2, 3];\
         \n  type: string;\
         \n}\n"
        );
    }
    #[test]
    fn comma() {
        let runner = runner().with_cwd("comma");
        assert_eq!(
            runner.ok("$result: inspect((1, 2, 3));\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
            "a {\
         \n  value: 1, 2, 3;\
         \n  type: string;\
         \n}\n"
        );
    }
    #[test]
    fn empty() {
        let runner = runner().with_cwd("empty");
        assert_eq!(
            runner.ok("$result: inspect(());\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
            "a {\
         \n  value: ();\
         \n  type: string;\
         \n}\n"
        );
    }
    mod nested {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("nested")
        }

        mod bracketed {
            #[allow(unused)]
            fn runner() -> crate::TestRunner {
                super::runner().with_cwd("bracketed")
            }

            mod in_comma {
                #[allow(unused)]
                fn runner() -> crate::TestRunner {
                    super::runner().with_cwd("in_comma")
                }

                #[test]
                fn bracketed() {
                    let runner = runner().with_cwd("bracketed");
                    assert_eq!(
                        runner.ok("$result: inspect([[1, 2], [3, 4]]);\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                        "a {\
         \n  value: [[1, 2], [3, 4]];\
         \n  type: string;\
         \n}\n"
                    );
                }
                #[test]
                fn unbracketed() {
                    let runner = runner().with_cwd("unbracketed");
                    assert_eq!(
                        runner.ok("$result: inspect(((1, 2), (3, 4)));\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                        "a {\
         \n  value: (1, 2), (3, 4);\
         \n  type: string;\
         \n}\n"
                    );
                }
            }
            mod in_slash {
                #[allow(unused)]
                fn runner() -> crate::TestRunner {
                    super::runner().with_cwd("in_slash")
                }

                #[test]
                fn bracketed() {
                    let runner = runner().with_cwd("bracketed");
                    assert_eq!(
        runner.ok(
            "$result: inspect(join([[1, 2], [3, 4]], (), $separator: slash));\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"
        ),
        "a {\
         \n  value: [[1, 2] / [3, 4]];\
         \n  type: string;\
         \n}\n"
    );
                }
                #[test]
                fn unbracketed() {
                    let runner = runner().with_cwd("unbracketed");
                    assert_eq!(
                        runner.ok("@use \"sass:list\";\
             \n$result: inspect(list.slash([1, 2], [3, 4]));\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                        "a {\
         \n  value: [1, 2] / [3, 4];\
         \n  type: string;\
         \n}\n"
                    );
                }
            }
            mod in_space {
                #[allow(unused)]
                fn runner() -> crate::TestRunner {
                    super::runner().with_cwd("in_space")
                }

                #[test]
                fn bracketed() {
                    let runner = runner().with_cwd("bracketed");
                    assert_eq!(
                        runner.ok("$result: inspect([[1, 2] [3, 4]]);\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                        "a {\
         \n  value: [[1, 2] [3, 4]];\
         \n  type: string;\
         \n}\n"
                    );
                }
                #[test]
                fn unbracketed() {
                    let runner = runner().with_cwd("unbracketed");
                    assert_eq!(
                        runner.ok("$result: inspect([1, 2] [3, 4]);\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                        "a {\
         \n  value: [1, 2] [3, 4];\
         \n  type: string;\
         \n}\n"
                    );
                }
            }
        }
        mod comma {
            #[allow(unused)]
            fn runner() -> crate::TestRunner {
                super::runner().with_cwd("comma")
            }

            mod in_comma {
                #[allow(unused)]
                fn runner() -> crate::TestRunner {
                    super::runner().with_cwd("in_comma")
                }

                #[test]
                fn bracketed() {
                    let runner = runner().with_cwd("bracketed");
                    assert_eq!(
                        runner.ok("$result: inspect([(1, 2), (3, 4)]);\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                        "a {\
         \n  value: [(1, 2), (3, 4)];\
         \n  type: string;\
         \n}\n"
                    );
                }
                #[test]
                fn unbracketed() {
                    let runner = runner().with_cwd("unbracketed");
                    assert_eq!(
                        runner.ok("$result: inspect(((1, 2), (3, 4)));\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                        "a {\
         \n  value: (1, 2), (3, 4);\
         \n  type: string;\
         \n}\n"
                    );
                }
            }
            mod in_slash {
                #[allow(unused)]
                fn runner() -> crate::TestRunner {
                    super::runner().with_cwd("in_slash")
                }

                #[test]
                fn bracketed() {
                    let runner = runner().with_cwd("bracketed");
                    assert_eq!(
        runner.ok(
            "$result: inspect(join([(1, 2), (3, 4)], (), $separator: slash));\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"
        ),
        "a {\
         \n  value: [(1, 2) / (3, 4)];\
         \n  type: string;\
         \n}\n"
    );
                }
                #[test]
                fn unbracketed() {
                    let runner = runner().with_cwd("unbracketed");
                    assert_eq!(
                        runner.ok("@use \"sass:list\";\
             \n$result: inspect(list.slash((1, 2), (3, 4)));\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                        "a {\
         \n  value: (1, 2) / (3, 4);\
         \n  type: string;\
         \n}\n"
                    );
                }
            }
            mod in_space {
                #[allow(unused)]
                fn runner() -> crate::TestRunner {
                    super::runner().with_cwd("in_space")
                }

                #[test]
                fn bracketed() {
                    let runner = runner().with_cwd("bracketed");
                    assert_eq!(
                        runner.ok("$result: inspect([(1, 2) (3, 4)]);\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                        "a {\
         \n  value: [(1, 2) (3, 4)];\
         \n  type: string;\
         \n}\n"
                    );
                }
                #[test]
                fn unbracketed() {
                    let runner = runner().with_cwd("unbracketed");
                    assert_eq!(
                        runner.ok("$result: inspect((1, 2) (3, 4));\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                        "a {\
         \n  value: (1, 2) (3, 4);\
         \n  type: string;\
         \n}\n"
                    );
                }
            }
        }
        mod empty {
            #[allow(unused)]
            fn runner() -> crate::TestRunner {
                super::runner().with_cwd("empty")
            }

            mod in_comma {
                #[allow(unused)]
                fn runner() -> crate::TestRunner {
                    super::runner().with_cwd("in_comma")
                }

                #[test]
                fn bracketed() {
                    let runner = runner().with_cwd("bracketed");
                    assert_eq!(
                        runner.ok("$result: inspect([(), ()]);\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                        "a {\
         \n  value: [(), ()];\
         \n  type: string;\
         \n}\n"
                    );
                }
                #[test]
                fn unbracketed() {
                    let runner = runner().with_cwd("unbracketed");
                    assert_eq!(
                        runner.ok("$result: inspect(((), ()));\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                        "a {\
         \n  value: (), ();\
         \n  type: string;\
         \n}\n"
                    );
                }
            }
            mod in_slash {
                #[allow(unused)]
                fn runner() -> crate::TestRunner {
                    super::runner().with_cwd("in_slash")
                }

                #[test]
                fn bracketed() {
                    let runner = runner().with_cwd("bracketed");
                    assert_eq!(
        runner.ok(
            "$result: inspect(join([(), ()], (), $separator: slash));\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"
        ),
        "a {\
         \n  value: [() / ()];\
         \n  type: string;\
         \n}\n"
    );
                }
                #[test]
                fn unbracketed() {
                    let runner = runner().with_cwd("unbracketed");
                    assert_eq!(
                        runner.ok("@use \"sass:list\";\
             \n$result: inspect(list.slash((), ()));\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                        "a {\
         \n  value: () / ();\
         \n  type: string;\
         \n}\n"
                    );
                }
            }
            mod in_space {
                #[allow(unused)]
                fn runner() -> crate::TestRunner {
                    super::runner().with_cwd("in_space")
                }

                #[test]
                fn bracketed() {
                    let runner = runner().with_cwd("bracketed");
                    assert_eq!(
                        runner.ok("$result: inspect([() ()]);\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                        "a {\
         \n  value: [() ()];\
         \n  type: string;\
         \n}\n"
                    );
                }
                #[test]
                fn unbracketed() {
                    let runner = runner().with_cwd("unbracketed");
                    assert_eq!(
                        runner.ok("$result: inspect(() ());\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                        "a {\
         \n  value: () ();\
         \n  type: string;\
         \n}\n"
                    );
                }
            }
        }
        mod space {
            #[allow(unused)]
            fn runner() -> crate::TestRunner {
                super::runner().with_cwd("space")
            }

            mod in_comma {
                #[allow(unused)]
                fn runner() -> crate::TestRunner {
                    super::runner().with_cwd("in_comma")
                }

                #[test]
                fn bracketed() {
                    let runner = runner().with_cwd("bracketed");
                    assert_eq!(
                        runner.ok("$result: inspect([1 2, 3 4]);\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                        "a {\
         \n  value: [1 2, 3 4];\
         \n  type: string;\
         \n}\n"
                    );
                }
                #[test]
                fn unbracketed() {
                    let runner = runner().with_cwd("unbracketed");
                    assert_eq!(
                        runner.ok("$result: inspect((1 2, 3 4));\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                        "a {\
         \n  value: 1 2, 3 4;\
         \n  type: string;\
         \n}\n"
                    );
                }
            }
            mod in_slash {
                #[allow(unused)]
                fn runner() -> crate::TestRunner {
                    super::runner().with_cwd("in_slash")
                }

                #[test]
                fn bracketed() {
                    let runner = runner().with_cwd("bracketed");
                    assert_eq!(
        runner.ok(
            "$result: inspect(join([1 2, 3 4], (), $separator: slash));\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"
        ),
        "a {\
         \n  value: [1 2 / 3 4];\
         \n  type: string;\
         \n}\n"
    );
                }
                #[test]
                fn unbracketed() {
                    let runner = runner().with_cwd("unbracketed");
                    assert_eq!(
                        runner.ok("@use \"sass:list\";\
             \n$result: inspect(list.slash(1 2, 3 4));\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                        "a {\
         \n  value: 1 2 / 3 4;\
         \n  type: string;\
         \n}\n"
                    );
                }
            }
            mod in_space {
                #[allow(unused)]
                fn runner() -> crate::TestRunner {
                    super::runner().with_cwd("in_space")
                }

                #[test]
                fn bracketed() {
                    let runner = runner().with_cwd("bracketed");
                    assert_eq!(
                        runner.ok("$result: inspect([(1 2) (3 4)]);\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                        "a {\
         \n  value: [(1 2) (3 4)];\
         \n  type: string;\
         \n}\n"
                    );
                }
                #[test]
                fn unbracketed() {
                    let runner = runner().with_cwd("unbracketed");
                    assert_eq!(
                        runner.ok("$result: inspect((1 2) (3 4));\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                        "a {\
         \n  value: (1 2) (3 4);\
         \n  type: string;\
         \n}\n"
                    );
                }
            }
        }
    }
    mod single {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("single")
        }

        mod bracketed {
            #[allow(unused)]
            fn runner() -> crate::TestRunner {
                super::runner().with_cwd("bracketed")
            }

            #[test]
            fn comma() {
                let runner = runner().with_cwd("comma");
                assert_eq!(
                    runner.ok("$result: inspect([1,]);\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                    "a {\
         \n  value: [1,];\
         \n  type: string;\
         \n}\n"
                );
            }
            #[test]
            fn undecided() {
                let runner = runner().with_cwd("undecided");
                assert_eq!(
                    runner.ok("$result: inspect([1]);\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                    "a {\
         \n  value: [1];\
         \n  type: string;\
         \n}\n"
                );
            }
        }
        #[test]
        fn comma() {
            let runner = runner().with_cwd("comma");
            assert_eq!(
                runner.ok("$result: inspect((1,));\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                "a {\
         \n  value: (1,);\
         \n  type: string;\
         \n}\n"
            );
        }
        #[test]
        fn slash() {
            let runner = runner().with_cwd("slash");
            assert_eq!(
                runner.ok("$result: inspect(append((), 1, slash));\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                "a {\
         \n  value: (1/);\
         \n  type: string;\
         \n}\n"
            );
        }
        #[test]
        fn space() {
            let runner = runner().with_cwd("space");
            assert_eq!(
                runner.ok("$result: inspect(append((), 1, space));\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                "a {\
         \n  value: 1;\
         \n  type: string;\
         \n}\n"
            );
        }
    }
    #[test]
    fn space() {
        let runner = runner().with_cwd("space");
        assert_eq!(
            runner.ok("$result: inspect(1 2 3);\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
            "a {\
         \n  value: 1 2 3;\
         \n  type: string;\
         \n}\n"
        );
    }
}
mod map {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("map")
    }

    mod list {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("list")
        }

        mod key {
            #[allow(unused)]
            fn runner() -> crate::TestRunner {
                super::runner().with_cwd("key")
            }

            #[test]
            fn comma() {
                let runner = runner().with_cwd("comma");
                assert_eq!(
                    runner.ok("$result: inspect(((1, 2): 3, (4, 5): 6));\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                    "a {\
         \n  value: ((1, 2): 3, (4, 5): 6);\
         \n  type: string;\
         \n}\n"
                );
            }
            #[test]
            fn space() {
                let runner = runner().with_cwd("space");
                assert_eq!(
                    runner.ok("$result: inspect((1 2: 3, 4 5: 6));\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                    "a {\
         \n  value: (1 2: 3, 4 5: 6);\
         \n  type: string;\
         \n}\n"
                );
            }
        }
        mod value {
            #[allow(unused)]
            fn runner() -> crate::TestRunner {
                super::runner().with_cwd("value")
            }

            #[test]
            fn comma() {
                let runner = runner().with_cwd("comma");
                assert_eq!(
                    runner.ok("$result: inspect((1: (2, 3), 4: (5, 6)));\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                    "a {\
         \n  value: (1: (2, 3), 4: (5, 6));\
         \n  type: string;\
         \n}\n"
                );
            }
            #[test]
            fn space() {
                let runner = runner().with_cwd("space");
                assert_eq!(
                    runner.ok("$result: inspect((1: 2 3, 4: 5 6));\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                    "a {\
         \n  value: (1: 2 3, 4: 5 6);\
         \n  type: string;\
         \n}\n"
                );
            }
        }
    }
    #[test]
    fn number() {
        let runner = runner().with_cwd("number");
        assert_eq!(
            runner.ok("$result: inspect((1: 2, 3: 4));\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
            "a {\
         \n  value: (1: 2, 3: 4);\
         \n  type: string;\
         \n}\n"
        );
    }
}
#[test]
fn null() {
    let runner = runner().with_cwd("null");
    assert_eq!(
        runner.ok("$result: inspect(null);\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
        "a {\
         \n  value: null;\
         \n  type: string;\
         \n}\n"
    );
}
mod number {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("number")
    }

    #[test]
    fn unit() {
        let runner = runner().with_cwd("unit");
        assert_eq!(
        runner.ok(
            "// We explicitly don\'t test the inspect format for complex units. Their format\
             \n// isn\'t guaranteed by the spec, since they can\'t be written literally in Sass.\
             \n$result: inspect(50px);\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"
        ),
        "a {\
         \n  value: 50px;\
         \n  type: string;\
         \n}\n"
    );
    }
    #[test]
    fn unitless() {
        let runner = runner().with_cwd("unitless");
        assert_eq!(
            runner.ok("$result: inspect(123.456);\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
            "a {\
         \n  value: 123.456;\
         \n  type: string;\
         \n}\n"
        );
    }
}
mod string {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("string")
    }

    #[test]
    fn quoted() {
        let runner = runner().with_cwd("quoted");
        assert_eq!(
        runner.ok(
            "$result: inspect(\"foo\");\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\n\
             \n  // inspect() should always return an unquoted string, so when it\'s passed a\
             \n  // quoted string its return value should contain quote characters. We check\
             \n  // the length to verify that the quotes are included, since there\'s no\
             \n  // built-in way to check whether a string is quoted.\
             \n  length: str-length($result);\
             \n}\n"
        ),
        "a {\
         \n  value: \"foo\";\
         \n  type: string;\
         \n  length: 5;\
         \n}\n"
    );
    }
    #[test]
    fn unquoted() {
        let runner = runner().with_cwd("unquoted");
        assert_eq!(
            runner.ok("$result: inspect(foo);\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
            "a {\
         \n  value: foo;\
         \n  type: string;\
         \n}\n"
        );
    }
}
