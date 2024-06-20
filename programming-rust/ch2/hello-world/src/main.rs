// use the std::env module to get the command-line arguments
use std::env;
// import the FromStr trait from the std::str module, because we need to parse the arguments
// use u64::from_str parse string to u64
use std::str::FromStr;

fn main() {
    let mut numbers = Vec::new();

    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!(
        "The greatest common divisor of the numbers {:?} is {}",
        numbers, d
    );
}

/// Calculate the greatest common divisor of two numbers.
// use /// to document the function
// pass the arguments by value
pub fn gcd(mut n: u64, mut m: u64) -> u64 {
    // use -> to specify the return type
    // assert! is a macro that panics if the condition is false
    assert!(n != 0 && m != 0, "GCD called with n or m equal to 0");
    // debug_assert! is a macro that panics if the condition is false in debug mode
    // and when release mode is enabled, the condition is not checked
    //debug_assert!(n != 0 && m != 0, "GCD called with n or m equal to 0");
    while m != 0 {
        if m < n {
            // swap the values of m and n
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    // return the value of n, if function body ends with an expression, it is returned
    // without a semicolon
    //
    // actually, the return statement is not needed, because the last expression is returned
    //
    n
}

/// Calculate the greatest common divisor of two numbers using recursion.
pub fn gcd_recursion(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd_recursion(b, a % b)
    }
}

// use #[test] to define a test function
// cargo test to run the tests
#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}

#[test]
fn test_regcd() {
    assert_eq!(gcd_recursion(14, 15), 1);
    assert_eq!(
        gcd_recursion(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19),
        3 * 11
    );
}
