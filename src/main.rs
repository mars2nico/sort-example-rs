use std::time::Instant;
use algorithms_and_data_structures::MinimumCostSort;
use ::rand::thread_rng;
use ::rand::seq::SliceRandom;

mod sort;
mod algorithms_and_data_structures;

fn main() {
    let mut rng = thread_rng();
    let choices: Vec<_> = (1..10001).collect();
    let mut base_arr = Vec::new();
    for _ in 0..10000 {
        base_arr.push(*choices.choose(&mut rng).unwrap());
    }
    {
        let mut arr = base_arr.clone();
        println!(" arr(before) = {:?}", &arr[0..10]);

        let start = Instant::now();
        sort::insertion_sort(&mut arr);

        println!(" arr(after ) = {:?}, elapsed = {:?}", &arr[0..10], start.elapsed());
    }
    {
        let mut arr = base_arr.clone();
        println!(" arr(before) = {:?}", &arr[0..10]);

        let start = Instant::now();
        sort::shell_sort(&mut arr);

        println!(" arr(after ) = {:?}, elapsed = {:?}", &arr[0..10], start.elapsed());
    }
    {
        let mut arr = base_arr.clone();
        println!(" arr(before) = {:?}", &arr[0..10]);

        let start = Instant::now();
        sort::heap_sort(&mut arr);

        println!(" arr(after ) = {:?}, elapsed = {:?}", &arr[0..10], start.elapsed());
    }
    {
        let solver = MinimumCostSort {
            arr: &[
                (2, "scale"),
                (1, "pen"),
                (8, "board"),
                (10, "paper"),
                (7, "tape"),
                (9, "ink"),
            ]
            // (0, 2), (1, 1), (2, 8), (3,10), (4, 7), (5, 9)
            //
            // 0:      1:      2:      3:      4:      5:
            // (1, 1), (0, 2), (4, 7), (2, 8), (5, 9), (3,10)
        };
        match solver.solve(|&x| x.0) {
            Ok(result) => println!("{}", result),
            Err(_) => println!("!err"),
        }
    }
}
