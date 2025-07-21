// There is a new mobile game that starts with consecutively numbered clouds.
// Some of the clouds are thunderheads and others are cumulus.
// The player can jump on any cumulus cloud having a number that is
// equal to the number of the current cloud plus 1 or 2.
// The player must avoid the thunderheads. Determine the minimum number of jumps
// it will take to jump from the starting postion to the last cloud.
// It is always possible to win the game.
// For each game, you will get an array of clouds numbered 0 if they are safe
// or 1 if they must be avoided.

// Example:
// c = [0, 1, 0, 0, 0, 1, 0]

// 0, 1, 0, 0, 0, 1, 0
// 0, 1, 2, 3, 4, 5, 6
// 0---->2->3->4---->6 = 4 steps
// 0---->2---->4---->6 = 3 steps (minimum)

// 0, 0, 1, 0, 0, 1, 0
// 0, 1, 2, 3, 4, 5, 6
// 0->1---->3->4---->6 = 4 steps

// Index the array from 0...6. The number on each cloud is its index in the list
// so the player must avoid the clouds at indices 1 and 5.
// They could follow these two paths: 0 -> 2 -> 4 -> 6 or 0 -> 2 -> 3 -> 4 -> 6.
// The first path takes 3 jumps while the second takes 4. Return 3.

// Function Description:
// Complete the jumpingOnClouds function in the editor below.

// jumpingOnClouds has the following parameter(s):
//     int c[n]: an array of binary integers

// Returns:
//     int: the minimum number of jumps required

// Input Format:
// The first line contains an integer n,
// the total number of clouds.
// The second line contains n space-separated binary integers describing clouds
// c[i] where 0 <= i <= n.

// Output Format:
// Print the minimum number of jumps needed to win the game.

// Sample Input 0:
// 7
// 0 0 1 0 0 1 0

// Sample Output 0:
// 4

// Explanation 0:
// The player must avoid c[2] and c[5].
// The game can be won with a minimum of 4 jumps.

// Sample Input 1:
// 6
// 0 0 0 0 1 0

// Sample Output 1:
// 3

// Explanation 1:
// The only thundercloud to avoid is c[4].
// The game can be won in 3 jumps

// Program
use std::io::{self, BufRead};

/*
 * Complete the 'jumpingOnClouds' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY c as parameter.
 */

fn jumping_on_clouds(c: &[i32]) -> i32 {
    let mut jumps = 0;
    let mut i: usize = 0;

    let mut step: usize;
    while i < c.len() - 1 {
        if (i + 1 < c.len() && c[i + 1] == 0) && (i + 2 < c.len() && c[i + 2] == 0) {
            step = 2;
        } else if (i + 1 < c.len() && c[i + 1] == 0) && (i + 2 < c.len() && c[i + 2] == 1) {
            step = 1;
        } else {
            step = 2;
        }

        jumps += 1;

        i += step;
    }

    jumps
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

    let c: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = jumping_on_clouds(&c);

    println!("{}", result);
}
