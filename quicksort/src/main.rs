use std::mem;

fn quicksort(arr: &mut Vec<i64>) -> &mut Vec<i64> {
    if arr.len() <= 1 {
        return arr;
    }
    let mut p1: usize = 0;
    let mut p2: usize = arr.len() - 1;
    while p1 != p2 {
        if (arr[p1] > arr[p2] && p1 > p2) || (arr[p1] < arr[p2] && p1 < p2) {
            let mut temp = arr.clone();
            mem::swap(&mut arr[p1], &mut temp[p2]);
            mem::swap(&mut arr[p2], &mut temp[p1]);
            mem::swap(&mut p1,&mut p2);

            if p1 > p2 {
                p1 -= 1;
            } else {
                p1 += 1;
            }
        }
    }
    let mut left = quicksort(&mut arr[..p1].to_vec());
    let mut right = quicksort(&mut arr[p1+1..].to_vec());

    left.to
}

fn main() {
    println!("Hello, world!");
}
