pub fn fib(n: u32) -> u32 {
    if n > 46 { panic!("Output will be too large for 32-bit integer.") };

    if n == 0 { return 0 };
    if n == 1 { return 1 };

    return fib(n - 1) + fib(n - 2);
}


#[test]
fn test_zero() {
    assert_eq!(0, fib(0));
}

#[test]
fn test_one() {
    assert_eq!(1, fib(1));
}

#[test]
fn test_ten() {
    assert_eq!(55, fib(10));
}

#[test]
fn test_twenty() {
    assert_eq!(6765, fib(20));
}

#[test]
fn test_big() {
    assert_eq!(102334155, fib(40));
}


#[test]
#[should_panic]
fn test_fail() {
    assert_eq!(2971215073, fib(47));
}
