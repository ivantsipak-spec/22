pub fn grading_students(grades: Vec<i32>) -> Vec<i32> {
    grades
        .iter()
        .map(|&grade| {
            if grade >= 38 {
                let next_multiple = ((grade / 5) + 1) * 5;

                if next_multiple - grade < 3 {
                    return next_multiple;
                }
            }

            grade
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grading_students() {
        let grades = vec![73, 67, 38, 33];

        assert_eq!(
            grading_students(grades),
            vec![75, 67, 40, 33]
        );
    }
}