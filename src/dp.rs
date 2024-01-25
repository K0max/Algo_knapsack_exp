// pub struct Item {
//     pub weight: usize,
//     pub value: usize,
// }
// mod defs;
// use defs::Item;
use super::defs::Item;

pub fn knapsack(items: &[Item], capacity: usize) -> usize {
    let n = items.len();
    let mut dp = vec![vec![0; capacity + 1]; n + 1];

    for i in 1..=n {
        let item = &items[i - 1];
        for j in 0..=capacity {
            if item.weight > j {
                dp[i][j] = dp[i - 1][j];
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i - 1][j - item.weight] + item.value);
            }
        }
    }

    dp[n][capacity]
}

fn main() {
    // 示例输入
    let items = [
        Item { weight: 2, value: 3 },
        Item { weight: 3, value: 4 },
        Item { weight: 4, value: 5 },
        Item { weight: 5, value: 6 },
    ];
    let capacity = 8;

    let max_value = knapsack(&items, capacity);
    println!("最大价值: {}", max_value);
}