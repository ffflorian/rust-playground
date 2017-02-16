extern crate helloworld;

#[test]
fn test_hello() {
    assert_eq!("Hello world!", helloworld::hello());
}
