// Difficulty - Medium

// fn main(){
//     let arr = vec![1, 2, 5, 3, 7, 8, 6, 4]; // 7
//     minimum_bribes(arr);
// }

// fn minimum_bribes(q: Vec<i32>){
//     let mut isChaotic = false;
//     let n = q.len();
//     let qarr = new Array(n).fill(0).map((item, index) => index + 1);
//     let qobj = {};
//     let i = 0;
//     let answer = 0;

//     for i, o in enumerate(q):
//         cur = i

//         if o - cur > 2:
//             print("Too chaotic")
//             return
        
//         for k in q[max(o - 1, 0):i]:
//             if k > o:
//                 bribes += 1

//     print(bribes)
// }















// use std::io;

// // Enter your code here 
// fn main(){
//     let mut tstr = String::new();
//     io::stdin().read_line(&mut tstr).expect("failed to readline");
    
//     let t = tstr.trim().parse::<i32>().unwrap_or(-1);
    
//     for itr in 0..t{
//         let mut nstr = String::new();
//         io::stdin().read_line(&mut nstr).expect("failed to readline");
//         let n = nstr.trim().parse::<i32>().unwrap_or(-1);
        
//         for inr in 0..n{
//             let mut arrstr = String::new();
//             io::stdin().read_line(&mut arrstr).expect("failed to readline");
//             let q: Result<Vec<i32>, _> = line.trim.split(' ').map(str::parse).collect();
            
//             minimum_bribes(&q);
//         }
//     }
// }
