use std::time::Instant;
use ::rand::thread_rng;
use ::rand::seq::SliceRandom;
mod sort;

fn main() {
    let mut rng = thread_rng();
    let choices: Vec<_> = (1..11).collect();
    let mut base_arr = Vec::new();
    for _ in 0..10 {
        base_arr.push(*choices.choose(&mut rng).unwrap());
    }
    let mut arr1 = base_arr.clone();
    let mut arr2 = base_arr.clone();
    {
        println!(" arr(before) = {:?}", &arr1[0..10]);

        let start = Instant::now();
        sort::insertion_sort(&mut arr1);

        println!(" arr(after ) = {:?}, elapsed = {:?}", &arr1[0..10], start.elapsed());
    }
    {
        println!(" arr(before) = {:?}", &arr2[0..10]);

        let start = Instant::now();
        sort::shell_sort(&mut arr2);

        println!(" arr(after ) = {:?}, elapsed = {:?}", &arr2[0..10], start.elapsed());
    }
    for i in 0..base_arr.len() {
        if arr1[i] != arr2[i] {
            println!("!ERR at {:?}", i);
        }
    }
}
