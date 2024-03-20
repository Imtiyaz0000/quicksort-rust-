use std::mem::{self, swap};

fn quicksort(arr: &mut Vec<i64>) -> &mut Vec<i64> {
    if arr.len() <= 1 {
        return arr;
    }
    let mut p1: usize = 0;
    let p2: usize = arr.len() - 1;
    while p1 != p2 {
        if (arr[p1] > arr[p2] && p1 > p2) || (arr[p1] < arr[p2] && p1 < p2) {
            let mut temp = arr.clone();
            swap(&mut arr[p1], &mut temp[p2]);
            swap(&mut arr[p2], &mut temp[p1]);

            if p1 > p2 {
                p1 -= 1;
            } else {
                p1 += 1;
            }
        }
    }
    arr
}

fn main() {
    println!("Hello, world!");
}
