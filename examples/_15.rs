use cached::*;
use log::*;
use num_bigint::BigUint;
use num_traits::One;
use stopwatch::Stopwatch;

extern crate num_bigint;
extern crate num_traits;
cached! {
    PATH;
    fn compute_paths(x: u32, y: u32) -> BigUint = {
        if x == 0 {
            debug!("X is 0, returning 1");
            return One::one();
        };
        if y == 0 {
            debug!("Y is 0, returning 1)");
            return One::one();
        };
        debug!("x: {:?}   y: {:?}", &x, &y);
        return compute_paths(x - 1, y) + compute_paths(x, y - 1);
    }
}

fn main() {
    for i in 0..=100000000 {
        let sw = Stopwatch::start_new();
        let result = compute_paths(i, i);
        println!(
            "Answer(?) for {:?}x{:?}: {} -- elasped: {:?}",
            &i,
            &i,
            result,
            sw.elapsed_ms()
        );
    }
}
