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
