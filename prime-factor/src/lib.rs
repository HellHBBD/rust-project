pub fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 {
        return true;
    }

    let max = (n as f64).sqrt() as i32 + 1;
    for i in 2..max {
        if n % i == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod is_prime {
    use super::*;

    #[test]
    fn basic_primes() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
    }

    #[test]
    fn medium_numbers() {
        assert!(is_prime(97));
        assert!(!is_prime(121));
        assert!(!is_prime(169));
        assert!(!is_prime(210));
    }

    #[test]
    fn large_primes() {
        assert!(is_prime(3571));
        assert!(is_prime(7919));
        assert!(!is_prime(825265));
    }

    #[test]
    fn negative_and_invalid() {
        assert!(!is_prime(-2));
        assert!(!is_prime(-7));
        assert!(!is_prime(i32::MIN));
    }

    #[test]
    fn performance_sensitive() {
        assert!(!is_prime(1_000_000));
        assert!(is_prime(1_000_003));
    }
}

pub fn decompose(n: i32) -> Vec<(i32, i32)> {
    if n <= 1 {
        return Vec::new();
    }

    let mut n = n;
    let mut result = Vec::new();
    let mut i = 2;

    while i * i <= n {
        let mut count = 0;
        while n % i == 0 {
            count += 1;
            n /= i;
        }
        if count != 0 {
            result.push((i, count));
        }
        i += 1;
    }

    if n > 1 {
        result.push((n, 1));
    }

    result
}

#[cfg(test)]
mod decompose {
    use super::*;

    #[test]
    fn basic_cases() {
        assert_eq!(decompose(1), vec![]);
        assert_eq!(decompose(2), vec![(2, 1)]);
        assert_eq!(decompose(12), vec![(2, 2), (3, 1)]);
        assert_eq!(decompose(60), vec![(2, 2), (3, 1), (5, 1)]);
    }

    #[test]
    fn prime_numbers() {
        assert_eq!(decompose(7), vec![(7, 1)]);
        assert_eq!(decompose(29), vec![(29, 1)]);
        assert_eq!(decompose(7919), vec![(7919, 1)]);
    }

    #[test]
    fn repeated_factors() {
        assert_eq!(decompose(8), vec![(2, 3)]);
        assert_eq!(decompose(625), vec![(5, 4)]);
        assert_eq!(decompose(1024), vec![(2, 10)]);
    }

    #[test]
    fn invalid_input() {
        assert_eq!(decompose(0), vec![]);
        assert_eq!(decompose(-42), vec![]);
    }
}

pub fn print(list: Vec<(i32, i32)>) {
    for (factor, exponent) in list {
        print!("({factor}^{exponent})");
    }
}
