//! Tests auto-converted from "sass-spec/spec/directives/forward/comment.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("comment")
        .mock_file("after_close_paren/loud/other.scss", "$a: 1 !default\n")
        .mock_file("after_close_paren/silent/other.scss", "$a: 1 !default\n")
        .mock_file("after_colon/loud/other.scss", "$a: 1 !default\n")
        .mock_file("after_colon/silent/other.scss", "$a: 1 !default\n")
        .mock_file("after_keyword/loud/other.scss", "")
        .mock_file("after_keyword/silent/other.scss", "")
        .mock_file("after_modifier/loud/other.scss", "")
        .mock_file("after_modifier/silent/other.scss", "")
        .mock_file("after_open_paren/loud/other.scss", "$a: 1 !default\n")
        .mock_file("after_paren/silent/other.scss", "$a: 1 !default\n")
        .mock_file("after_url/loud/other.scss", "")
        .mock_file("after_url/silent/other.scss", "")
        .mock_file("before_close_paren/loud/other.scss", "$a: 1 !default\n")
        .mock_file("before_close_paren/silent/other.scss", "$a: 1 !default\n")
        .mock_file("before_colon/loud/other.scss", "$a: 1 !default\n")
        .mock_file("before_colon/silent/other.scss", "$a: 1 !default\n")
        .mock_file("before_keyword/loud/other.scss", "")
        .mock_file("before_keyword/silent/other.scss", "")
        .mock_file("before_url/loud/other.scss", "")
        .mock_file("before_url/silent/other.scss", "")
}

mod after_close_paren {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("after_close_paren")
    }

    #[test]
    fn loud() {
        let runner = runner().with_cwd("loud");
        assert_eq!(runner.ok("@forward \"other\" with ($a: b) /**/\n"), "");
    }
    #[test]
    fn silent() {
        let runner = runner().with_cwd("silent");
        assert_eq!(runner.ok("@forward \"other\" with ($a: b) //\n"), "");
    }
}
mod after_colon {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("after_colon")
    }

    #[test]
    fn loud() {
        let runner = runner().with_cwd("loud");
        assert_eq!(runner.ok("@forward \"other\" with ($a: /**/ b)\n"), "");
    }
    #[test]
    fn silent() {
        let runner = runner().with_cwd("silent");
        assert_eq!(
            runner.ok("@forward \"other\" with ($a: //\
             \n  b)\n"),
            ""
        );
    }
}
mod after_keyword {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("after_keyword")
    }

    #[test]
    fn loud() {
        let runner = runner().with_cwd("loud");
        assert_eq!(runner.ok("@forward \"other\" as /**/ a-*\n"), "");
    }
    #[test]
    fn silent() {
        let runner = runner().with_cwd("silent");
        assert_eq!(
            runner.ok("@forward \"other\" as //\
             \n  a-*\n"),
            ""
        );
    }
}
mod after_modifier {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("after_modifier")
    }

    #[test]
    fn loud() {
        let runner = runner().with_cwd("loud");
        assert_eq!(runner.ok("@forward \"other\" as a-* /**/\n"), "");
    }
    #[test]
    fn silent() {
        let runner = runner().with_cwd("silent");
        assert_eq!(runner.ok("@forward \"other\" as a-* //\n"), "");
    }
}
mod after_open_paren {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("after_open_paren")
    }

    #[test]
    fn loud() {
        let runner = runner().with_cwd("loud");
        assert_eq!(runner.ok("@forward \"other\" with (/**/ $a: b)\n"), "");
    }
}
mod after_paren {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("after_paren")
    }

    #[test]
    fn silent() {
        let runner = runner().with_cwd("silent");
        assert_eq!(
            runner.ok("@forward \"other\" with (//\
             \n  $a: b)\n"),
            ""
        );
    }
}
mod after_url {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("after_url")
    }

    #[test]
    fn loud() {
        let runner = runner().with_cwd("loud");
        assert_eq!(runner.ok("@forward \"other\" /**/\n"), "");
    }
    #[test]
    fn silent() {
        let runner = runner().with_cwd("silent");
        assert_eq!(runner.ok("@forward \"other\" //\n"), "");
    }
}
mod before_close_paren {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("before_close_paren")
    }

    #[test]
    fn loud() {
        let runner = runner().with_cwd("loud");
        assert_eq!(runner.ok("@forward \"other\" with ($a: b /**/)\n"), "");
    }
    #[test]
    fn silent() {
        let runner = runner().with_cwd("silent");
        assert_eq!(
            runner.ok("@forward \"other\" with ($a: b //\
             \n  )\n"),
            ""
        );
    }
}
mod before_colon {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("before_colon")
    }

    #[test]
    fn loud() {
        let runner = runner().with_cwd("loud");
        assert_eq!(runner.ok("@forward \"other\" with ($a /**/ : b)\n"), "");
    }
    #[test]
    fn silent() {
        let runner = runner().with_cwd("silent");
        assert_eq!(
            runner.ok("@forward \"other\" with ($a //\
             \n  : b)\n"),
            ""
        );
    }
}
mod before_keyword {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("before_keyword")
    }

    #[test]
    fn loud() {
        let runner = runner().with_cwd("loud");
        assert_eq!(runner.ok("@forward \"other\" /**/ as a-*\n"), "");
    }
    #[test]
    fn silent() {
        let runner = runner().with_cwd("silent");
        assert_eq!(
            runner.ok("@forward \"other\" //\
             \n  as a-*\n"),
            ""
        );
    }
}
mod before_url {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("before_url")
    }

    #[test]
    fn loud() {
        let runner = runner().with_cwd("loud");
        assert_eq!(runner.ok("@forward /**/ \"other\"\n"), "");
    }
    #[test]
    fn silent() {
        let runner = runner().with_cwd("silent");
        assert_eq!(
            runner.ok("@forward //\
             \n  \"other\"\n"),
            ""
        );
    }
}
