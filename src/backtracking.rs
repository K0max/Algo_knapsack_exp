// struct Item {
//     weight: usize,
//     value: usize,
// }
// mod defs;
// use defs::Item;
use super::defs::Item;

pub fn knapsack(items: &[Item], capacity: usize) -> usize {
    let mut max_value = 0;
    let mut current_value = 0;
    let mut current_weight = 0;
    let mut selected = vec![false; items.len()];

    backtrack(items, capacity, &mut selected, &mut current_value, &mut current_weight, &mut max_value, 0);

    max_value
}

fn backtrack(items: &[Item], capacity: usize, selected: &mut Vec<bool>, current_value: &mut usize, current_weight: &mut usize, max_value: &mut usize, index: usize) {
    if index == items.len() {
        if *current_weight <= capacity && *current_value > *max_value {
            *max_value = *current_value;
        }
        return;
    }

    if *current_weight > capacity {
        return;
    }

    // 不选择当前物品
    backtrack(items, capacity, selected, current_value, current_weight, max_value, index + 1);

    // 选择当前物品
    selected[index] = true;
    *current_value += items[index].value;
    *current_weight += items[index].weight;

    backtrack(items, capacity, selected, current_value, current_weight, max_value, index + 1);

    // 回溯
    selected[index] = false;
    *current_value -= items[index].value;
    *current_weight -= items[index].weight;
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