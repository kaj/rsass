macro_rules! test_size {
    ($what:ident, $size:expr) => {
        #[test]
        #[allow(non_snake_case)]
        fn $what() {
            assert_eq!(std::mem::size_of::<$what>(), $size);
        }
    };
}
pub(crate) use test_size;
