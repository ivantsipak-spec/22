mod hackerrank;

use hackerrank::task12::birthday_cake_candles;

fn main() {
    let result = birthday_cake_candles(vec![3, 2, 1, 3]);

    println!("{}", result);
}