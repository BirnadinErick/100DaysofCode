/// gcd(m, n) utilizes Euclid Algorithm for finding GCD
/// * recursive function to eliminate borrow headache
pub fn gcd(m: u64, n: u64) -> u64 {
    if n == 0 {
        return m;
    }

    let r = m % n;
    gcd(n, r)
}

// tests whether given 2 positive integers are coprime
// or not
pub fn are_coprime(x: u64, y: u64) -> bool {
    let g = gcd(x, y);
    return g == 1;
}

// calculates Euler's Totient value for given integer `n`
pub fn totient(n: u64) -> usize {
    (1..=n).filter(|&i| are_coprime(n, i)).count()
}

// tail of a list
pub fn tail<T: Copy>(a: Vec<T>) -> Option<T> {
    match a[..] {
        [.., x] => Some(x),
        [] => None,
    }
}

// last 2 tails of a list
pub fn tail2<T: Copy>(a: Vec<T>) -> Option<(T, T)> {
    match a[..] {
        [.., y, x] => Some((x, y)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tail_test() {
        let r = tail(vec![2003, 5, 19]);
        assert_eq!(Some(19), r);

        let r = tail(vec![2003]);
        assert_eq!(Some(2003), r);

        let r: Option<u64> = tail(Vec::new());
        assert_eq!(None, r);
    }

    #[test]
    fn tail2_test() {
        let r = tail2(vec![2003, 5, 19]);
        assert_eq!(Some((19, 5)), r);

        let r = tail2(vec![2003]);
        assert_eq!(None, r);

        let r: Option<(u64, u64)> = tail2(Vec::new());
        assert_eq!(None, r);
    }

    #[test]
    fn gcd_test() {
        let result = gcd(20536, 7826);
        assert_eq!(result, 2);

        let result = gcd(13, 27);
        assert_eq!(result, 1);
    }

    #[test]
    fn coprime_test() {
        let r = are_coprime(13, 27);
        assert_eq!(r, true);

        let r = are_coprime(20536, 7826);
        assert_ne!(r, true);
    }

    #[test]
    fn totient_test() {
        let rhs = totient(10);
        assert_eq!(4, rhs);
    }
}
