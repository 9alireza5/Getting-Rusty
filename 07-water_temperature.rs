fn main() {
    println!("please enter water's temperature in celsius:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);
    let mut temperature: i32 = input.trim().parse().expect("please eneter an integer");
    if temperature > 100 {
        println!("Steam");
    } else if temperature < 0 {
        println!("Ice");
    } else {
        println!("Water");
    }
}
