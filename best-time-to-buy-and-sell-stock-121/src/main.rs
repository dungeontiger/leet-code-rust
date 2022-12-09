fn main() {
}

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max_profit = 0;
    for i in 0..prices.len() {
        for j in i..prices.len() {
            let profit = prices[j] - prices[i];
            if profit > max_profit {
                max_profit = profit;
            }
        }
    }
    max_profit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(5, max_profit([7,1,5,3,6,4].to_vec()));
    }

    #[test]
    fn example2() {
        assert_eq!(0, max_profit([7,6,4,3,1].to_vec()));
    }
}