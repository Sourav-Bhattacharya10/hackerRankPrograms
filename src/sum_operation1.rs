// Starting with a 1-indexed array of zeros and a list of operations,
// for each operation add a value to each array element between two given indices,
// inclusive. Once all operations have been performed, return the maximum value in the array.

// Example:
// n = 10
// queries = [[1, 5, 3], [4, 8, 7], [6, 9, 1]]

// Queries are interpreted as follows:

//     a b k
//     1 5 3
//     4 8 7
//     6 9 1

// Add the values of k
// between the indices a and b inclusive:

// [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
// [3, 3, 3, 3, 3, 0, 0, 0, 0, 0]
// [3, 3, 3, 10, 10, 7, 7, 7, 0, 0]
// [3, 3, 3, 10, 10, 8, 8, 8, 1, 0]

// The largest value is 10 after all operations are performed.

// Function Description:
// Complete the function arrayManipulation with the following parameters:
// int n: the number of elements in the array
// int queries[q][3]: a two dimensional array of queries where each queries[i]
// contains three integers. a, b and k.

// Returns:
// int: the maximum value in the resultant array

// Input Format:
// The first line contains two space-separated integers n
// and q, the size of the array and the number of queries.
// Each of the next q lines contains three space-separated integers a, b and k,
// the left index, right index and number to add.

// Sample Input:
// STDIN       Function
// -----       --------
// 5 3         arr[] size n = 5, queries[] size q = 3
// 1 2 100     queries = [[1, 2, 100], [2, 5, 100], [3, 4, 100]]
// 2 5 100
// 3 4 100

// Sample Output:
// 200

// Explanation:
// After the first update the list is 100 100 0 0 0.
// After the second update list is 100 200 100 100 100.
// After the third update list is 100 200 200 200 100.

// The maximum value is 200.

// Program:
// use std::{
//     // convert::TryInto,
//     io::{self, BufRead},
// };

/*
 * Complete the 'arrayManipulation' function below.
 *
 * The function is expected to return a LONG_INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. 2D_INTEGER_ARRAY queries
 */

// fn array_manipulation(n: i32, queries: &Vec<[u32; 3]>) -> i64 {
//     let mut original_array: Vec<i64> = vec![0; n as usize];

//     for [a, b, k] in queries {
//         original_array = original_array
//             .iter()
//             .enumerate()
//             .map(|(i, &el)| {
//                 if i >= (*a as usize - 1) && i <= (*b as usize - 1) {
//                     return el + *k as i64;
//                 }

//                 el
//             })
//             .collect();
//     }

//     let result: i64 = match original_array.iter().max() {
//         Some(&max) => max,
//         None => 0,
//     };

//     result
// }

// fn main() {
//     let stdin = io::stdin();
//     let mut stdin_iterator = stdin.lock().lines();

//     let first_multiple_input: Vec<i32> = stdin_iterator
//         .next()
//         .unwrap()
//         .unwrap()
//         .split_whitespace()
//         .map(|s| s.parse::<i32>().unwrap())
//         .collect();

//     let n = first_multiple_input[0];

//     let q = first_multiple_input[1];

//     let mut queries: Vec<[u32; 3]> = Vec::with_capacity(q as usize);

//     for _ in 0..q as usize {
//         let query: [u32; 3] = stdin_iterator
//             .next()
//             .unwrap()
//             .unwrap()
//             .trim_end()
//             .split_whitespace()
//             .map(|s| s.to_string().parse::<u32>().unwrap())
//             .collect::<Vec<u32>>()
//             .try_into()
//             .expect("Vector must have exactly 3 elements");

//         queries.push(query);
//     }

//     let result = array_manipulation(n, &queries);

//     println!("{}", result);
// }

use std::io::{self, BufRead};

/*
 * Complete the 'arrayManipulation' function below.
 *
 * The function is expected to return a LONG_INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. 2D_INTEGER_ARRAY queries
 */

// prefix sum or difference array technique
fn arrayManipulation(n: i32, queries: &[Vec<i32>]) -> i64 {
    let mut arr = vec![0i64; (n + 2) as usize];

    for query in queries {
        let a = query[0] as usize;
        let b = query[1] as usize;
        let k = query[2] as i64;

        arr[a] += k;
        arr[b + 1] -= k;
    }

    let mut max_value = 0i64;
    let mut current = 0i64;
    for val in arr {
        current += val;
        max_value = max_value.max(current);
    }

    max_value
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let first_multiple_input: Vec<String> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let n = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let m = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let mut queries: Vec<Vec<i32>> = Vec::with_capacity(m as usize);

    for i in 0..m as usize {
        queries.push(Vec::with_capacity(3_usize));

        queries[i] = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    }

    let result = arrayManipulation(n, &queries);
    println!("{}", result);
}
