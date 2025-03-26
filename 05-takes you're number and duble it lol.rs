fn main() {
    println!("please enter a number: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);
    let number: i32 = input 
        .trim()
        .parse()
        .expect("invalid input");
    println!("You entered: {} and its double is: {}", number, number * 2);
}
