mod hackerrank;

use hackerrank::task13::divisible_sum_pairs;

fn main() {
    let result = divisible_sum_pairs(3, vec![1, 3, 2, 6, 1, 2]);

    println!("{}", result);
}