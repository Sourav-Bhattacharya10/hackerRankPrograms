// use std::io;
// use std::env;
use std::convert::TryInto;

// static MAX_HEALTH: i32 = 100;
// static GAME_NAME: &str = "Monster Attack";
// const MYPI: f32 = 3.14;

// struct Player {
//   fname: &'static str, // nickname
//   health: i32,
//   level: u8
// }

// #[derive(Debug)]
// enum Compass {
//   North, South, East, West
// }

// type static_string = &'static str;

// #[derive(Debug)]
// enum PlanetaryMonster {
//   VenusMonster(static_string, i32),
//   MarsMonster(static_string, i32)
// }

fn main() {
    //   print!("The Game you are playing is called {}.", GAME_NAME);
    //   println!("You start with {} health points.", MAX_HEALTH);
    //   println!("Pi: {}", MYPI);

    //   let mut a = 10;
    //   a = a + 1;
    //   println!("a: {}",a);

    // let adult = true;
    // let age: &str = if adult { "+18" } else { "-18" };
    // println!("Age is {}", age);

    // for n in (1..11).step_by(2) {
    //   println!("The square of {} is {}", n, n * n);
    // }

    //   let mut x = 10;
    //   for _ in 1..x { x -= 1; print!("."); }

    //   fn abs(x: i32) -> u32 {
    //     if x > 0 {
    //           x as u32
    //     } else {
    //           -x as u32
    //     }
    //  }

    //  println!("{}",abs(-15));

    // let aliens = ["Cherfer", "Fynock", "Shirack", "Zuxu"];
    // println!("{:?}", aliens);

    // for a in aliens.iter() {
    //   println!("The next alien is {}", a);
    // }

    // let arr : [(i32, i32); 5] = [(0,2), (1,4), (2,5), (3,1), (4,3)];
    // let mut arr0 = [2, 1, 3];
    //   arr0.sort_by(|a, b| b.cmp(a));
    // let n : usize = arr0.len();
    // let arr1 = vec![0; n];
    // println!("{:?}", &arr0);

    // let arr0: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8];

    // let mut arr1 = Vec::from(&arr0[2..5]);
    // for (ix,a) in arr1.iter_mut().enumerate(){
    //   // *a *= 2;
    //   println!("{:?} - {}", a, ix);
    // }

    // println!("\n{:?}", arr1);

    // let p1 = Player { fname: "Sourav", health: 100, level: 10 };
    // let mut p2 = Player {fname: "Deviac", ..p1 };
    // p2.health = 50;
    // println!("{} has health {} and level {}", p1.fname, p1.health, p1.level);
    // println!("{} has health {} and level {}", p2.fname, p2.health, p2.level);

    // let direction = Compass::East;
    // println!("{:?}", direction);

    // let naruto_monster = PlanetaryMonster::MarsMonster("Naruto", 200);
    // let kakashi_monster = PlanetaryMonster::MarsMonster("Kakashi", 100);
    // let sasuke_monster = PlanetaryMonster::VenusMonster("Sasuke", 200);

    // println!("{:?}", naruto_monster);
    // println!("{:?}", kakashi_monster);
    // println!("{:?}", sasuke_monster);

    // let parsed_int1: i32 = "100".parse().unwrap();
    // println!("{}", parsed_int1);

    // let parsed_int2: i32 = match "s".parse() {
    //   Ok(num) => num,
    //   Err(_) => panic!("Unable to parse")
    // };
    // println!("{}", parsed_int2);

    // panic!();
    // panic!("this is a terrible mistake!");
    // panic!("this is a {} {message}", "fancy", message = "panic");
    // std::panic::panic_any(4); // panic with the value of 4 to be collected elsewhere

    //Taking input
    // println!("Enter a and b : ");
    // let mut astr = String::new();
    // let mut bstr = String::new();
    // let a:i32;
    // let b:i32;
    // io::stdin().read_line(&mut astr).ok().expect("Failed to read_line");
    // io::stdin().read_line(&mut bstr).ok().expect("Failed to read_line");

    // a = astr.trim().parse::<i32>().unwrap_or(-1);
    // b = bstr.trim().parse::<i32>().unwrap_or(-1);

    // println!("Sum of a and b is : {}",(a+b));

    // Taing input for array
    // println!("Enter number of elements in array : ");
    // let mut nstr = String::new();
    // io::stdin().read_line(&mut nstr).ok().expect("Failed to read_line");
    // let n:usize = nstr.trim().parse::<usize>().unwrap_or(0);

    // println!("Enter space separated elements of array : ");
    // let mut arrstr = String::with_capacity((n*2)-1);
    // io::stdin().read_line(&mut arrstr).ok().expect("Failed to read_line");
    // let arr: Vec<i32> = arrstr.trim().split(' ').flat_map(str::parse::<i32>).collect();
    // println!("{:?}", &arr);

    // let args: Vec<String> = env::args().collect();
    // println!("{:?}", args); // ["target/debug/hacker_rank", "ab", "10"]

    // let osvars = env::vars();
    // for (key, value) in osvars {
    //     println!("{}: {}", key, value);
    // }

    let v = vec![1, 2, 3];
    let arr: [i32; 3] = v.try_into().expect("Vector must have exactly 5 elements");
    println!("{:?}", arr); // Output: [1, 2, 3, 4, 5]
}

// use std::io;

// fn main(){
//     let mut tstr = String::new();
//     io::stdin().read_line(&mut tstr).expect("failed to readline");

//     let t = tstr.trim().parse::<i32>().unwrap_or(-1);

//     for _itr in 0..t{
//         let mut nstr = String::new();
//         io::stdin().read_line(&mut nstr).expect("failed to readline");
//         let n = nstr.trim().parse::<i32>().unwrap_or(-1);

//         let mut arrstr = String::new();
//         io::stdin().read_line(&mut arrstr).expect("failed to readline");
//         let q: Vec<i32> = arrstr.trim().split(' ').flat_map(str::parse::<i32>).collect();

//         minimum_bribes(q);
//     }
// }

// fn minimum_bribes(q: Vec<i32>){
//   let mut is_chaotic = false;
//   let n = q.len();
//   let qarr = vec![0; n];
//   let qobj = {};
//   let i = 0;
//   let answer = 0;

//   for (i, o) in q{

//   }
//       // cur = i

//       // if o - cur > 2:
//       //     print("Too chaotic")
//       //     return

//       // for k in q[max(o - 1, 0):i]:
//       //     if k > o:
//       //         bribes += 1
// }
