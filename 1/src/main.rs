use std::collections::HashMap;

fn main() {
    let buff = String::from_utf8(std::fs::read("input").unwrap()).unwrap();
    let nums : Vec<i64> = buff.lines().map(|x| i64::from_str_radix(x, 10).unwrap()).collect();
    let mut sum : i64 = 0;
    let mut index : usize = 0;
    println!("sum: {}", nums.iter().fold(sum, |acc, x| acc + x));

    let mut seen = HashMap::new();
    let freq = loop {
        let freq = seen.entry(sum).or_insert(0);
        *freq += 1;
        if *freq > 1 {
            break sum;
        }
        sum += nums[index];
        index = (index + 1) % nums.len();
    };
    println!("freq: {}", freq);
}
