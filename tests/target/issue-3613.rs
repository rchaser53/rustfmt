struct Test<const T: u8>;

impl<const T: u8> Test<{ T }> {
    fn foo() -> u8 {
        {
            T
        }
    }
}
