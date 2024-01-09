use std::env;
use std::str::FromStr;

fn main() {
    // parse the commandline arguments
    let numbers: Vec<u64> = env::args()
        .skip(1)
        .map(|a| u64::from_str(&a).expect("parsing failed"))
        .collect();

    // wrong usage, instruct the user
    if numbers.is_empty() {
        eprintln!("Usage: rust-101 arg0 arg1 arg2...");
        std::process::exit(1);
    }

    // find GCD of given commandline arguments in order given
    let d = numbers.into_iter().reduce(gcd).unwrap_or(0);

    println!("{}", d);
}

/// gcd(m, n) utilizes Euclid Algorithm for finding GCD
/// * recursive function to eliminate borrow headache
fn gcd(m: u64, n: u64) -> u64 {
    if n == 0 {
        return m;
    }

    let r = m % n;
    gcd(n, r)
}

#[test]
fn test_gcd() {
    // few general tests to assert the core functionality
    assert_eq!(gcd(15, 14), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}
