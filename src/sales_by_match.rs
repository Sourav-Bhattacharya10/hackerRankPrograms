// There is a large pile of socks that must be paired by color.
// Given an array of integers representing the color of each sock,
// determine how many pairs of socks with matching colors there are.

// Example:
// n = 7
// ar = [1, 2, 1, 2, 1, 3, 2]

// There is one pair of color and one of color.
// There are three odd socks left, one of each color.
// The number of pairs is 2.

// Function Description:
// Complete the sockMerchant function in the editor below.

// sockMerchant has the following parameter(s):
// int n: the number of socks in the pile
// int ar[n]: the colors of each sock

// Returns:
// int: the number of pairs

// Input Format:
// The first line contains an integer n,
// the number of socks represented in ar.
// The second line contains n space-separated integers,
// ar[i], the colors of the socks in the pile.

// Sample Input:

// STDIN                       Function
// -----                       --------
// 9                           n = 9
// 10 20 20 10 10 30 50 10 20  ar = [10, 20, 20, 10, 10, 30, 50, 10, 20]

// Sample Output:
// 3

// Program
use std::{
    collections::HashMap,
    io::{self, BufRead},
};

/*
 * Complete the 'sockMerchant' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. INTEGER_ARRAY ar
 */

fn sock_merchant(_n: i32, ar: &[i32]) -> i32 {
    let mut color_hash_map: HashMap<i32, i32> = HashMap::new();

    for &val in ar {
        let count = match color_hash_map.get(&val) {
            Some(&c) => c,
            None => 0,
        };
        color_hash_map.insert(val, count + 1);
    }

    let counter = color_hash_map.values().fold(0, |acc, val| {
        let pairs = val / 2;
        if pairs >= 1 {
            return acc + pairs;
        }

        acc
    });

    counter
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let ar: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = sock_merchant(n, &ar);

    println!("{}", result);
}
