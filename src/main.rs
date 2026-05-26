mod hackerrank;

use hackerrank::task04::grading_students;

fn main() {
    let grades = vec![73, 67, 38, 33];

    let result = grading_students(grades);

    for grade in result {
        println!("{}", grade);
    }
}