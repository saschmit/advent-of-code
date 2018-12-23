fn main() {
    let buff = String::from_utf8(std::fs::read("input").unwrap()).unwrap();
    let nums : Vec<i64> = buff.lines().map(|x| i64::from_str_radix(x, 10).unwrap()).collect();
    let mut sum : i64 = 0;
    sum = nums.iter().fold(sum, |acc, x| acc + x);
    println!("{}", sum);
}
