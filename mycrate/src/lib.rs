pub mod mymodule {
    pub fn fizzbuzz(n: i32) -> String {
        match (n % 3, n % 5) {
            (0, 0) => "fizzbuzz".to_string(),
            (0, _) => "fizz".to_string(),
            (_, 0) => "buzz".to_string(),
            _ => n.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use mymodule::fizzbuzz;

    #[test]
    fn fizzbuzz_1() {
        assert_eq!("1", fizzbuzz(1));
    }

    #[test]
    fn fizzbuzz_3() {
        assert_eq!("fizz", fizzbuzz(3));
    }

    #[test]
    fn fizzbuzz_9() {
        assert_eq!("fizz", fizzbuzz(9));
    }

    #[test]
    fn fizzbuzz_5() {
        assert_eq!("buzz", fizzbuzz(5));
    }

    #[test]
    fn fizzbuzz_25() {
        assert_eq!("buzz", fizzbuzz(25));
    }

    #[test]
    fn fizzbuzz_15() {
        assert_eq!("fizzbuzz", fizzbuzz(15));
    }

    #[test]
    fn fizzbuzz_75() {
        assert_eq!("fizzbuzz", fizzbuzz(75));
    }
}
