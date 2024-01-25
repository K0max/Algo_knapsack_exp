// struct Item {
//     weight: usize,
//     value: usize,
// }
// mod defs;
// use defs::Item;
use super::defs::Item;

pub fn knapsack(items: &[Item], capacity: usize) -> usize {
    // 递归终止条件：物品列表为空或背包容量为0
    if items.is_empty() || capacity == 0 {
        return 0;
    }

    let last_item = items.last().unwrap();
    let remaining_items = &items[..items.len() - 1];

    // 如果最后一个物品的重量大于背包容量，则不考虑该物品
    if last_item.weight > capacity {
        return knapsack(remaining_items, capacity);
    }

    // 在两种情况下选择最大值：
    // 1. 不包含最后一个物品的最大价值
    // 2. 包含最后一个物品的最大价值
    let value_without_last = knapsack(remaining_items, capacity);
    let value_with_last = last_item.value + knapsack(remaining_items, capacity - last_item.weight);

    value_without_last.max(value_with_last)
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