//! Tests auto-converted from "sass-spec/spec/non_conformant/sass"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/non_conformant/sass/basic.hrx"

mod comment;

// From "sass-spec/spec/non_conformant/sass/extend.hrx"

// From "sass-spec/spec/non_conformant/sass/functions.hrx"

mod import;

// From "sass-spec/spec/non_conformant/sass/imported.hrx"
#[test]
#[ignore] // wrong result
fn imported() {
    assert_eq!(
        rsass(
            "@import \"imported.sass\";\
            \n"
        )
        .unwrap(),
        "div a {\
        \n  color: red;\
        \n}\
        \ndiv li {\
        \n  color: green;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/sass/indentation.hrx"
mod indentation {
    #[allow(unused)]
    use super::rsass;
    mod different {
        #[allow(unused)]
        use super::rsass;
    }
    mod error {
        #[allow(unused)]
        use super::rsass;
    }
    mod unusual_newlines {
        #[allow(unused)]
        use super::rsass;
    }
}

// From "sass-spec/spec/non_conformant/sass/mixins.hrx"

// From "sass-spec/spec/non_conformant/sass/pseudo.hrx"

// From "sass-spec/spec/non_conformant/sass/selectors.hrx"

mod semicolon;

mod var_args;
