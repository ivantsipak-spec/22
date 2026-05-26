mod hackerrank;

use hackerrank::task08::breaking_records;

fn main() {
    let result = breaking_records(
        vec![10, 5, 20, 20, 4, 5, 2, 25, 1],
    );

    println!("{} {}", result[0], result[1]);
}