//! Tests auto-converted from "sass-spec/spec/libsass/http_import.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@import \"http://fonts.googleapis.com/css?family=Droid+Sans\";"
        )
        .unwrap(),
        "@import \"http://fonts.googleapis.com/css?family=Droid+Sans\";\
        \n"
    );
}
