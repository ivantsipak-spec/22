#[allow(dead_code)]
pub fn bon_appetit(bill: Vec<i32>, k: usize, b: i32) -> String {
    let total: i32 = bill.iter().sum();
    let anna_share = (total - bill[k]) / 2;

    if anna_share == b {
        "Bon Appetit".to_string()
    } else {
        (b - anna_share).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bon_appetit_refund() {
        let result = bon_appetit(vec![3, 10, 2, 9], 1, 12);

        assert_eq!(result, "5");
    }

    #[test]
    fn test_bon_appetit_fair() {
        let result = bon_appetit(vec![3, 10, 2, 9], 1, 7);

        assert_eq!(result, "Bon Appetit");
    }
}