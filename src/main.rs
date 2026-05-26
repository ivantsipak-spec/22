mod hackerrank;

use hackerrank::task05::count_apples_and_oranges;

fn main() {
    let result = count_apples_and_oranges(
        7,
        11,
        5,
        15,
        vec![-2, 2, 1],
        vec![5, -6],
    );

    println!("{}", result.0);
    println!("{}", result.1);
}