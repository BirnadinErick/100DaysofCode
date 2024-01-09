use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    numbers: Vec<u64>,
}

fn main() {
    let args = Args::parse();

    let gcd = match args.numbers.into_iter().reduce(gcd) {
        Some(x) => x,
        None => {
            eprintln!("Couldn't determine the GCD!");
            std::process::exit(1);
        }
    };

    println!("{}", gcd);
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
