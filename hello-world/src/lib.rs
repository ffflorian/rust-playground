pub fn hello() -> String {
    return String::from("Hello world!");
}


#[test]
fn test_hello() {
    assert_eq!("Hello world!", hello());
}
