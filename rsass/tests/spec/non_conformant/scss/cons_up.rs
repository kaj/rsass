//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/cons-up.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("cons-up")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "@use \"sass:string\";\
             \n@use \"sass:list\";\n\
             \n$inputs-list: \'input[type=\"email\"]\',\
             \n              \'input[type=\"number\"]\',\
             \n              \'input[type=\"password\"]\',\
             \n              \'input[type=\"search\"]\',\
             \n              \'input[type=\"tel\"]\',\
             \n              \'input[type=\"text\"]\',\
             \n              \'input[type=\"url\"]\',\n\
             \n              // Webkit & Gecko may change the display of these in the future\
             \n              \'input[type=\"color\"]\',\
             \n              \'input[type=\"date\"]\',\
             \n              \'input[type=\"datetime\"]\',\
             \n              \'input[type=\"datetime-local\"]\',\
             \n              \'input[type=\"month\"]\',\
             \n              \'input[type=\"time\"]\',\
             \n              \'input[type=\"week\"]\';\n\
             \n$unquoted-inputs-list: ();\n\
             \n@each $input-type in $inputs-list {\
             \n  $unquoted-inputs-list: list.append($unquoted-inputs-list, string.unquote($input-type), comma);\
             \n}\n\
             \ndiv {\
             \n  content: $unquoted-inputs-list;\
             \n  content: list.append((), hello);\
             \n  content: list.length(());\
             \n}"
        ),
        "div {\
         \n  content: input[type=\"email\"], input[type=\"number\"], input[type=\"password\"], input[type=\"search\"], input[type=\"tel\"], input[type=\"text\"], input[type=\"url\"], input[type=\"color\"], input[type=\"date\"], input[type=\"datetime\"], input[type=\"datetime-local\"], input[type=\"month\"], input[type=\"time\"], input[type=\"week\"];\
         \n  content: hello;\
         \n  content: 0;\
         \n}\n"
    );
}
