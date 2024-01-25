use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use std::time::Instant;
use super::defs::Item;

// struct Item {
//     weight: usize,
//     value: usize,
// }

pub fn knapsack(items: &[Item], capacity: usize, num_samples: usize) -> usize {
    let mut max_value = 0;

    for _ in 0..num_samples {
        let mut rng = thread_rng();
        let mut current_weight = 0;
        let mut current_value = 0;

        // 随机选择物品放入背包
        let mut shuffled_items: Vec<&Item> = items.iter().collect();
        shuffled_items.shuffle(&mut rng);

        for item in shuffled_items {
            if current_weight + item.weight <= capacity {
                current_weight += item.weight;
                current_value += item.value;
            }
        }

        // 更新最大价值
        if current_value > max_value {
            max_value = current_value;
        }
    }

    max_value
}

fn main() {
    // let items = [
    //     Item { weight: 2, value: 3 },
    //     Item { weight: 3, value: 4 },
    //     Item { weight: 4, value: 5 },
    //     Item { weight: 5, value: 6 },
    // ];
    // let capacity = 8;
    let n_size = 10000;
    let mut items = Vec::new();
    let mut rng = thread_rng();
    for _ in 0..n_size {
        let weight = rng.gen_range(1..50);
        let value = rng.gen_range(1..50);
        items.push(Item { weight, value });
    }
    // let capacity = rng.gen_range(1..n_size * 2);
    // let capacity = 0.5 * sum of weight
    let mut capacity = 0;
    for item in &items {
        capacity += item.weight;
    }
    capacity /= 4;
    let num_samples = 10000;

    let time_start = std::time::Instant::now();
    let max_value = knapsack(&items, capacity, num_samples);
    println!("time: {:?}", time_start.elapsed());
    println!("最大价值: {}", max_value);
}