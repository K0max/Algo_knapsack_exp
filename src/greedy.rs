// #[derive(Clone)]
// struct Item {
//     weight: usize,
//     value: usize,
// }
// mod defs;
// use defs::Item;
use super::defs::Item;

pub fn knapsack(items: &[Item], capacity: usize) -> usize {
    let mut sorted_items: Vec<Item> = items.iter().cloned().collect();
    sorted_items.sort_by(|a, b| {
        let ratio_a = a.value as f64 / a.weight as f64;
        let ratio_b = b.value as f64 / b.weight as f64;
        ratio_b.partial_cmp(&ratio_a).unwrap()
    });

    let mut total_value = 0;
    let mut remaining_capacity = capacity;

    for item in sorted_items {
        if item.weight <= remaining_capacity {
            total_value += item.value;
            remaining_capacity -= item.weight;
        } else {
            let fractional_value = remaining_capacity as f64 / item.weight as f64;
            total_value += (item.value as f64 * fractional_value) as usize;
            break;
        }
    }

    total_value
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