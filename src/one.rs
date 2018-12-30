/// # Multiples of 3 and 5
/// ## Problem 1
/// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
///
/// Find the sum of all the multiples of 3 or 5 below 1000.
fn one() -> u32 {
    (0..)
        .take_while(|&n| n < 1000)
        .filter(|&n| n % 3 == 0 || n % 5 == 0)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(233168, one());
    }
}
