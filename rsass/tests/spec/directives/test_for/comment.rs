//! Tests auto-converted from "sass-spec/spec/directives/for/comment.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("comment")
}

mod after_from {
    #[allow(unused)]
    use super::runner;

    mod loud {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn scss() {
            assert_eq!(
                runner().ok("@for $i from /**/ 1 through 10 {}\n"),
                ""
            );
        }
    }
    mod silent {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn scss() {
            assert_eq!(
                runner().ok("@for $i from //\
             \n  1 through 10 {}\n"),
                ""
            );
        }
    }
}
mod after_through {
    #[allow(unused)]
    use super::runner;

    mod loud {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn scss() {
            assert_eq!(
                runner().ok("@for $i from 1 through /**/ 10 {}\n"),
                ""
            );
        }
    }
    mod silent {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn scss() {
            assert_eq!(
                runner().ok("@for $i from 1 through //\
             \n  10 {}\n"),
                ""
            );
        }
    }
}
mod before_block {
    #[allow(unused)]
    use super::runner;

    mod loud {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn scss() {
            assert_eq!(
                runner().ok("@for $i from 1 through 10 /**/ {}\n"),
                ""
            );
        }
    }
    mod silent {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn scss() {
            assert_eq!(
                runner().ok("@for $i from 1 through 10 //\
             \n  {}\n"),
                ""
            );
        }
    }
}
mod before_from {
    #[allow(unused)]
    use super::runner;

    mod loud {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn scss() {
            assert_eq!(
                runner().ok("@for $i /**/ from 1 through 10 {}\n"),
                ""
            );
        }
    }
    mod silent {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn scss() {
            assert_eq!(
                runner().ok("@for $i //\
             \n  from 1 through 10 {}\n"),
                ""
            );
        }
    }
}
mod before_through {
    #[allow(unused)]
    use super::runner;

    mod loud {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn scss() {
            assert_eq!(
                runner().ok("@for $i from 1 /**/ through 10 {}\n"),
                ""
            );
        }
    }
    mod silent {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn scss() {
            assert_eq!(
                runner().ok("@for $i from 1 //\
             \n  through 10 {}\n"),
                ""
            );
        }
    }
}
mod before_var {
    #[allow(unused)]
    use super::runner;

    mod loud {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn scss() {
            assert_eq!(
                runner().ok("@for /**/ $i from 1 through 10 {}\n"),
                ""
            );
        }
    }
    mod silent {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn scss() {
            assert_eq!(
                runner().ok("@for //\
             \n  $i from 1 through 10 {}\n"),
                ""
            );
        }
    }
}
mod error {
    #[allow(unused)]
    use super::runner;

    mod after_from {
        #[allow(unused)]
        use super::runner;

        mod silent {
            #[allow(unused)]
            use super::runner;
        }
    }
    mod after_through {
        #[allow(unused)]
        use super::runner;

        mod silent {
            #[allow(unused)]
            use super::runner;
        }
    }
    mod before_from {
        #[allow(unused)]
        use super::runner;

        mod silent {
            #[allow(unused)]
            use super::runner;
        }
    }
    mod before_through {
        #[allow(unused)]
        use super::runner;

        mod silent {
            #[allow(unused)]
            use super::runner;
        }
    }
    mod before_var {
        #[allow(unused)]
        use super::runner;

        mod silent {
            #[allow(unused)]
            use super::runner;
        }
    }
}
