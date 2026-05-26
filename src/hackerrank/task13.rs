#[allow(dead_code)]
pub fn divisible_sum_pairs(k: i32, ar: Vec<i32>) -> i32 {
    let mut count = 0;

    for i in 0..ar.len() {
        for j in (i + 1)..ar.len() {
            if (ar[i] + ar[j]) % k == 0 {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divisible_sum_pairs() {
        let result = divisible_sum_pairs(3, vec![1, 3, 2, 6, 1, 2]);

        assert_eq!(result, 5);
    }
}