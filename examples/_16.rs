//! # Power digit sum
//! ## Problem 16
//! 2^15 = 32678 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26
//!
//! What is the sum of the digits of the number 2^1000?
//!

/// # Solve Problem Sixteen
/// ## Answer:
/// ```
/// use euler::_16;
/// let result = main();
/// ```

use num_traits::pow::Pow;


pub fn get_string_sum(number: &num_bigint::BigUint) -> num_bigint::BigUint {
    number.to_string().chars().fold(num_bigint::BigUint::from(0u16) , |x,y|x + num_bigint::BigUint::from(y.to_digit(10).unwrap()))
}

fn main() -> Result<(), std::io::Error> {
    let sample: num_bigint::BigUint = num_bigint::BigUint::from(32768u16).clone();
    let sample_sum = get_string_sum(&sample);

    let big_one: num_bigint::BigUint = num_bigint::BigUint::from(2u16).clone();
    let big_pow = big_one.pow(1000u16);
    println!("Big pow: {}", &big_pow);
    let big_sum = get_string_sum(&big_pow);

    println!("{} : {:?}", &sample, sample_sum.to_string());
    println!("{} : {:?}", &big_one, big_sum.to_string());
    Ok(())
}