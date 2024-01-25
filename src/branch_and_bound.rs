// struct Item {
//     weight: usize,
//     value: usize,
// }
// mod defs;
// use defs::Item;
use super::defs::Item;

// 本來想顺便把解也返回，但是发现这样会导致所有权问题，所以还是算了
// struct Solution {
//     solution: Vec<bool>,
//     max_value: usize,
// }

pub fn knapsack(items: &[Item], capacity: usize) -> usize {
    let mut max_value = 0;
    let mut current_value = 0;
    let mut current_weight = 0;
    let mut current_idx = 0;

    let mut current_solution = vec![false; items.len()];

    branch_and_bound(
        items,
        capacity,
        &mut current_solution,
        &mut current_value,
        &mut current_weight,
        &mut current_idx,
        &mut max_value,
    );

    // return solution and max value
    max_value
}

fn branch_and_bound(
    items: &[Item],
    capacity: usize,
    current_solution: &mut Vec<bool>,
    current_value: &mut usize,
    current_weight: &mut usize,
    current_idx: &mut usize,
    max_value: &mut usize,
) {
    if *current_weight > capacity {
        return;
    }

    if *current_value > *max_value {
        *max_value = *current_value;
    }

    if *current_idx >= items.len() {
        return;
    }

    let item = &items[*current_idx];

    // Include the item
    current_solution[*current_idx] = true;
    *current_value += item.value;
    *current_weight += item.weight;

    branch_and_bound(
        items,
        capacity,
        current_solution,
        current_value,
        current_weight,
        &mut (*current_idx + 1),
        max_value,
    );

    // Exclude the item
    current_solution[*current_idx] = false;
    *current_value -= item.value;
    *current_weight -= item.weight;

    branch_and_bound(
        items,
        capacity,
        current_solution,
        current_value,
        current_weight,
        &mut (*current_idx + 1),
        max_value,
    );
}

fn main() {
    let items = [
        Item { weight: 2, value: 3 },
        Item { weight: 3, value: 4 },
        Item { weight: 4, value: 5 },
        Item { weight: 5, value: 6 },
    ];

    let capacity = 8;
    let max_value = knapsack(&items, capacity);

    println!("Maximum value: {}", max_value);
}
