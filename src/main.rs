mod hackerrank;

use hackerrank::task07::get_total_x;

fn main() {
    let result = get_total_x(
        vec![2, 4],
        vec![16, 32, 96],
    );

    println!("{}", result);
}