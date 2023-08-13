// *practice* algorithms and data structures

use std::cmp;
use std::hash::Hash;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct MinimumCostSort<'a, T> where
    T: Copy + Hash + Ord {
    pub arr: &'a [T],
}

impl<T> MinimumCostSort<'_, T> where
    T: Copy + Hash + Ord {

    pub fn solve<Eval>(&self, eval: Eval) -> Result<i32> where
        Eval: Fn(&T) -> i32 {
        let mut ans = 0;
        let mut arr = Vec::from_iter(self.arr.iter().enumerate());
        let mut sorted = arr.clone();
        let mut v = vec![false; arr.len()];

        sorted.sort_by(|(_, &x), (_, &y)| x.cmp(&y));
        for i in 0..arr.len() {
            arr[sorted[i].0].0 = i;
        }

        for i in 0..arr.len() {
            if v[i] {
                continue;
            }
            let mut cur = i;
            let mut sum = 0;
            let (_, mut min) = sorted.last().unwrap();
            let mut an = 0;
            loop {
                v[cur] = true;
                an += 1;
                let (val_idx, val) = arr[cur];
                min = cmp::min(min, val);
                sum += eval(val);
                cur = val_idx;
                if v[cur] {
                    break;
                }
            }
            let (_, &global_min) = sorted.first().unwrap();
            ans += (sum + (an - 2) * eval(min)).min(eval(min) + sum + (an + 1) * eval(&global_min));
        }
        Ok(ans)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minumum_cost_sort1() {
        let solver = MinimumCostSort {
            arr: &[1, 5, 3, 4, 2]
        };
        match solver.solve(|&x| x) {
            Ok(result) => assert_eq!(result, 7),
            Err(_) => panic!("fail"),
        }
    }

    #[test]
    fn test_minumum_cost_sort2() {
        let solver = MinimumCostSort {
            arr: &[4, 3, 2, 7, 1, 6, 5]
        };
        match solver.solve(|&x| x) {
            Ok(result) => assert_eq!(result, 24),
            Err(_) => panic!("fail"),
        }
    }

    #[test]
    fn test_minumum_cost_sort3() {
        let solver = MinimumCostSort {
            arr: &[2, 1, 8, 10, 7, 9]
        };
        match solver.solve(|&x| x) {
            Ok(result) => assert_eq!(result, 49),
            Err(_) => panic!("fail"),
        }
    }
}
