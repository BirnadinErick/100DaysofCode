use std::str::FromStr;

fn main() {
    // get the user defined n
    let n = std::env::args().skip(1).next().expect("Usage: nprimes n");
    let n = i64::from_str(&n).expect("Invalid N. Should be positive integer");

    if n < 1 {
        panic!("Invalid n. Should be positive integer");
    }

    let primes = sieve_eros(
        n.try_into()
            .expect("n should be able to cast into unsigned 64bit integer"),
    );

    println!("{:?}", primes);
}

fn sieve_eros(n: u64) -> Vec<u64> {
    let ns: Vec<u64> = (1..=n).collect();

    ns
}
