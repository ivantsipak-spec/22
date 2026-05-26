#[allow(dead_code)]
pub fn breaking_records(scores: Vec<i32>) -> Vec<i32> {
    let mut highest = scores[0];
    let mut lowest = scores[0];

    let mut max_breaks = 0;
    let mut min_breaks = 0;

    for &score in scores.iter().skip(1) {
        if score > highest {
            highest = score;
            max_breaks += 1;
        }

        if score < lowest {
            lowest = score;
            min_breaks += 1;
        }
    }

    vec![max_breaks, min_breaks]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breaking_records() {
        let result = breaking_records(
            vec![10, 5, 20, 20, 4, 5, 2, 25, 1],
        );

        assert_eq!(result, vec![2, 4]);
    }
}