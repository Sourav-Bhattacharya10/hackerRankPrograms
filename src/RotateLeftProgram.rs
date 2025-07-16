// Difficulty - Easy

fn main(){
    let arr = [1, 2, 3, 4, 5];
    println!("{:?}", rot_left(&arr, 2));
}

fn rot_left(arr: &[i32], d: usize) -> Vec<i32> {
    let left = &arr[d..arr.len()];
    let right = &arr[..d];

    return [left, right].concat();
}