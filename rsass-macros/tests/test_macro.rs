#[test]
fn test_macro() {
    const CSS: &'static str = rsass_macros::scss!(
        r#"
        .class {
            background: red;
            &:hover {
                background: blue;
            }
        }
    "#
    );
    assert_eq!(".class{background:red}.class:hover{background:blue}\n", CSS)
}
