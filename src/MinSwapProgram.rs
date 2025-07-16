// Difficulty - Medium

fn main() { 
    let arr = [2, 4, 5, 1, 3];
    println!("{}", min_swaps(&arr));
} 

fn min_swaps(arr: &[i32]) -> i32 {
    let mut ans: i32 = 0;

    let n: usize = arr.len();

    let mut vis = vec![false; n];
    let mut arrpos = vec![(0,0); n];
    for i in 0..arr.len()
    {
        arrpos[i] = (arr[i], i);
    }

    arrpos.sort_by(|a, b| a.0.cmp(&b.0));

    for i in 0..arr.len()
    {
        let mut cycles = 0;
        let mut j = i;

        if vis[i] || arrpos[i].0 == arrpos[i].1 as i32 {
            continue;
        }

        while !vis[j] {
            vis[j] = true;

            j = arrpos[j].1;
            cycles += 1;
        }

        if cycles > 0 {
            ans += cycles - 1;
        }
    }

    return ans;
}