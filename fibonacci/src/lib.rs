pub fn fib_recursive(n: u32) -> u32 {
    match n {
        0...1 => return n,
        2...46 => return fib_recursive(n - 1) + fib_recursive(n - 2),
        _ => panic!("Result will be too large for 32-bit integer."),
    }
}

pub fn fib_iterative(n: u32) -> u32 {
    if n > 46 { panic!("Result will be too large for 32-bit integer.") };

    if n == 0 { return 0 };
    if n == 1 { return 1 };

    let mut i = 0;
    let mut sum = 0;
    let mut last = 0;
    let mut curr = 1;
    while i < n - 1 {
        sum = last + curr;
        last = curr;
        curr = sum;
        i += 1;
    }

    return sum;
}


#[test]
fn test_zero() {
    assert_eq!(0, fib_recursive(0));
    assert_eq!(0, fib_iterative(0));
}

#[test]
fn test_one() {
    assert_eq!(1, fib_recursive(1));
    assert_eq!(1, fib_iterative(1));
}

#[test]
fn test_ten() {
    assert_eq!(55, fib_recursive(10));
    assert_eq!(55, fib_iterative(10));
}

#[test]
fn test_twenty() {
    assert_eq!(6765, fib_recursive(20));
    assert_eq!(6765, fib_iterative(20));
}

#[test]
fn test_big() {
    assert_eq!(102334155, fib_recursive(40));
    assert_eq!(102334155, fib_iterative(40));
}


#[test]
#[should_panic]
fn test_fail() {
    assert_eq!(2971215073, fib_recursive(47));
    assert_eq!(2971215073, fib_iterative(47));
}
