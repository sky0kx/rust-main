pub fn is_prime(n: &u32) -> bool {
    if *n < 2 {
        return false;
    }
    for i in 2..=(*n as f64).sqrt() as u32 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        let test_data = [
            (0, false),
            (1, false),
            (2, true),
            (3, true),
            (4, false),
            (5, true),
            (100, false),
            (10007, true),
        ];

        test_data
            .iter()
            .for_each(|(n, prime)| assert_eq!(is_prime(n), *prime));
    }
}
