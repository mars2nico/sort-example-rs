/// *practice* basic sorting
///
/// TODO avoid Copy Trait

pub fn insertion_sort<T>(slice: &mut [T])
where
    T: Copy + Ord,
{
    _insertion_sort(slice, 1);
}

fn _insertion_sort<T>(slice: &mut [T], gap: usize)
where
    T: Copy + Ord,
{
    for i in gap..slice.len() {
        let v = slice[i];
        let mut j = i;
        while j >= gap && slice[j - gap] > v {
            slice[j] = slice[j - gap];
            j -= gap;
        }
        slice[j] = v;
    }
}

// TODO: consider another method
pub fn shell_sort<T>(slice: &mut [T])
where
    T: Copy + Ord,
{
    // g[0] = 0, g[N+1] = 3 * g[N] + 1
    #[rustfmt::skip]
    let g: [usize; 41] = [
        0,1,4,13,40,
        121,364,1093,3280,
        9841,29524,88573,265720,
        797161,2391484,7174453,21523360,
        64570081,193710244,581130733,1743392200,
        5230176601,15690529804,47071589413,141214768240,
        423644304721,1270932914164,3812798742493,11438396227480,
        34315188682441,102945566047324,308836698141973,926510094425920,
        2779530283277761,8338590849833284,25015772549499853,75047317648499560,
        225141952945498681,675425858836496044,2026277576509488133,6078832729528464400
    ];
    //let mut i = g.iter().filter(|&h| *h <= slice.len()).count() - 1;
    let mut i = g.iter().position(|&h| h >= slice.len()).unwrap();
    while i > 0 {
        _insertion_sort(slice, g[i]);
        i -= 1;
    }
}

struct HeapSort<'a, T>
where
    T: Copy + Ord,
{
    arr: &'a mut [T],
}

impl<T> HeapSort<'_, T>
where
    T: Copy + Ord,
{
    fn build_heap(&mut self) {
        let mut i = self.arr.len() / 2;
        while {
            i -= 1;
            self.heapify(i, self.arr.len());
            i > 0
        } {}
    }

    fn heapify(&mut self, idx: usize, max: usize) {
        let left = 2 * idx + 1;
        let right = 2 * idx + 2;
        let mut largest;

        if left < max && self.arr[left] > self.arr[idx] {
            largest = left;
        } else {
            largest = idx;
        }

        if right < max && self.arr[right] > self.arr[largest] {
            largest = right;
        }

        if largest != idx {
            self.arr.swap(idx, largest);

            self.heapify(largest, max);
        }
    }

    fn sort(&mut self) {
        self.build_heap();
        let mut i = self.arr.len() - 1;
        while i >= 1 {
            self.arr.swap(0, i);

            self.heapify(0, i);
            i -= 1;
        }
    }
}

pub fn heap_sort<T>(arr: &mut [T])
where
    T: Copy + Ord,
{
    if arr.is_empty() {
        return;
    };

    let mut solver = HeapSort { arr };
    solver.sort();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort() {
        let mut flag = false;
        let mut arr = [6, 2, 4, 5, 3, 1];
        insertion_sort(&mut arr);

        for i in 1..arr.len() {
            if arr[i - 1] > arr[i] {
                flag = true;
            }
        }
        assert!(!flag);
    }

    #[test]
    fn test_heap_sort() {
        let mut flag = false;
        let mut arr = [6, 2, 4, 5, 3, 1];
        heap_sort(&mut arr);

        for i in 1..arr.len() {
            if arr[i - 1] > arr[i] {
                flag = true;
            }
        }
        assert!(!flag);
    }

    #[test]
    fn test_zero() {
        let mut arr: [i32; 0] = [];

        insertion_sort(&mut arr);
        heap_sort(&mut arr);
        shell_sort(&mut arr);
    }
}
