// Difficulty - Easy

fn main(){
    let arr: Vec<Vec<i32>> = vec![
        vec![1, 1, 1, 0, 0, 0],
        vec![0, 1, 0, 0, 0, 0],
        vec![1, 1, 1, 0, 0, 0],
        vec![0, 0, 2, 4, 4, 0],
        vec![0, 0, 0, 2, 0, 0],
        vec![0, 0, 1, 2, 4, 0]
    ];

    println!("{}",hour_glass_sum(arr));
}

fn hour_glass_sum(arr: Vec<Vec<i32>>) -> i32 {
    let mut sums: Vec<i32> = Vec::new();

    for i in 0..4 {
        for j in 0..4 {
            let sum = arr[i][j]   + arr[i][j+1]   + arr[i][j+2]
                                  + arr[i+1][j+1]
                    + arr[i+2][j] + arr[i+2][j+1] + arr[i+2][j+2];

            sums.push(sum);
        }
    }

    return *sums.iter().max().unwrap();
}