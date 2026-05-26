mod hackerrank;

use hackerrank::task11::diagonal_difference;

fn main() {
    let result = diagonal_difference(vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![9, 8, 9],
    ]);

    println!("{}", result);
}