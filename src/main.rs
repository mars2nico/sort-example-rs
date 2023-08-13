use std::time::Instant;
use ::rand::thread_rng;
use ::rand::seq::SliceRandom;
mod sort;

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
}
