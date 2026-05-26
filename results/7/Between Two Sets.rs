#[allow(dead_code)]
pub fn get_total_x(a: Vec<i32>, b: Vec<i32>) -> i32 {
    let mut count = 0;

    for x in 1..=100 {
        let valid_for_a = a.iter().all(|&num| x % num == 0);
        let valid_for_b = b.iter().all(|&num| num % x == 0);

        if valid_for_a && valid_for_b {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_total_x() {
        let result = get_total_x(
            vec![2, 4],
            vec![16, 32, 96],
        );

        assert_eq!(result, 3);
    }
}