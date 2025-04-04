fn main() {
    let mut input = String::new();
    println!("please enter three numbers separated by spaces: ");
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut numbers = input.trim().split_whitespace();
    
    let a: f64 = numbers.next().expect("Missing first number").parse().expect("Invalid first number");
    let b: f64 = numbers.next().expect("Missing second number").parse().expect("Invalid second number");
    let c: f64 = numbers.next().expect("Missing third number").parse().expect("Invalid third number");

    println!("Sum: {}", sum_three_numbers(a, b, c));
}
fn sum_three_numbers(a: f64, b: f64, c: f64) -> f64 {
    a + b + c
}