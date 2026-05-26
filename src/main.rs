mod hackerrank;

use hackerrank::task03::staircase;

fn main() {
    let result = staircase(4);

    for line in result {
        println!("{}", line);
    }
}