pub fn count_apples_and_oranges(
    s: i32,
    t: i32,
    a: i32,
    b: i32,
    apples: Vec<i32>,
    oranges: Vec<i32>,
) -> (i32, i32) {
    let apples_count = apples
        .iter()
        .filter(|&&x| {
            let position = a + x;
            position >= s && position <= t
        })
        .count() as i32;

    let oranges_count = oranges
        .iter()
        .filter(|&&x| {
            let position = b + x;
            position >= s && position <= t
        })
        .count() as i32;

    (apples_count, oranges_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_apples_and_oranges() {
        let result = count_apples_and_oranges(
            7,
            11,
            5,
            15,
            vec![-2, 2, 1],
            vec![5, -6],
        );

        assert_eq!(result, (1, 1));
    }
}
