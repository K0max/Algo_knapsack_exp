mod defs;
use defs::Item;
mod dp;
mod greedy;
mod divide_and_conquer;
mod backtracking;
mod branch_and_bound;
use std::time::Instant;
mod monte_carlo;
use rand::Rng;
use rand::thread_rng;
// rs 的多文件怎么这么诡异, tnnd

fn main() {
    // let items = [
    //     Item { weight: 2, value: 3 },
    //     Item { weight: 3, value: 4 },
    //     Item { weight: 4, value: 5 },
    //     Item { weight: 5, value: 6 },
    // ];
    // let capacity = 8;
    // create random items

    let all_start = Instant::now();
    for _ in 0..10 {
        let n_size = 20;
        let mut items = Vec::new();
        let mut rng = thread_rng();
        for _ in 0..n_size {
            let weight = rng.gen_range(1..100);
            let value = rng.gen_range(1..100);
            items.push(Item { weight, value });
        }
        // let capacity = rng.gen_range(1..n_size * 2);
        // let capacity = 0.5 * sum of weight
        let mut capacity = 0;
        for item in &items {
            capacity += item.weight;
        }
        capacity /= 2;
        // let num_samples = 10000;
        // let dp_start = Instant::now();
        let dp_res = dp::knapsack(&items, capacity);
        // println!("dp time: {:?}", dp_start.elapsed());
    }
    println!("10 times loop time: {:?}", all_start.elapsed());

    // let greedy_start = Instant::now();
    // let greedy_res = greedy::knapsack(&items, capacity);
    // println!("greedy time: {:?}", greedy_start.elapsed());

    // let monte_carlo_start = Instant::now();
    // let monte_carlo_res = monte_carlo::knapsack(&items, capacity, num_samples);
    // println!("monte_carlo time: {:?}", monte_carlo_start.elapsed());

    // if running too long, stop and continue to next algorithm
    // use a moniter thread to kill it
    // define the thread
    // let moniter_thread = std::thread::spawn(move || {
    //     std::thread::sleep(std::time::Duration::from_secs(10));
    //     std::process::exit(0);
    // });
    
    // let divide_and_conquer_start = Instant::now();
    // let divide_and_conquer_res = divide_and_conquer::knapsack(&items, capacity);
    // println!("divide_and_conquer time: {:?}", divide_and_conquer_start.elapsed());

    // let backtracking_start = Instant::now();
    // let backtracking_res = backtracking::knapsack(&items, capacity);
    // println!("backtracking time: {:?}", backtracking_start.elapsed());
    
    // let branch_and_bound_start = Instant::now();
    // let branch_and_bound_res = branch_and_bound::knapsack(&items, capacity);
    // println!("branch_and_bound time: {:?}", branch_and_bound_start.elapsed());
    // let monte_carlo_res = main::knapsack(&items, capacity);

    // println!("dp_res: {}", dp_res);
    // println!("greedy_res: {}", greedy_res);
    // println!("divide_and_conquer_res: {}", divide_and_conquer_res);
    // println!("backtracking_res: {}", backtracking_res);
    // println!("branch_and_bound_res: {}", branch_and_bound_res);
    // println!("monte_carlo_res: {}", monte_carlo_res);
}
