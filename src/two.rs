//! # Even Fibonacci numbers
//! ## Problem 2
//! Each new term in the Fibonacci sequence is generated by adding the previous two terms. By starting with 1 and 2, the first 10 terms will be:
//!
//! 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
//!
//! By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms.
//!

/// # The Golden Ratio
/// ## Notes:
/// See: https://en.wikipedia.org/wiki/Golden_ratio
/// ** this must be stored as a f64
pub const PHI: f64 = 1.618033988749895;

/// Find the fibonacci number for the nth term
/// ## Examples:
/// ```
/// // Get the 26th term in the fibonacci sequence
/// let fib = euler::two::get_fib(10);
/// assert_eq!(34, fib);
/// ```
/// ### Notes:
/// This function uses one of the phi-based approaches: `Phi ^ n / (Phi + 2)`
pub fn get_fib(number: usize) -> usize {
    let fib = (PHI.powi(number as i32) / (PHI + 2f64)).round() as usize;
    fib
}

/// # Solve Problem Two
/// ## Examples:
/// ```
/// let result = answer();
/// assert_eq!(4613732, result);
/// ```
pub fn answer() -> usize {
    (0..)
        .map(|n| get_fib(n))
        .take_while(|&n| n < 4_000_000)
        .filter(|&n| n % 2 == 0)
        .sum()
}
